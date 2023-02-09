use zuri_net_derive::proto;

/// Sent from the server to display a toast to the top of the screen. These toasts are the same as
/// the ones seen when, for example, loading a new resource pack or obtaining an achievement.\
#[proto]
#[derive(Debug, Clone)]
pub struct ToastRequest {
    /// The title of the toast.
    pub title: String,
    /// The message that the toast may contain alongside the title.
    pub message: String,
}
