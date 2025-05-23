use crate::error::{NetworkError, Result};
use bytes::{Buf, BytesMut};

#[derive(Clone)]
pub struct UnconnectedPing {
    pub time: u64,
    pub guid: u64,
}

impl UnconnectedPing {
    pub fn deserialize(buf: &mut BytesMut) -> Result<Self> {
        if buf.remaining() < 32 {
            return Err(NetworkError::Custom("Buffer too short for UnconnectedPing".to_string()));
        }

        let time = buf.get_u64();
        let mut magic = [0u8; 16];
        buf.copy_to_slice(&mut magic);
        let guid = buf.get_u64();

        Ok(Self { time, guid })
    }
}
