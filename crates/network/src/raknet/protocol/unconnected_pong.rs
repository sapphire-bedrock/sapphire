use crate::raknet::protocol::MAGIC_BYTES;
use crate::error::{NetworkError, Result};
use bytes::{BufMut, BytesMut};

#[derive(Clone)]
pub struct UnconnectedPong {
    pub time: u64,
    pub guid: u64,
    pub motd: String,
}

impl UnconnectedPong {
    pub fn serialize(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u64(self.time);
        buf.put_u64(self.guid);
        buf.put(&MAGIC_BYTES[..]);

        let motd_bytes = self.motd.as_bytes();
        if motd_bytes.len() > u16::MAX as usize {
            return Err(NetworkError::Custom("MOTD string too long".to_string()));
        }

        buf.put_u16(motd_bytes.len() as u16);
        buf.put(motd_bytes);
        Ok(())
    }
}
