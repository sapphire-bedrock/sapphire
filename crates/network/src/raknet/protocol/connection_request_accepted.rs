use crate::error::Result;
use bytes::{BufMut, BytesMut};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct ConnectionRequestAccepted {
    pub client_address: SocketAddr,
    pub system_index: u16,
    pub request_timestamp: u64,
    pub server_timestamp: u64,
}

impl ConnectionRequestAccepted {
    pub fn serialize(&self, buf: &mut BytesMut) -> Result<()> {
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

        buf.put_u16(self.system_index);

        let placeholder_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0));
        for _ in 0..10 {
            match placeholder_addr {
                SocketAddr::V4(addr_v4) => {
                    buf.put_u8(4);
                    buf.put(&addr_v4.ip().octets()[..]);
                    buf.put_u16(addr_v4.port());
                }
                SocketAddr::V6(addr_v6) => {
                    buf.put_u8(6);
                    buf.put(&addr_v6.ip().octets()[..]);
                    buf.put_u16(addr_v6.port());
                }
            }
        }

        buf.put_u64(self.request_timestamp);
        buf.put_u64(self.server_timestamp);

        Ok(())
    }
}

pub fn get_raknet_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
