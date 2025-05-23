use crate::error::Result;
use crate::raknet::protocol::MAGIC_BYTES;
use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone)]
pub struct OpenConnectionReply1 {
    pub server_guid: u64,
    pub use_security: bool,
    pub mtu_size: u16,
}

impl OpenConnectionReply1 {
    pub fn serialize(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put(&MAGIC_BYTES[..]);
        buf.put_u64(self.server_guid);
        buf.put_u8(if self.use_security { 1 } else { 0 });
        buf.put_u16(self.mtu_size);
        Ok(())
    }
}
