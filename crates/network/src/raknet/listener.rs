use crate::error::Result;
use crate::raknet::motd::Motd;
use crate::raknet::protocol::connection_request_accepted::get_raknet_timestamp;
use crate::raknet::protocol::{
    ConnectionRequest, ConnectionRequestAccepted, DEFAULT_SERVER_MAX_MTU, ID_CONNECTION_REQUEST,
    ID_CONNECTION_REQUEST_ACCEPTED, ID_INCOMPATIBLE_PROTOCOL_VERSION, ID_OPEN_CONNECTION_REPLY_1,
    ID_OPEN_CONNECTION_REPLY_2, ID_OPEN_CONNECTION_REQUEST_1, ID_OPEN_CONNECTION_REQUEST_2,
    ID_UNCONNECTED_PING, ID_UNCONNECTED_PONG, OpenConnectionReply1, OpenConnectionReply2,
    OpenConnectionRequest1, OpenConnectionRequest2, RAKNET_PROTOCOL_VERSION, UnconnectedPing,
    UnconnectedPong,
};
use bytes::{BufMut, Bytes, BytesMut};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;

pub struct Listener {
    socket: Arc<UdpSocket>,
    server_guid: u64,
}

impl Listener {
    pub async fn bind(addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(addr).await?;
        let random_unique_id: u64 = rand::random();
        tracing::info!("Server GUID: {}", random_unique_id);
        Ok(Self {
            socket: Arc::new(socket),
            server_guid: random_unique_id,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut buf = [0u8; 2048];

        loop {
            let (size, addr) = self.socket.recv_from(&mut buf[..]).await?;
            let data_slice = &buf[..size];

            if data_slice.is_empty() {
                tracing::warn!("Received an empty packet from {}", addr);
                continue;
            }

            if let Err(e) = self.handle_packet(data_slice, addr).await {
                tracing::error!("Error handling packet from {}: {:?}", addr, e);
            }
        }
    }

    async fn handle_packet(&self, data_slice: &[u8], addr: SocketAddr) -> Result<()> {
        let packet_id = data_slice[0];

        tracing::debug!("Received packet 0x{:02X} from {}", packet_id, addr);

        let response = match packet_id {
            ID_UNCONNECTED_PING => Some(self.handle_unconnected_ping(&data_slice[1..]).await?),
            ID_OPEN_CONNECTION_REQUEST_1 => Some(
                self.handle_open_connection_request1(&data_slice[1..], addr)
                    .await?,
            ),
            ID_OPEN_CONNECTION_REQUEST_2 => Some(
                self.handle_open_connection_request2(&data_slice[1..], addr)
                    .await?,
            ),
            ID_CONNECTION_REQUEST => Some(
                self.handle_connection_request(&data_slice[1..], addr)
                    .await?,
            ),
            _ => {
                tracing::warn!(
                    "Received unknown packet ID 0x{:02X} from {}",
                    packet_id,
                    addr
                );
                None
            }
        };

        if let Some(response_data) = response {
            self.socket.send_to(&response_data, addr).await?;
        }

        Ok(())
    }

    async fn handle_unconnected_ping(&self, incoming_data_slice: &[u8]) -> Result<Bytes> {
        let mut bytes = BytesMut::from(incoming_data_slice);
        let ping = UnconnectedPing::deserialize(&mut bytes)?;

        let motd = Motd {
            server_name: "Sapphire".to_string(),
            protocol_version: 800,
            minecraft_version: "1.21.82".to_string(),
            player_count: 0,
            max_players: 50,
            server_guid: self.server_guid,
            world_name: "Bedrock World".to_string(),
            game_mode: "Survival".to_string(),
            game_mode_id: 1,
            port_v4: self.socket.local_addr().ok().map_or(19132, |sa| sa.port()),
            port_v6: self.socket.local_addr().ok().map_or(19133, |sa| sa.port()),
        };

        let pong = UnconnectedPong {
            time: ping.time,
            guid: self.server_guid,
            motd,
        };

        let mut response_buf = BytesMut::new();
        response_buf.put_u8(ID_UNCONNECTED_PONG);
        pong.serialize(&mut response_buf)?;

        Ok(response_buf.freeze())
    }

    async fn handle_open_connection_request1(
        &self,
        incoming_data_slice: &[u8],
        addr: SocketAddr,
    ) -> Result<Bytes> {
        let slice_length = incoming_data_slice.len();
        let mut bytes = BytesMut::from(incoming_data_slice);

        let request = OpenConnectionRequest1::deserialize(&mut bytes, slice_length)?;

        tracing::info!(
            "OCR1: client protocol {}, client MTU {}",
            request.protocol_version,
            request.estimated_mtu
        );

        if request.protocol_version != RAKNET_PROTOCOL_VERSION {
            tracing::warn!(
                "Client {} has incompatible RakNet protocol version: {} (server: {})",
                addr.to_string(),
                request.protocol_version,
                RAKNET_PROTOCOL_VERSION,
            );

            let mut response_buf = BytesMut::new();
            response_buf.put_u8(ID_INCOMPATIBLE_PROTOCOL_VERSION);
            response_buf.put_u8(RAKNET_PROTOCOL_VERSION);
            response_buf.put(&crate::raknet::protocol::MAGIC_BYTES[..]);
            response_buf.put_u64(self.server_guid);
            return Ok(response_buf.freeze());
        }

        let server_chosen_mtu = request.estimated_mtu.min(DEFAULT_SERVER_MAX_MTU);

        let reply = OpenConnectionReply1 {
            server_guid: self.server_guid,
            use_security: false,
            mtu_size: server_chosen_mtu,
        };

        let mut response_buf = BytesMut::new();
        response_buf.put_u8(ID_OPEN_CONNECTION_REPLY_1);
        reply.serialize(&mut response_buf)?;

        Ok(response_buf.freeze())
    }

    async fn handle_open_connection_request2(
        &self,
        incoming_data_slice: &[u8],
        addr: SocketAddr,
    ) -> Result<Bytes> {
        let mut bytes = BytesMut::from(incoming_data_slice);

        let request = OpenConnectionRequest2::deserialize(&mut bytes)?;
        tracing::info!(
            "OCR2: client GUID {}, client MTU {}",
            request.client_guid,
            request.mtu_size
        );

        let final_mtu = request.mtu_size.min(DEFAULT_SERVER_MAX_MTU);

        let reply = OpenConnectionReply2 {
            server_guid: self.server_guid,
            client_address: addr,
            mtu_size: final_mtu,
            use_encryption: false,
        };

        let mut response_buf = BytesMut::new();
        response_buf.put_u8(ID_OPEN_CONNECTION_REPLY_2);
        reply.serialize(&mut response_buf)?;
        Ok(response_buf.freeze())
    }

    async fn handle_connection_request(
        &self,
        incoming_data_slice: &[u8],
        addr: SocketAddr,
    ) -> Result<Bytes> {
        let mut bytes = BytesMut::from(incoming_data_slice);

        let request = ConnectionRequest::deserialize(&mut bytes)?;
        tracing::info!(
            "CR: client GUID {}, timestamp {}, use_security {}",
            request.client_guid,
            request.request_timestamp,
            request.use_security
        );

        let reply = ConnectionRequestAccepted {
            client_address: addr,
            system_index: 0,
            request_timestamp: request.request_timestamp,
            server_timestamp: get_raknet_timestamp(),
        };

        let mut response_buf = BytesMut::new();
        response_buf.put_u8(ID_CONNECTION_REQUEST_ACCEPTED);
        reply.serialize(&mut response_buf)?;
        Ok(response_buf.freeze())
    }
}
