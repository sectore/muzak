use std::process::Child;

use gpui::*;

#[derive(IntoElement)]
pub struct Header {}

#[cfg(not(target_os = "macos"))]
impl RenderOnce for Header {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .w_full()
            .h(px(60.0))
            .bg(rgb(0x111827))
            .on_mouse_move(|_e, cx| cx.refresh())
            .on_mouse_down(MouseButton::Left, move |e, cx| cx.start_window_move())
            .flex()
            .child(InfoSection {})
            .child(PlaybackSection::default())
    }
}

#[cfg(target_os = "macos")]
impl RenderOnce for Header {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .w_full()
            .h(px(60.0))
            .bg(rgb(0x111827))
            .on_mouse_move(|_e, cx| cx.refresh())
            .on_mouse_down(MouseButton::Left, move |e, cx| cx.start_window_move())
            .flex()
            .child(WindowControls {})
            .child(InfoSection {})
            .child(PlaybackSection::default())
    }
}

#[derive(IntoElement)]
pub struct InfoSection {}

impl RenderOnce for InfoSection {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .m(px(12.0))
            .gap(px(10.0))
            .flex()
            .child(
                div()
                    .rounded(px(4.0))
                    .bg(rgb(0x4b5563))
                    .shadow_sm()
                    .w(px(36.0))
                    .h(px(36.0)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .line_height(rems(1.0))
                    .text_size(px(15.0))
                    .gap_1()
                    .child(
                        div()
                            .font_weight(FontWeight::EXTRA_BOLD)
                            .child(format!("{}", "Artist Name")),
                    )
                    .child(div().child(format!("{}", "Track Name"))),
            )
    }
}

#[derive(IntoElement, Default)]
pub struct PlaybackSection {
    play_hovered: bool,
    prev_hovered: bool,
    next_hovered: bool,
}

impl RenderOnce for PlaybackSection {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .child(
                div()
                    .w(px(51.0))
                    .h(px(30.0))
                    .pl(px(21.0))
                    .mr(px(-21.0))
                    .rounded(px(15.0))
                    .bg(rgb(0x1f2937)),
            )
            .child(deferred(
                div()
                    .w(px(42.0))
                    .h(px(42.0))
                    .rounded(px(21.0))
                    .bg(rgb(0x374151)),
            ))
            .child(
                div()
                    .w(px(51.0))
                    .h(px(30.0))
                    .pl(px(21.0))
                    .ml(px(-21.0))
                    .rounded(px(15.0))
                    .bg(rgb(0x1f2937)),
            )
    }
}

#[derive(IntoElement)]
pub struct WindowControls {}

#[cfg(target_os = "macos")]
impl RenderOnce for WindowControls {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div().w(px(65.0)).h_full()
    }
}
