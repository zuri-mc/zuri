use bytes::Bytes;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum PhotoType {
    Portfolio,
    PhotoItem,
    Book,
}

/// Sent by the server to transfer a photo (image) file to the client. It is typically used to
/// transfer photos so that the client can display it in a portfolio in Education Edition. While
/// previously usable in the default Bedrock Edition, the displaying of photos in books was disabled
/// and the packet now has little use anymore.
#[derive(Debug, Clone)]
pub struct PhotoTransfer {
    /// The name of the photo to transfer. It is the exact file name that the client will download
    /// the photo as, including the extension of the file.
    pub photo_name: String,
    /// The raw data of the photo image. The format of this data may vary: Formats such as JPEG or
    /// PNG work, as long as `photo_name` has the correct extension.
    pub photo_data: Bytes,
    /// The ID of the book that the photo is associated with. If the `photo_name` in a book with
    /// this ID is set to `photo_name`, it will display the photo (provided Education Edition is
    /// used). The photo image is downloaded to a sub-folder with this book ID.
    pub book_id: String,
    /// The type of photo being transferred. It is used to determine where the photo is stored.
    pub photo_type: PhotoType,
    /// The source photo type. It is one of the three photo types above.
    pub source_type: u8,
    /// The entity unique ID of the photo's owner.
    pub owner_entity_unique_id: i64,
    /// The new name of the photo.
    pub new_photo_name: String,
}

impl PacketType for PhotoTransfer {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.photo_name.as_str());
        writer.byte_slice(&self.photo_data);
        writer.string(self.book_id.as_str());
        writer.u8(self.photo_type.to_u8().unwrap());
        writer.u8(self.source_type);
        writer.i64(self.owner_entity_unique_id);
        writer.string(self.new_photo_name.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            photo_name: reader.string(),
            photo_data: reader.byte_slice(),
            book_id: reader.string(),
            photo_type: PhotoType::from_u8(reader.u8()).unwrap(),
            source_type: reader.u8(),
            owner_entity_unique_id: reader.i64(),
            new_photo_name: reader.string(),
        }
    }
}
