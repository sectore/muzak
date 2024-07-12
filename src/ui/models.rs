use std::{
    thread::{current, sleep, sleep_ms},
    time::Duration,
};

use gpui::{AppContext, Context, EventEmitter, Global, Model};

use crate::media::metadata::Metadata;

// yes this looks a little silly
impl EventEmitter<Metadata> for Metadata {}

pub struct Models {
    pub metadata: Model<Metadata>,
}

impl Global for Models {}

pub fn build_models(cx: &mut AppContext) {
    let metadata: Model<Metadata> = cx.new_model(|_| Metadata::default());

    cx.subscribe(&metadata, |model, event: &Metadata, cx| {
        println!(
            "metadata update on thread {:?}: {:?}",
            current().name(),
            event
        );
        cx.update_model(&model, |model, cx| {
            *model = event.clone();
            cx.notify();
        })
    })
    .detach();

    cx.set_global(Models { metadata });
}
