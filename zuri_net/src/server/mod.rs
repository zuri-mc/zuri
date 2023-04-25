use crate::proto::{CURRENT_PROTOCOL, CURRENT_VERSION};

pub mod listener;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Motd {
    /// The game edition, either education edition or bedrock edition.
    pub edition: Edition,
    /// The first line of the MOTD, for publicly accessible servers usually the main MOTD. In
    /// vanilla, it corresponds to the world name.
    pub motd: String,
    /// The second line of the MOTD, only visible in the LAN Games list.
    pub local_motd: String,
    /// The amount of players currently connected to the server. For some reason, negative numbers
    /// do also show up in the client.
    pub player_count: i32,
    /// The maximum amount of players that are able to connect to the server. If this is lower than
    /// or equal to the player_count, the client will not allow the user to connect and will instead
    /// show its own 'server full' message.
    pub max_player_count: i32,
}

/// The game edition, used in [Motd].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Edition {
    /// Indicates that the server is for Minecraft: Bedrock Edition.
    #[default]
    Bedrock,
    /// Indicates that the server is for Minecraft: Education Edition. Bedrock clients will not be
    /// able to connect to the server.
    Education,
}

impl Motd {
    fn serialize(&self, server_uniq_id: u64, port: u16) -> String {
        format!(
            "{};{};{};{};{};{};{};{};{};{};{};{}",
            match self.edition {
                Edition::Bedrock => "MCPE",
                Edition::Education => "MCEE",
            },
            self.motd,
            CURRENT_PROTOCOL,
            CURRENT_VERSION,
            self.player_count,
            self.max_player_count,
            server_uniq_id,
            self.local_motd,
            "Creative",
            1,
            port,
            port,
        )
    }
}

// Edition (MCPE or MCEE for Education Edition);MOTD line 1;Protocol Version;Version Name;Player Count;Max Player Count;Server Unique ID;MOTD line 2;Game mode;Game mode (numeric);Port (IPv4);Port (IPv6);
