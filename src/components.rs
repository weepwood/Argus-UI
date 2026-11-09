//! Stateless visual primitives, independent of the application's business state.
use gpui_kit::{*, base::Button};
use crate::design::Palette;

#[derive(Debug, Clone, Copy)]
pub enum ButtonTone { Normal, Primary, Ghost, Danger }

pub fn button(
    id: impl Into<SharedString>,
    label: impl Into<SharedString>,
    p: Palette,
    tone: ButtonTone,
) -> Button {
    let (background, ink, stroke) = match tone {
        ButtonTone::Normal => (p.control, p.text, p.border),
        ButtonTone::Primary => (p.accent, p.accent_ink, p.accent),
        ButtonTone::Ghost => (p.surface, p.text, p.surface),
        ButtonTone::Danger => (p.danger, p.accent_ink, p.danger),
    };
    Button::new(id.into())
        .h(px(32.))
        .px(px(12.))
        .rounded(px(5.))
        .border_1()
        .border_color(stroke)
        .bg(background)
        .text_color(ink)
        .text_size(px(13.))
        .hover(move |s| s.bg(if matches!(tone, ButtonTone::Primary | ButtonTone::Danger) {
            background.opacity(0.85)
        } else {p.hover}))
        .active(move |s| s.bg(p.accent_soft))
        .child(label.into())
}

pub fn panel(p: Palette) -> Div {
    div()
        .bg(p.surface)
        .border_1()
        .border_color(p.border)
        .rounded(px(8.))
}

pub fn heading(title: impl Into<SharedString>, p: Palette) -> Div {
    div()
        .text_size(px(20.))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(p.text)
        .child(title.into())
}

pub fn small_label(label: impl Into<SharedString>, p: Palette) -> Div {
    div().text_size(px(12.)).text_color(p.muted).child(label.into())
}

pub fn tag(text: impl Into<SharedString>, p: Palette, emphasized: bool) -> Div {
    div()
        .rounded(px(4.))
        .px(px(7.))
        .py(px(3.))
        .bg(if emphasized {p.accent_soft} else {p.control})
        .text_color(if emphasized {p.accent} else {p.muted})
        .text_size(px(11.))
        .child(text.into())
}

pub fn divider(p: Palette) -> Div {
    div().h(px(1.)).w_full().bg(p.border)
}

pub fn progress(p: Palette, color: Hsla, percent: f32) -> Div {
    let percent = percent.clamp(0., 100.);
    div()
        .h(px(6.))
        .w_full()
        .rounded(px(4.))
        .bg(p.control)
        .overflow_hidden()
        .child(
            div()
                .h_full()
                .w(relative(percent / 100.))
                .rounded(px(4.))
                .bg(color),
        )
}

pub fn sparkline(samples: &[f32], color: Hsla) -> Div {
    let data: Vec<f32> = samples.to_vec();
    div()
        .h(px(54.))
        .flex()
        .items_end()
        .gap(px(3.))
        .children(data.into_iter().map(move |v| {
            div()
                .w(px(6.))
                .h(px((v / 100. * 50.).max(5.)))
                .rounded(px(2.))
                .bg(color.opacity(0.85))
        }))
}

pub fn color_swatch(name: &'static str, color: Hsla, p: Palette) -> Div {
    div().flex().items_center().gap(px(7.))
       .child(div().size(px(12.)).rounded(px(3.)).bg(color))
       .child(small_label(name,p))
}
