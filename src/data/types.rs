use std::sync::Arc;

use gpui::ImageData;

use crate::media::metadata::Metadata;

#[derive(Debug, Clone)]
pub struct UIQueueItem {
    pub metadata: Metadata,
    pub file_path: String,
    pub album_art: Option<Arc<ImageData>>,
}
