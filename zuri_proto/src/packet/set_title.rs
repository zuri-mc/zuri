use num_derive::{FromPrimitive, ToPrimitive};

use crate::packet::PacketType;
use crate::io::{Reader, Writer};

#[derive(Debug, FromPrimitive, ToPrimitive)]
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

#[derive(Debug)]
pub struct SetTitle {
    pub action_type: i32,
    pub text: String,
    pub fade_in_duration: i32,
    pub remain_duration: i32,
    pub fade_out_duration: i32,
    pub xuid: String,
    pub platform_online_id: String,
}

impl PacketType for SetTitle {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.action_type);
        writer.string(self.text.as_str());
        writer.var_i32(self.fade_in_duration);
        writer.var_i32(self.remain_duration);
        writer.var_i32(self.fade_out_duration);
        writer.string(self.xuid.as_str());
        writer.string(self.platform_online_id.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: reader.var_i32(),
            text: reader.string(),
            fade_in_duration: reader.var_i32(),
            remain_duration: reader.var_i32(),
            fade_out_duration: reader.var_i32(),
            xuid: reader.string(),
            platform_online_id: reader.string(),
        }
    }
}
