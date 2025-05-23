use crate::error::{NetworkError, Result};
use bytes::{Buf, BytesMut};

#[derive(Debug, Clone)]
pub struct ConnectionRequest {
    pub client_guid: u64,
    pub request_timestamp: u64,
    pub use_security: bool,
}

impl ConnectionRequest {
    pub fn deserialize(buf: &mut BytesMut) -> Result<Self> {
        const EXPECTED_SIZE: usize = 8 + 8 + 1; // Client GUID + Timestamp + Use Security
        if buf.remaining() < EXPECTED_SIZE {
            return Err(NetworkError::BufferTooShort {
                expected: EXPECTED_SIZE,
                actual: buf.remaining(),
            });
        }

        let client_guid = buf.get_u64();
        let request_timestamp = buf.get_u64();
        let use_security_byte = buf.get_u8();
        let use_security = use_security_byte != 0;

        Ok(Self {
            client_guid,
            request_timestamp,
            use_security,
        })
    }
}
