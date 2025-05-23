use crate::error::{NetworkError, Result};
use crate::raknet::motd::Motd;
use crate::raknet::protocol::MAGIC_BYTES;
use bytes::{BufMut, BytesMut};

#[derive(Clone, Debug)]
pub struct UnconnectedPong {
    pub time: u64,
    pub guid: u64,
    pub motd: Motd,
}

impl UnconnectedPong {
    pub fn serialize(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u64(self.time);
        buf.put_u64(self.guid);

        buf.put(&MAGIC_BYTES[..]);

        let motd_bytes = self.motd.to_string();
        let motd_bytes = motd_bytes.as_bytes();
        if motd_bytes.len() > u16::MAX as usize {
            return Err(NetworkError::MotdTooLong(
                motd_bytes.len(),
                u16::MAX as usize,
            ));
        }

        buf.put_u16(motd_bytes.len() as u16);
        buf.put(motd_bytes);

        Ok(())
    }
}
