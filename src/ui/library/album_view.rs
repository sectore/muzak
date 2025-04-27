use std::{collections::VecDeque, rc::Rc};

use gpui::*;

use crate::{
    library::{scan::ScanEvent, types::Album},
    ui::{
        components::table::{Table, TableEvent, TableLayout},
        models::Models,
    },
};

use super::ViewSwitchMessage;

#[derive(Clone)]
pub struct AlbumView {
    table: Entity<Table<Album>>,
    layout: Entity<TableLayout>,
}

impl AlbumView {
    pub(super) fn new(
        cx: &mut App,
        view_switch_model: Entity<VecDeque<ViewSwitchMessage>>,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let state = cx.global::<Models>().scan_state.clone();

            let handler = Rc::new(move |cx: &mut App, id: &(u32, String)| {
                view_switch_model
                    .update(cx, |_, cx| cx.emit(ViewSwitchMessage::Release(id.0 as i64)))
            });

            // TODO: Switch layout
            // let layout = cx.new(|_| TableLayout::Row);
            let layout = cx.new(|_| TableLayout::Tiles(3));
            let current_layout = layout.read(cx).clone();
            let table = Table::new(cx, Some(handler), current_layout);

            let table_clone = table.clone();

            cx.observe(&state, move |_: &mut AlbumView, e, cx| {
                let value = e.read(cx);
                match value {
                    ScanEvent::ScanCompleteIdle => {
                        table_clone.update(cx, |_, cx| cx.emit(TableEvent::NewRows));
                    }
                    ScanEvent::ScanProgress { current, .. } => {
                        if current % 100 == 0 {
                            table_clone.update(cx, |_, cx| cx.emit(TableEvent::NewRows));
                        }
                    }
                    _ => {}
                }
            })
            .detach();

            AlbumView { table, layout }
        })
    }
}

impl Render for AlbumView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .max_w(px(1000.0))
            .mx_auto()
            .pt(px(24.0))
            .pb(px(0.0))
            .child(self.table.clone())
    }
}
