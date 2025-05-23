pub mod unconnected_ping;
pub mod unconnected_pong;

pub use unconnected_ping::UnconnectedPing;
pub use unconnected_pong::UnconnectedPong;

pub const ID_CONNECTED_PING: u8 = 0x00;
pub const ID_UNCONNECTED_PING: u8 = 0x01;
pub const ID_UNCONNECTED_PING_OPEN_CONNECTIONS: u8 = 0x02;
pub const ID_CONNECTED_PONG: u8 = 0x03;
pub const ID_DETECT_LOST_CONNECTIONS: u8 = 0x04;
pub const ID_OPEN_CONNECTION_REQUEST_1: u8 = 0x05;
pub const ID_OPEN_CONNECTION_REQUEST_2: u8 = 0x06;
pub const ID_OPEN_CONNECTION_REPLY_1: u8 = 0x07;
pub const ID_OPEN_CONNECTION_REPLY_2: u8 = 0x08;
pub const ID_CONNECTION_REQUEST: u8 = 0x09;
pub const ID_CONNECTION_REQUEST_ACCEPTED: u8 = 0x10;
pub const ID_NEW_INCOMING_CONNECTION: u8 = 0x13;
pub const ID_DISCONNECTION_NOTIFICATION: u8 = 0x15;
pub const ID_INCOMPATIBLE_PROTOCOL_VERSION: u8 = 0x19;
pub const ID_UNCONNECTED_PONG: u8 = 0x1c;

pub const MAGIC_BYTES: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78,
];

pub const RAKNET_PROTOCOL_VERSION: u8 = 11;
