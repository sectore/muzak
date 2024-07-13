use gpui::{AppContext, Context, EventEmitter, Global, Model};
use image::RgbaImage;

use crate::media::metadata::Metadata;

// yes this looks a little silly
impl EventEmitter<Metadata> for Metadata {}

pub struct Models {
    pub metadata: Model<Metadata>,
    pub albumart: Model<Option<RgbaImage>>,
}

impl Global for Models {}

pub fn build_models(cx: &mut AppContext) {
    let metadata: Model<Metadata> = cx.new_model(|_| Metadata::default());
    let albumart: Model<Option<RgbaImage>> = cx.new_model(|_| None);

    cx.set_global(Models { metadata, albumart });
}
