use crate::error::{NetworkError, Result};
use crate::raknet::protocol::MAGIC_BYTES;
use bytes::{Buf, BytesMut};
#[derive(Debug, Clone)]
pub struct OpenConnectionRequest2 {
    pub mtu_size: u16,
    pub client_guid: u64,
}

impl OpenConnectionRequest2 {
    pub fn deserialize(buf: &mut BytesMut) -> Result<Self> {
        if buf.remaining() < 16 {
            // Magic
            return Err(NetworkError::BufferTooShort { expected: 16, actual: buf.remaining() });
        }

        let mut magic_read = [0u8; 16];
        buf.copy_to_slice(&mut magic_read);
        if magic_read != MAGIC_BYTES {
            return Err(NetworkError::InvalidMagicBytes);
        }

        if !buf.has_remaining() {
            return Err(NetworkError::Custom(
                "Buffer too short for server address IP version".into(),
            ));
        }

        if buf.remaining() < 1 {
            return Err(NetworkError::Custom(
                "Buffer too short for server address IP version".into(),
            ));
        }

        let ip_ver = buf.get_u8();

        let skip_len = match ip_ver {
            4 => 4 + 2,  // IPv4 (4 bytes) + port (2 bytes)
            6 => 16 + 2, // IPv6 (16 bytes) + port (2 bytes)
            _ => {
                return Err(NetworkError::Custom(format!(
                    "Unknown IP version {} in OCR2 ServerAddress",
                    ip_ver
                )));
            }
        };

        if buf.remaining() < skip_len {
            return Err(NetworkError::Custom(format!(
                "Buffer too short for IPv{} server address details",
                ip_ver
            )));
        }
        buf.advance(skip_len);

        const MTU_AND_GUID_SIZE: usize = 2 + 8;
        if buf.remaining() < MTU_AND_GUID_SIZE {
            return Err(NetworkError::BufferTooShort {
                expected: MTU_AND_GUID_SIZE,
                actual: buf.remaining(),
            });
        }
        let mtu_size = buf.get_u16();
        let client_guid = buf.get_u64();

        Ok(Self { mtu_size, client_guid })
    }
}
