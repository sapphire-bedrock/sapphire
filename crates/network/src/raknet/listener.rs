use crate::error::Result;
use crate::raknet::protocol::{
    ID_UNCONNECTED_PING, ID_UNCONNECTED_PONG, UnconnectedPing, UnconnectedPong,
};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;

pub struct Listener {
    socket: Arc<UdpSocket>,
}

impl Listener {
    pub async fn bind(addr: SocketAddr) -> Result<Self> {
        let socket = UdpSocket::bind(addr).await?;

        Ok(Self { socket: Arc::new(socket) })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut buf = [0u8; 2048];

        loop {
            let (size, addr) = self.socket.recv_from(&mut buf[..]).await?;

            let data = Bytes::copy_from_slice(&buf[..size]);
            let socket = Arc::clone(&self.socket);

            self.handle_packet(&data, addr, socket).await?;
        }
    }

    async fn handle_packet(
        &self,
        data: &[u8],
        addr: SocketAddr,
        socket: Arc<UdpSocket>,
    ) -> Result<()> {
        if data.is_empty() {
            return Ok(());
        }

        let packet_id = data[0];

        tracing::info!("Handling packet: {:?}", packet_id);

        let response = match packet_id {
            ID_UNCONNECTED_PING => Some(self.handle_unconnected_ping(data).await?),
            _ => None,
        };

        if let Some(response_data) = response {
            socket.send_to(&response_data, addr).await?;
        }

        Ok(())
    }

    async fn handle_unconnected_ping(&self, data: &[u8]) -> Result<Bytes> {
        let mut cursor = BytesMut::from(data);
        let _id = cursor.get_u8();
        let ping = UnconnectedPing::deserialize(&mut cursor)?;

        let pong = UnconnectedPong {
            time: ping.time,
            guid: rand::random(),
            motd: "MCPE;Dedicated Server;390;1.14.60;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133;".to_string(),
        };

        let mut response_buf = BytesMut::new();
        response_buf.put_u8(ID_UNCONNECTED_PONG);
        pong.serialize(&mut response_buf)?;

        Ok(response_buf.freeze())
    }
}
