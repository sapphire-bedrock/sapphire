use crate::error::{NetworkError, Result};
use crate::raknet::motd::Motd;
use crate::raknet::protocol::{
    ID_UNCONNECTED_PING, ID_UNCONNECTED_PONG, UnconnectedPing, UnconnectedPong,
};
use bytes::{BufMut, Bytes, BytesMut};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;

pub struct Listener {
    socket: Arc<UdpSocket>,
    guid: u64,
}

impl Listener {
    pub async fn bind(addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(addr).await?;
        let guid: u64 = rand::random();

        Ok(Self { socket: Arc::new(socket), guid })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut buf = [0u8; 2048];

        loop {
            let (size, addr) = self.socket.recv_from(&mut buf[..]).await?;

            let data_slice = &buf[..size];

            self.handle_packet(data_slice, addr).await?;
        }
    }

    async fn handle_packet(&self, data_slice: &[u8], addr: SocketAddr) -> Result<()> {
        if data_slice.is_empty() {
            return Err(NetworkError::BufferTooShort { expected: 1, actual: 0 });
        }

        let packet_id = data_slice[0];

        tracing::info!("Received packet: 0x{:02x}", packet_id);

        let response = match packet_id {
            ID_UNCONNECTED_PING => Some(self.handle_unconnected_ping(&data_slice[1..]).await?),
            _ => None,
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
            server_name: "Dedicated Server".to_string(),
            protocol_version: 800,
            minecraft_version: "1.21.82".to_string(),
            player_count: 0,
            max_players: 10,
            server_guid: self.guid,
            world_name: "Bedrock level".to_string(),
            game_mode: "Survival".to_string(),
            game_mode_id: 1,
            port_v4: 19132,
            port_v6: 19133,
        };

        let pong = UnconnectedPong { time: ping.time, guid: ping.guid, motd };

        let mut response_buf = BytesMut::new();
        response_buf.put_u8(ID_UNCONNECTED_PONG);
        pong.serialize(&mut response_buf)?;

        Ok(response_buf.freeze())
    }
}
