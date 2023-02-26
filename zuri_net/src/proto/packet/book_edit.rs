use zuri_net_derive::proto;

/// Sent by the client when it edits a book. It is sent each time a modification was made and the
/// player stops its typing 'session', rather than simply after closing the book.
#[proto]
#[derive(Debug, Clone)]
pub struct BookEdit {
    /// The type of the book edit action. The data obtained depends on what type this is.
    pub action_type: BookAction,
}

#[proto(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum BookAction {
    ReplacePage(ReplaceOrAddPage),
    AddPage(ReplaceOrAddPage),
    DeletePage(DeletePage),
    SwapPages(SwapPages),
    Sign(Sign),
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct ReplaceOrAddPage {
    /// The slot in which the book that was edited may be found. Typically, the server should check
    /// if this slot matches the held item slot of the player.
    pub inventory_slot: u8,
    /// The number of the page that the book edit action concerns. It applies for all actions but
    /// signing. When swapping pages, it is one of the pages that was swapped.
    pub page_number: u8,
    /// The text that was written in a particular page of the book.
    pub text: String,
    /// The name of the photo on the page in the book. Unfortunately, the functionality of this
    /// field was removed from the default Minecraft: Bedrock Edition. It is still available on
    /// Education Edition.
    pub photo_name: String,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct DeletePage {
    /// The slot in which the book that was edited may be found. Typically, the server should check
    /// if this slot matches the held item slot of the player.
    pub inventory_slot: u8,
    /// The number of the page that the book edit action concerns. It applies for all actions but
    /// signing. When swapping pages, it is one of the pages that was swapped.
    pub page_number: u8,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct SwapPages {
    /// The slot in which the book that was edited may be found. Typically, the server should check
    /// if this slot matches the held item slot of the player.
    pub inventory_slot: u8,
    /// The number of the page that the book edit action concerns. It applies for all actions but
    /// signing. When swapping pages, it is one of the pages that was swapped.
    pub page_number: u8,
    /// The page number of the second page that the action concerned.
    pub secondary_page_number: u8,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct Sign {
    /// The slot in which the book that was edited may be found. Typically, the server should check
    /// if this slot matches the held item slot of the player.
    pub inventory_slot: u8,
    /// The title that the player has given the book.
    pub title: String,
    /// The author that the player has given the book. Note that the author may be freely changed,
    /// so no assumptions can be made on if the author is actually the name of a player.
    pub author: String,
    /// The XBOX Live User ID of the player that edited the book. The field is rather pointless, as
    /// the server is already aware of the XUID of the player anyway.
    pub xuid: String,
}
