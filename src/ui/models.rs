use gpui::{AppContext, Context, EventEmitter, Global, Model};
use image::RgbaImage;

use crate::{
    data::{
        events::{ImageLayout, ImageType},
        interface::GPUIDataInterface,
    },
    media::metadata::Metadata,
};

// yes this looks a little silly
impl EventEmitter<Metadata> for Metadata {}

#[derive(Debug, PartialEq, Clone)]
pub struct ImageEvent(pub Box<[u8]>);

impl EventEmitter<ImageEvent> for Option<RgbaImage> {}

pub struct Models {
    pub metadata: Model<Metadata>,
    pub albumart: Model<Option<RgbaImage>>,
}

impl Global for Models {}

pub fn build_models(cx: &mut AppContext) {
    let metadata: Model<Metadata> = cx.new_model(|_| Metadata::default());
    let albumart: Model<Option<RgbaImage>> = cx.new_model(|_| None);

    cx.subscribe(&albumart, |_, ev, cx| {
        let img = ev.0.clone();
        cx.global::<GPUIDataInterface>().decode_image(
            img,
            ImageType::CurrentAlbumArt,
            ImageLayout::BGR,
        );
    })
    .detach();

    cx.set_global(Models { metadata, albumart });
}
