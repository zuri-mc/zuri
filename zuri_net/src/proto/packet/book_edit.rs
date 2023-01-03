use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum BookAction {
    ReplacePage,
    AddPage,
    DeletePage,
    SwapPages,
    Sign,
}

/// Sent by the client when it edits a book. It is sent each time a modification was made and the
/// player stops its typing 'session', rather than simply after closing the book.
#[derive(Debug, Clone)]
pub struct BookEdit {
    /// The type of the book edit action. The data obtained depends on what type this is.
    pub action_type: BookAction,
    /// The slot in which the book that was edited may be found. Typically, the server should check
    /// if this slot matches the held item slot of the player.
    pub inventory_slot: u8,
    /// The number of the page that the book edit action concerns. It applies for all actions but
    /// signing. When swapping pages, it is one of the pages that was swapped.
    pub page_number: u8,
    /// The page number of the second page that the action concerned. It is only set when swapping
    /// pages, in which case it is the other page that is swapped.
    pub secondary_page_number: u8,
    /// The text that was written in a particular page of the book. It applies for the add and
    /// replace page actions only.
    pub text: String,
    /// The name of the photo on the page in the book. It applies for the add and replace page
    /// actions only. Unfortunately, the functionality of this field was removed from the default
    /// Minecraft: Bedrock Edition. It is still available on Education Edition.
    pub photo_name: String,
    /// The title that the player has given the book. It applies only for the signing action.
    pub title: String,
    /// The author that the player has given the book. It applies only for the signing action. Note
    /// that the author may be freely changed, so no assumptions can be made on if the author is
    /// actually the name of a player.
    pub author: String,
    /// The XBOX Live User ID of the player that edited the book. The field is rather pointless, as
    /// the server is already aware of the XUID of the player anyway.
    pub xuid: String,
}

impl PacketType for BookEdit {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.action_type.to_u8().unwrap());
        writer.u8(self.inventory_slot);
        match self.action_type {
            BookAction::ReplacePage | BookAction::AddPage => {
                writer.u8(self.page_number);
                writer.string(self.text.as_str());
                writer.string(self.photo_name.as_str());
            }
            BookAction::DeletePage => {
                writer.u8(self.page_number);
            }
            BookAction::SwapPages => {
                writer.u8(self.page_number);
                writer.u8(self.secondary_page_number);
            }
            BookAction::Sign => {
                writer.string(self.title.as_str());
                writer.string(self.author.as_str());
                writer.string(self.xuid.as_str());
            }
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = BookAction::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            inventory_slot: reader.u8(),
            page_number: if action_type != BookAction::Sign { reader.u8() } else { 0 },
            secondary_page_number: if action_type == BookAction::SwapPages { reader.u8() } else { 0 },
            text: if action_type == BookAction::ReplacePage || action_type == BookAction::AddPage { reader.string() } else { String::new() },
            photo_name: if action_type == BookAction::ReplacePage || action_type == BookAction::AddPage { reader.string() } else { String::new() },
            title: if action_type == BookAction::Sign { reader.string() } else { String::new() },
            author: if action_type == BookAction::Sign { reader.string() } else { String::new() },
            xuid: if action_type == BookAction::Sign { reader.string() } else { String::new() },
        }
    }
}
