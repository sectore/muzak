pub mod table_data;
mod table_item;

use std::rc::Rc;

use ahash::AHashMap;
use gpui::{prelude::FluentBuilder, *};
use image::ImageFormat;
use table_data::{TableData, TableSort};
use table_item::TableItem;
use tracing::warn;

use crate::{
    data::events::ImageType,
    ui::{
        constants::FONT_AWESOME,
        theme::Theme,
        util::{create_or_retrieve_view, prune_views},
    },
};

type RowMap<T> = AHashMap<usize, Entity<TableItem<T>>>;
// pub type ViewsModelMap<T: Render> = AHashMap<usize, Entity<T>>;

#[derive(Clone, PartialEq)]
pub enum TableLayout {
    /// Row means one element in a row
    Row,
    // Tiles means as many elements in a row
    Tiles(usize),
}

impl TableLayout {
    pub fn is_tiles(&self) -> bool {
        match self {
            TableLayout::Tiles(_) => true,
            _ => false,
        }
    }
}

#[allow(type_alias_bounds)]
pub type OnSelectHandler<T>
where
    T: TableData,
= Rc<dyn Fn(&mut App, &T::Identifier) + 'static>;

#[derive(Clone)]
pub struct Table<T>
where
    T: TableData + 'static,
{
    columns: &'static [&'static str],
    widths: Entity<Vec<f32>>,
    views: Entity<RowMap<T>>,
    render_counter: Entity<usize>,
    list_state: ListState,
    sort_method: Entity<Option<TableSort>>,
    on_select: Option<OnSelectHandler<T>>,
    layout: TableLayout,
}

pub enum TableEvent {
    NewRows,
}

impl<T> EventEmitter<TableEvent> for Table<T> where T: TableData {}

impl<T> Table<T>
where
    T: TableData + 'static,
{
    pub fn new(
        cx: &mut App,
        on_select: Option<OnSelectHandler<T>>,
        layout: TableLayout,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let widths = cx.new(|_| T::default_column_widths());
            let views = cx.new(|_| AHashMap::new());
            let render_counter = cx.new(|_| 0);
            let sort_method = cx.new(|_| None);

            let list_state = Self::make_list_state(
                cx,
                views.clone(),
                render_counter.clone(),
                &sort_method,
                widths.clone(),
                on_select.clone(),
                layout.clone(),
            );

            cx.observe(&sort_method, |this, _, cx| {
                this.regenerate_list_state(cx);
                cx.notify();
            })
            .detach();

            cx.subscribe(&cx.entity(), |this, _, event, cx| match event {
                TableEvent::NewRows => this.regenerate_list_state(cx),
            })
            .detach();

            Self {
                columns: T::get_column_names(),
                widths,
                views,
                render_counter,
                list_state,
                sort_method,
                on_select,
                layout,
            }
        })
    }

    fn regenerate_list_state(&mut self, cx: &mut Context<'_, Self>) {
        let curr_scroll = self.list_state.logical_scroll_top();
        self.views = cx.new(|_| AHashMap::new());
        self.render_counter = cx.new(|_| 0);

        self.list_state = Self::make_list_state(
            cx,
            self.views.clone(),
            self.render_counter.clone(),
            &self.sort_method,
            self.widths.clone(),
            self.on_select.clone(),
            self.layout.clone(),
        );

        self.list_state.scroll_to(curr_scroll);

        cx.notify();
    }

    fn make_list_state(
        cx: &mut Context<'_, Self>,
        views: Entity<RowMap<T>>,
        render_counter: Entity<usize>,
        sort_method_entity: &Entity<Option<TableSort>>,
        widths: Entity<Vec<f32>>,
        handler: Option<OnSelectHandler<T>>,
        layout: TableLayout,
    ) -> ListState {
        let sort_method = *sort_method_entity.read(cx);
        let Ok(rows) = T::get_rows(cx, sort_method) else {
            warn!("Failed to get rows");
            return ListState::new(0, ListAlignment::Top, px(64.0), move |_, _, _| {
                div().into_any_element()
            });
        };

        match layout.clone() {
            TableLayout::Row => {
                let idents_rc = Rc::new(rows);

                ListState::new(
                    idents_rc.len(),
                    ListAlignment::Top,
                    px(300.0),
                    move |idx, _, cx| {
                        let idents_rc = idents_rc.clone();
                        prune_views(&views, &render_counter, idx, cx);
                        div()
                            .w_full()
                            .child(create_or_retrieve_view(
                                &views,
                                idx,
                                |cx| {
                                    TableItem::new(
                                        cx,
                                        idents_rc[idx].clone(),
                                        widths.clone(),
                                        handler.clone(),
                                        layout.clone(),
                                        None,
                                    )
                                },
                                cx,
                            ))
                            .into_any_element()
                    },
                )
            }
            TableLayout::Tiles(no) => {
                // Create chunks of size 'no'
                let chunked: Vec<Vec<_>> = rows.chunks(no).map(|chunk| chunk.to_vec()).collect();
                let idents_rc = Rc::new(chunked);
                tracing::warn!("RC {:?}", idents_rc);
                ListState::new(
                    idents_rc.len(),
                    ListAlignment::Top,
                    px(300.0),
                    move |idx, _, cx| {
                        let mut row = div().h_full().flex().flex_row().justify_start();
                        let idents_rc = idents_rc.clone();
                        let idents: Vec<T::Identifier> = idents_rc[idx].clone();
                        // tracing::warn!("items {:?}", items);
                        for (i, id) in idents.iter().enumerate() {
                            let current_idx = idx * idents.len() + i;
                            // tracing::warn!("current_idx {:?}", current_idx);
                            tracing::error!("{:?} {:?} {:?}", i, current_idx, id);
                            // prune_views(&views, &render_counter, current_idx, cx);
                            row = row.child(create_or_retrieve_view(
                                &views,
                                current_idx,
                                |cx| {
                                    tracing::warn!("RENDER {:?} {:?} {:?}", i, id, current_idx);
                                    TableItem::new(
                                        cx,
                                        id.clone(),
                                        widths.clone(),
                                        handler.clone(),
                                        layout.clone(),
                                        Some(ImageType::AlbumArt(current_idx as i64)),
                                    )
                                },
                                cx,
                            ));
                        }
                        div().h_64().w_full().child(row).into_any_element()
                    },
                )
            }
        }
    }
}

impl<T> Render for Table<T>
where
    T: TableData + 'static,
{
    fn render(&mut self, _: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let mut header = div().w_full().flex();
        let theme = cx.global::<Theme>();
        let sort_method = self.sort_method.read(cx);

        if T::has_images() {
            header = header.child(
                div()
                    .w(px(53.0))
                    .h(px(36.0))
                    .pl(px(27.0))
                    .pr(px(10.0))
                    .py(px(2.0))
                    .text_sm()
                    .flex_shrink_0()
                    .text_ellipsis()
                    // .border_r_1()
                    .border_color(theme.border_color)
                    .border_b_1()
                    .border_color(theme.border_color),
            );
        }

        for (i, column) in self.columns.iter().enumerate() {
            let width = self.widths.read(cx)[i];
            if self.layout == TableLayout::Row {
                header = header.child(
                    div()
                        .flex()
                        .w(px(width))
                        .when(T::has_images(), |div| {
                            div.h(px(36.0)).px(px(12.0)).py(px(6.0))
                        })
                        .when(!T::has_images(), |div| {
                            div.h(px(30.0))
                                .px(px(10.0))
                                .py(px(2.0))
                                .when(i == 0, |div| div.pl(px(27.0)))
                        })
                        .text_sm()
                        .flex_shrink_0()
                        // .when(i != self.columns.len() - 1, |div| {
                        //     div.border_r_1().border_color(theme.border_color)
                        // })
                        .border_b_1()
                        .border_color(theme.border_color)
                        .font_weight(FontWeight::BOLD)
                        .child(SharedString::new_static(column))
                        .when_some(sort_method.as_ref(), |this, method| {
                            this.when(method.column == *column, |this| {
                                this.child(
                                    div()
                                        .ml(px(7.0))
                                        .text_size(px(10.0))
                                        .my_auto()
                                        .font_family(FONT_AWESOME)
                                        .when(method.ascending, |div| div.child(""))
                                        .when(!method.ascending, |div| div.child("")),
                                )
                            })
                        })
                        .id(*column)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.sort_method.update(cx, |this, cx| {
                                if let Some(method) = this.as_mut() {
                                    if method.column == *column {
                                        method.ascending = !method.ascending;
                                    } else {
                                        *this = Some(TableSort {
                                            column,
                                            ascending: true,
                                        });
                                    }
                                } else {
                                    *this = Some(TableSort {
                                        column,
                                        ascending: true,
                                    });
                                }

                                cx.notify();
                            })
                        })),
                );
            }
        }

        div()
            .id(T::get_table_name())
            .overflow_x_scroll()
            .w_full()
            .h_full()
            .child(
                div()
                    .w_full()
                    .pb(px(11.0))
                    .px(px(24.0))
                    .line_height(px(26.0))
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(26.0))
                    .child(T::get_table_name()),
            )
            .child(header)
            .child(list(self.list_state.clone()).w_full().h_full())
    }
}
