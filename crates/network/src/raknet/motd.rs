#[derive(Clone, Debug)]
pub struct Motd {
    pub server_name: String,
    pub protocol_version: u32,
    pub minecraft_version: String,
    pub player_count: u32,
    pub max_players: u32,
    pub server_guid: u64,
    pub world_name: String,
    pub game_mode: String,
    pub game_mode_id: u32,
    pub port_v4: u16,
    pub port_v6: u16,
}

impl std::fmt::Display for Motd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MCPE;{};{};{};{};{};{};{};{};{};{};{};",
            self.server_name,
            self.protocol_version,
            self.minecraft_version,
            self.player_count,
            self.max_players,
            self.server_guid,
            self.world_name,
            self.game_mode,
            self.game_mode_id,
            self.port_v4,
            self.port_v6,
        )
    }
}
