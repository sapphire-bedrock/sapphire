use crate::error::{NetworkError, Result};
use crate::raknet::protocol::MAGIC_BYTES;
use bytes::{Buf, BufMut, BytesMut};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[derive(Debug, Clone)]
pub struct OpenConnectionReply2 {
    pub server_guid: u64,
    pub client_address: SocketAddr,
    pub mtu_size: u16,
    pub use_encryption: bool,
}

impl OpenConnectionReply2 {
    pub fn serialize(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put(&MAGIC_BYTES[..]);
        buf.put_u64(self.server_guid);

        match self.client_address {
            SocketAddr::V4(addr_v4) => {
                buf.put_u8(4); // IP Version 4
                buf.put(&addr_v4.ip().octets()[..]);
                buf.put_u16(addr_v4.port());
            }
            SocketAddr::V6(addr_v6) => {
                buf.put_u8(6); // IP Version 6
                buf.put(&addr_v6.ip().octets()[..]);
                buf.put_u16(addr_v6.port());
            }
        }

        buf.put_u16(self.mtu_size);
        buf.put_u8(if self.use_encryption { 1 } else { 0 });
        Ok(())
    }
}

#[allow(dead_code)]
fn deserialize_raknet_socket_address<B: Buf>(buf: &mut B) -> Result<SocketAddr> {
    if buf.remaining() < 1 {
        return Err(NetworkError::BufferTooShort { expected: 1, actual: buf.remaining() });
    }
    let ip_version = buf.get_u8();
    match ip_version {
        4 => {
            if buf.remaining() < 4 + 2 {
                return Err(NetworkError::BufferTooShort { expected: 6, actual: buf.remaining() });
            }
            let mut ip_bytes = [0u8; 4];
            buf.copy_to_slice(&mut ip_bytes);
            let port = buf.get_u16();
            Ok(SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::from(ip_bytes),
                port,
            )))
        }
        6 => {
            if buf.remaining() < 16 + 2 {
                return Err(NetworkError::BufferTooShort { expected: 18, actual: buf.remaining() });
            }
            let mut ip_bytes = [0u8; 16];
            buf.copy_to_slice(&mut ip_bytes);
            let port = buf.get_u16();
            Ok(SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::from(ip_bytes),
                port,
                0,
                0,
            )))
        }
        _ => Err(NetworkError::Custom(format!(
            "Invalid IP version {} in RakNet SocketAddress",
            ip_version
        ))),
    }
}
