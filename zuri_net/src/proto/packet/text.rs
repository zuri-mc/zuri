use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
pub enum TextType {
    Raw,
    Chat,
    Translation,
    Popup,
    JukeboxPopup,
    Tip,
    System,
    Whisper,
    Announcement,
    ObjectWhisper,
    Object,
    ObjectAnnouncement,
}

/// Sent by the client to the server to send chat messages, and by the server to the client to
/// forward or send messages, which may be chat, popups, tips etc.
#[derive(Debug, Clone)]
pub struct Text {
    /// The type of the text sent. When a client sends this to the server, it should always be Chat.
    pub text_type: TextType,
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
    /// A list of parameters that should be filled into the message. These parameters are only
    /// written if the type of the packet is Translation, Tip, Popup or JukeboxPopup.
    pub parameters: Vec<String>,
    /// The XBOX Live user ID of the player that sent the message. It is only set for packets of
    /// text type Chat. When sent to a player, the player will only be shown the chat message if a
    /// player with this XUID is present in the player list and not muted, or if the XUID is empty.
    pub xuid: String,
    /// An identifier only set for particular platforms when chatting (presumably only for Nintendo
    /// Switch). It is otherwise an empty string, and is used to decide which players are able to
    /// chat with each other.
    pub platform_chat_id: String,
}

impl PacketType for Text {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.text_type.to_u8().unwrap());
        writer.bool(self.needs_translation);
        match self.text_type {
            TextType::Chat | TextType::Whisper | TextType::Announcement => {
                writer.string(self.source_name.as_str());
                writer.string(self.message.as_str());
            }
            TextType::Raw | TextType::Tip | TextType::System | TextType::Object | TextType::ObjectWhisper | TextType::ObjectAnnouncement => {
                writer.string(self.message.as_str());
            }
            TextType::Translation | TextType::Popup | TextType::JukeboxPopup => {
                writer.string(self.message.as_str());
                writer.var_u32(self.parameters.len() as u32);
                self.parameters.iter().for_each(|parameter| writer.string(parameter.as_str()));
            }
        }
        writer.string(self.xuid.as_str());
        writer.string(self.platform_chat_id.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        let text_type = TextType::from_u8(reader.u8()).unwrap();
        Self {
            text_type,
            needs_translation: reader.bool(),
            source_name: if text_type == TextType::Chat || text_type == TextType::Whisper || text_type == TextType::Announcement {
                reader.string()
            } else {
                String::new()
            },
            message: reader.string(),
            parameters: if text_type == TextType::Translation || text_type == TextType::Popup || text_type == TextType::JukeboxPopup {
                (0..reader.var_u32()).map(|_| reader.string()).collect()
            } else {
                Vec::new()
            },
            xuid: reader.string(),
            platform_chat_id: reader.string(),
        }
    }
}
