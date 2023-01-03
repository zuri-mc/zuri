use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent from the server to display a toast to the top of the screen. These toasts are the same as
/// the ones seen when, for example, loading a new resource pack or obtaining an achievement.
#[derive(Debug, Clone)]
pub struct ToastRequest {
    /// The title of the toast.
    pub title: String,
    /// The message that the toast may contain alongside the title.
    pub message: String,
}

impl PacketType for ToastRequest {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.title.as_str());
        writer.string(self.message.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            title: reader.string(),
            message: reader.string(),
        }
    }
}
