use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::PacketType;
use crate::io::{Reader, Writer};

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum BookAction {
    ReplacePage,
    AddPage,
    DeletePage,
    SwapPages,
    Sign,
}

#[derive(Debug)]
pub struct BookEdit {
    pub action_type: BookAction,
    pub inventory_slot: u8,
    pub page_number: u8,
    pub secondary_page_number: u8,
    pub text: String,
    pub photo_name: String,
    pub title: String,
    pub author: String,
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
            text: if action_type == BookAction::ReplacePage || action_type == BookAction::AddPage { reader.string() } else { "".to_string() },
            photo_name: if action_type == BookAction::ReplacePage || action_type == BookAction::AddPage { reader.string() } else { "".to_string() },
            title: if action_type == BookAction::Sign { reader.string() } else { "".to_string() },
            author: if action_type == BookAction::Sign { reader.string() } else { "".to_string() },
            xuid: if action_type == BookAction::Sign { reader.string() } else { "".to_string() },
        }
    }
}
