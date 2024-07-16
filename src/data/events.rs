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
    /// Requests that the data proccessing thread decode the specified image. The image type is
    /// used to keep track of which image is being decoded, and the layout is used to determine
    /// whether or not RGB to BGR conversion is necessary.
    DecodeImage(Box<[u8]>, ImageType, ImageLayout),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataEvent {
    /// Indicates that the data processing thread has decoded the specified image.
    ImageDecoded(RgbaImage, ImageType),
    /// Indicates that the data processing thread has encountered an error while decoding the
    /// specified image.
    DecodeError(ImageType),
}
