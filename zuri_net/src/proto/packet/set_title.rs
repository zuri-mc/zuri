use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum TitleAction {
    Clear,
    Reset,
    SetTitle,
    SetSubtitle,
    SetActionBar,
    SetDurations,
    TitleTextObject,
    SubtitleTextObject,
    ActionbarTextObject,
}

/// Sent by the server to make a title, subtitle or action bar shown to a player. It has several
/// fields that allow setting the duration of the titles.
#[derive(Debug, Clone)]
pub struct SetTitle {
    /// The type of the action that should be executed upon the title of a player. It specifies the
    /// response of the client to the packet.
    pub action_type: TitleAction,
    /// The text of the title, which has a different meaning depending on the `action_type` that the
    /// packet has. The text is of a title, subtitle or action bar, depending on the type set.
    pub text: String,
    /// The duration that the title takes to fade in on the screen of the player. It is measured in
    /// 20ths of a second (AKA in ticks).
    pub fade_in_duration: i32,
    /// The duration that the title remains on the screen of the player. It is measured in 20ths of
    /// a second (AKA in ticks).
    pub remain_duration: i32,
    /// The duration that the title takes to fade out of the screen of the player. It is measured in
    /// 20ths of a second (AKA in ticks).
    pub fade_out_duration: i32,
    /// The XBOX Live user ID of the player, which will remain consistent as long as the player is
    /// logged in with the XBOX Live account. It is empty if the user is not logged into its XBL
    /// account.
    pub xuid: String,
    /// Either an unsigned long or an empty string.
    pub platform_online_id: String,
}

impl PacketType for SetTitle {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.action_type.to_i32().unwrap());
        writer.string(self.text.as_str());
        writer.var_i32(self.fade_in_duration);
        writer.var_i32(self.remain_duration);
        writer.var_i32(self.fade_out_duration);
        writer.string(self.xuid.as_str());
        writer.string(self.platform_online_id.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: TitleAction::from_i32(reader.var_i32()).unwrap(),
            text: reader.string(),
            fade_in_duration: reader.var_i32(),
            remain_duration: reader.var_i32(),
            fade_out_duration: reader.var_i32(),
            xuid: reader.string(),
            platform_online_id: reader.string(),
        }
    }
}
