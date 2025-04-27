use std::sync::Arc;

use gpui::{prelude::FluentBuilder, *};

use crate::{
    data::{
        events::{ImageLayout, ImageType},
        interface::GPUIDataInterface,
    },
    ui::{models::Models, theme::Theme, util::drop_image_from_app},
};

use super::{table_data::TableData, OnSelectHandler, TableLayout};

#[derive(Clone)]
pub struct TableItem<T>
where
    T: TableData + 'static,
{
    data: Option<Vec<Option<SharedString>>>,
    thumb: Option<Arc<RenderImage>>,
    image: Option<Arc<RenderImage>>,
    widths: Entity<Vec<f32>>,
    on_select: Option<OnSelectHandler<T>>,
    row: Option<Arc<T>>,
    id: Option<ElementId>,
    layout: TableLayout,
}

impl<T> TableItem<T>
where
    T: TableData + 'static,
{
    pub fn new(
        cx: &mut App,
        id: T::Identifier,
        widths: Entity<Vec<f32>>,
        on_select: Option<OnSelectHandler<T>>,
        layout: TableLayout,
        image_type: Option<ImageType>,
    ) -> Entity<Self> {
        let (row, data) = match layout {
            TableLayout::Row => {
                let row = T::get_row(cx, id).ok().flatten();
                let data = row.as_ref().map(|row| {
                    T::get_column_names()
                        .iter()
                        .map(|v| row.get_column(cx, v))
                        .collect()
                });
                (row, data)
            }
            TableLayout::Tiles(_) => {
                let row = T::get_tile(cx, id).ok().flatten();
                let data = row.as_ref().map(|row| {
                    T::get_tile_names()
                        .iter()
                        .map(|v| row.get_column(cx, v))
                        .collect()
                });
                (row, data)
            }
        };

        let thumb = row.as_ref().and_then(|row| row.get_thumb());

        cx.new(|cx| {
            cx.on_release(|this: &mut Self, cx: &mut App| {
                if let Some(image) = this.thumb.clone() {
                    drop_image_from_app(cx, image);
                    this.thumb = None;
                    cx.refresh_windows();
                }
                if let Some(image) = this.image.clone() {
                    drop_image_from_app(cx, image);
                    this.image = None;
                    cx.refresh_windows();
                }
            })
            .detach();

            let image = row.as_ref().and_then(|row| row.get_image().clone());
            if let (Some(image_type), Some(image)) = (image_type, image) {
                let image_transfer_model = cx.global::<Models>().image_transfer_model.clone();

                cx.subscribe(
                    &image_transfer_model,
                    move |this: &mut TableItem<T>, _, image, cx| {
                        if image.0 == image_type {
                            tracing::debug!("captured decoded image id: {:?}", image_type);
                            this.image = Some(image.1.clone());

                            cx.notify();
                        }
                    },
                )
                .detach();

                cx.global::<GPUIDataInterface>().decode_image(
                    image,
                    image_type,
                    ImageLayout::BGR,
                    false,
                );
            };

            let element_id = row.as_ref().map(|row| row.get_element_id().into());

            Self {
                data,
                thumb,
                image: None,
                widths,
                on_select,
                id: element_id,
                row,
                layout,
            }
        })
    }
}

impl<T> Render for TableItem<T>
where
    T: TableData + 'static,
{
    fn render(&mut self, _: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let row_data = self.row.clone();
        match self.layout {
            TableLayout::Row => {
                let mut row = div()
                    .w_full()
                    .flex()
                    .id(self.id.clone().unwrap_or("bad".into()))
                    .when_some(self.on_select.clone(), move |div, on_select| {
                        div.on_click(move |_, _, cx| {
                            let id = row_data.as_ref().unwrap().get_table_id();
                            on_select(cx, &id)
                        })
                        .hover(|this| this.bg(theme.nav_button_hover))
                        .active(|this| this.bg(theme.nav_button_active))
                    });

                if T::has_images() {
                    row = row.child(
                        div()
                            .w(px(53.0))
                            .h(px(36.0))
                            .text_sm()
                            .pl(px(17.0))
                            .flex_shrink_0()
                            .text_ellipsis()
                            //.border_r_1()
                            .border_color(theme.border_color)
                            .border_b_1()
                            .border_color(theme.border_color)
                            .flex()
                            .child(
                                div()
                                    .m_auto()
                                    .w(px(22.0))
                                    .h(px(22.0))
                                    .rounded(px(3.0))
                                    .bg(theme.album_art_background)
                                    .when_some(self.thumb.clone(), |div, image| {
                                        div.child(
                                            img(image).w(px(22.0)).h(px(22.0)).rounded(px(3.0)),
                                        )
                                    }),
                            ),
                    );
                }

                if let Some(data) = self.data.as_ref() {
                    for (i, column) in data.iter().enumerate() {
                        let width = self.widths.read(cx).get(i).cloned().unwrap_or(100.0);
                        let monospace = T::column_monospace()[i];
                        let column = div()
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
                            .when(monospace, |div| div.font_family("Roboto Mono"))
                            .text_sm()
                            .flex_shrink_0()
                            .overflow_hidden()
                            .text_ellipsis()
                            // .when(i != data.len() - 1, |div| {
                            //     div.border_r_1().border_color(theme.border_color)
                            // })
                            .border_b_1()
                            .border_color(theme.border_color)
                            .when_some(column.clone(), |div, string| div.child(string));

                        row = row.child(column);
                    }
                }

                row
            }
            TableLayout::Tiles(_) => {
                // tracing::warn!("RENDER {:?}", self.data);
                let mut tile = div()
                    .h_full()
                    .w_64()
                    .id(self.id.clone().unwrap_or("bad".into()))
                    .when_some(self.on_select.clone(), move |div, on_select| {
                        div.on_click(move |_, _, cx| {
                            let id = row_data.as_ref().unwrap().get_table_id();
                            on_select(cx, &id)
                        })
                        .hover(|this| this.bg(theme.nav_button_hover))
                        .active(|this| this.bg(theme.nav_button_active))
                    });

                if T::has_images() {
                    tile = tile.child(
                        div()
                            .relative()
                            .m_auto()
                            .w_full()
                            .h_full()
                            .rounded(px(3.0))
                            .bg(theme.album_art_background)
                            .when_some(self.thumb.clone(), |div, thumb| {
                                div.child(img(thumb).absolute().left_0().top_0().w_full().h_full())
                                    .rounded(px(3.0))
                            })
                            .when_some(self.image.clone(), |div, image| {
                                div.child(img(image).absolute().left_0().top_0().w_full().h_full())
                                    .rounded(px(3.0))
                            })
                            .when_some(self.data.as_ref(), |parent, data| {
                                let text_area = parent
                                    .child(div().flex().flex_row().absolute().left_0().bottom_10());

                                let text_area = data.iter().fold(text_area, |text_area, text| {
                                    text_area.child(
                                        div()
                                            .px_5()
                                            .py_2()
                                            .bg(gpui::transparent_black())
                                            .opacity(50.0)
                                            .when_some(text.clone(), |div, string| {
                                                div.child(string)
                                            }),
                                    )
                                });

                                text_area
                            }),
                    )
                }

                tile
            }
        }
    }
}
