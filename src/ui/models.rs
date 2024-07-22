use gpui::{AppContext, Context, EventEmitter, Global, Model};
use image::RgbaImage;

use crate::{
    data::{
        events::{ImageLayout, ImageType},
        interface::GPUIDataInterface,
        types::UIQueueItem,
    },
    media::metadata::Metadata,
    playback::thread::PlaybackState,
};

// yes this looks a little silly
impl EventEmitter<Metadata> for Metadata {}

#[derive(Debug, PartialEq, Clone)]
pub struct ImageEvent(pub Box<[u8]>);

impl EventEmitter<ImageEvent> for Option<RgbaImage> {}

pub struct Models {
    pub metadata: Model<Metadata>,
    pub albumart: Model<Option<RgbaImage>>,
    pub queue: Model<Vec<UIQueueItem>>,
}

impl Global for Models {}

#[derive(Clone)]
pub struct PlaybackInfo {
    pub position: Model<u64>,
    pub duration: Model<u64>,
    pub playback_state: Model<PlaybackState>,
    pub current_track: Model<Option<String>>,
}

impl Global for PlaybackInfo {}

pub fn build_models(cx: &mut AppContext) {
    let metadata: Model<Metadata> = cx.new_model(|_| Metadata::default());
    let albumart: Model<Option<RgbaImage>> = cx.new_model(|_| None);
    let queue: Model<Vec<UIQueueItem>> = cx.new_model(|_| Vec::new());

    cx.subscribe(&albumart, |_, ev, cx| {
        let img = ev.0.clone();
        cx.global::<GPUIDataInterface>().decode_image(
            img,
            ImageType::CurrentAlbumArt,
            ImageLayout::BGR,
        );
    })
    .detach();

    cx.set_global(Models {
        metadata,
        albumart,
        queue,
    });

    let position: Model<u64> = cx.new_model(|_| 0);
    let duration: Model<u64> = cx.new_model(|_| 0);
    let playback_state: Model<PlaybackState> = cx.new_model(|_| PlaybackState::Stopped);
    let current_track: Model<Option<String>> = cx.new_model(|_| None);

    cx.set_global(PlaybackInfo {
        position,
        duration,
        playback_state,
        current_track,
    });
}
