use bytes::Bytes;
use zuri_net_derive::proto;

/// Sent by the server to transfer a photo (image) file to the client. It is typically used to
/// transfer photos so that the client can display it in a portfolio in Education Edition. While
/// previously usable in the default Bedrock Edition, the displaying of photos in books was disabled
/// and the packet now has little use anymore.
#[proto]
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

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum PhotoType {
    Portfolio,
    PhotoItem,
    Book,
}
