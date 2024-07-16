use image::RgbaImage;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ImageType {
    CurrentAlbumArt,
    CachedImage(u64),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ImageLayout {
    BGR,
    RGB,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataCommand {
    // Requests that the data proccessing thread decode the specified image.
    DecodeImage(Box<[u8]>, ImageType, ImageLayout),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataEvent {
    // Indicates that the data processing thread has decoded the specified image.
    ImageDecoded(RgbaImage, ImageType),
    DecodeError(ImageType),
}
