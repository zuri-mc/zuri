use crate::proto::ints::VarI32;
use zuri_net_derive::proto;

/// Sent by the server to make a title, subtitle or action bar shown to a player. It has several
/// fields that allow setting the duration of the titles.
#[proto]
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
    pub fade_in_duration: VarI32,
    /// The duration that the title remains on the screen of the player. It is measured in 20ths of
    /// a second (AKA in ticks).
    pub remain_duration: VarI32,
    /// The duration that the title takes to fade out of the screen of the player. It is measured in
    /// 20ths of a second (AKA in ticks).
    pub fade_out_duration: VarI32,
    /// The XBOX Live user ID of the player, which will remain consistent as long as the player is
    /// logged in with the XBOX Live account. It is empty if the user is not logged into its XBL
    /// account.
    pub xuid: String,
    /// Either an unsigned long or an empty string.
    pub platform_online_id: String,
}

#[proto(VarI32)]
#[derive(Debug, Clone)]
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
