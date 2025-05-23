use crate::error::{NetworkError, Result};
use crate::raknet::protocol::MAGIC_BYTES;
use bytes::{Buf, BytesMut};

#[derive(Clone, Debug)]
pub struct OpenConnectionRequest1 {
    pub protocol_version: u8,
    pub estimated_mtu: u16,
}

impl OpenConnectionRequest1 {
    pub fn deserialize(buf: &mut BytesMut, payload_size: usize) -> Result<Self> {
        const FIXED_FIELDS_SIZE: usize = 16 + 1;
        if buf.remaining() < FIXED_FIELDS_SIZE {
            return Err(NetworkError::BufferTooShort {
                expected: FIXED_FIELDS_SIZE,
                actual: buf.remaining(),
            });
        }

        let mut magic_read = [0u8; 16];
        buf.copy_to_slice(&mut magic_read);
        if magic_read != MAGIC_BYTES {
            return Err(NetworkError::InvalidMagicBytes);
        }

        let protocol_version = buf.get_u8();

        let estimated_mtu = payload_size as u16;

        Ok(Self { protocol_version, estimated_mtu })
    }
}
