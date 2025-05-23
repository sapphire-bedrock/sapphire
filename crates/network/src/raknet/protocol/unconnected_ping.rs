use crate::error::{NetworkError, Result};
use crate::raknet::protocol::MAGIC_BYTES;
use bytes::{Buf, BytesMut};

#[derive(Clone, Debug)]
pub struct UnconnectedPing {
    pub time: u64,
    pub guid: u64,
}

impl UnconnectedPing {
    pub fn deserialize(buf: &mut BytesMut) -> Result<Self> {
        const EXPECTED_PAYLOAD_SIZE: usize = 8 /* time */ + 16 /* magic */ + 8 /* guid */;
        if buf.remaining() < EXPECTED_PAYLOAD_SIZE {
            return Err(NetworkError::BufferTooShort {
                expected: EXPECTED_PAYLOAD_SIZE,
                actual: buf.remaining(),
            });
        }

        let time = buf.get_u64();
        let mut magic_read = [0u8; 16];
        buf.copy_to_slice(&mut magic_read);

        if magic_read != MAGIC_BYTES {
            return Err(NetworkError::InvalidMagicBytes);
        }

        let guid = buf.get_u64();
        Ok(Self { time, guid })
    }
}
