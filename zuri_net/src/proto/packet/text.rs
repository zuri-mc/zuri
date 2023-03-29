use crate::proto::ints::VarU32;
use zuri_net_derive::proto;

/// Sent by the client to the server to send chat messages, and by the server to the client to
/// forward or send messages, which may be chat, popups, tips etc.
#[proto]
#[derive(Debug, Clone)]
pub struct Text {
    /// The type of the text sent. When a client sends this to the server, it should always be Chat.
    pub text_type: TextType,
    /// The XBOX Live user ID of the player that sent the message. It is only set for packets of
    /// text type Chat. When sent to a player, the player will only be shown the chat message if a
    /// player with this XUID is present in the player list and not muted, or if the XUID is empty.
    pub xuid: String,
    /// An identifier only set for particular platforms when chatting (presumably only for Nintendo
    /// Switch). It is otherwise an empty string, and is used to decide which players are able to
    /// chat with each other.
    pub platform_chat_id: String,
}

#[proto(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum TextType {
    Raw(TextTypeSimple),
    Chat(TextTypeWithSource),
    Translation(TextTypeWithParams),
    Popup(TextTypeWithParams),
    JukeboxPopup(TextTypeWithParams),
    Tip(TextTypeSimple),
    System(TextTypeSimple),
    Whisper(TextTypeWithSource),
    Announcement(TextTypeWithSource),
    ObjectWhisper(TextTypeSimple),
    Object(TextTypeSimple),
    ObjectAnnouncement(TextTypeSimple),
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct TextTypeSimple {
    /// Specifies if any of the messages need to be translated. It seems that where % is found in
    /// translatable text types, these are translated regardless of this bool. Translatable text
    /// types include Translation, Tip, Popup, and JukeboxPopup.
    pub needs_translation: bool,
    /// The message of the packet. This field is set for each TextType and is the main component of
    /// the packet.
    pub message: String,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct TextTypeWithSource {
    /// Specifies if any of the messages need to be translated. It seems that where % is found in
    /// translatable text types, these are translated regardless of this bool. Translatable text
    /// types include Translation, Tip, Popup, and JukeboxPopup.
    pub needs_translation: bool,
    /// The name of the source of the messages. This source is displayed in text types such as Chat
    /// and Whisper, where typically the username is shown.
    pub source_name: String,
    /// The message of the packet. This field is set for each TextType and is the main component of
    /// the packet.
    pub message: String,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct TextTypeWithParams {
    /// Specifies if any of the messages need to be translated. It seems that where % is found in
    /// translatable text types, these are translated regardless of this bool. Translatable text
    /// types include Translation, Tip, Popup, and JukeboxPopup.
    pub needs_translation: bool,
    /// The message of the packet. This field is set for each TextType and is the main component of
    /// the packet.
    pub message: String,
    /// A list of parameters that should be filled into the message. These parameters are only
    /// written if the type of the packet is Translation, Tip, Popup or JukeboxPopup.
    #[len_type(VarU32)]
    pub parameters: Vec<String>,
}
