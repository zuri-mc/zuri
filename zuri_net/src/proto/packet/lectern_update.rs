use zuri_net_derive::proto;

use crate::proto::io::UBlockPos;

/// Sent by the client to update the server on which page was opened in a book on a lectern, or if
/// the book should be removed from it.
#[proto]
#[derive(Debug, Clone)]
pub struct LecternUpdate {
    /// The page number in the book that was opened by the player on the lectern.
    pub page: u8,
    /// The number of pages that the book opened in the lectern has.
    pub page_count: u8,
    /// The position of the lectern that was updated. If no lectern is at the block position, the
    /// packet should be ignored.
    pub position: UBlockPos,
    /// Specifies if the book currently set on display in the lectern should be dropped server-side.
    pub drop_book: bool,
}
