//! Reusable, product-neutral primitives for the Argus-inspired UI system.
use gpui_kit::{
    assets::IconName,
    base::Button,
    component::{Icon, chart::LineChart},
    prelude::FluentBuilder as _,
    *,
};

use crate::design::Palette;

#[derive(Debug, Clone, Copy)]
pub enum ButtonTone {
    Normal,
    Primary,
    Ghost,
    Danger,
}

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
        .px(px(11.))
        .rounded(px(5.))
        .border_1()
        .border_color(stroke)
        .bg(background)
        .text_color(ink)
        .text_size(px(12.))
        .hover(move |s| {
            s.bg(
                if matches!(tone, ButtonTone::Primary | ButtonTone::Danger) {
                    background.opacity(0.86)
                } else {
                    p.hover
                },
            )
        })
        .active(move |s| s.bg(p.accent_soft))
        .child(label.into())
}

pub fn toolbar_button(
    id: &'static str,
    icon: IconName,
    label: impl Into<SharedString>,
    p: Palette,
    selected: bool,
) -> Stateful<Div> {
    div()
        .id(id)
        .h(px(34.))
        .px(px(9.))
        .flex()
        .items_center()
        .justify_center()
        .gap(px(6.))
        .rounded(px(5.))
        .border_1()
        .border_color(if selected { p.accent } else { p.border })
        .bg(if selected {
            p.accent_soft
        } else {
            p.surface.opacity(0.72)
        })
        .text_color(if selected { p.accent } else { p.text })
        .text_size(px(12.))
        .cursor_pointer()
        .hover(move |s| s.bg(p.hover))
        .child(Icon::new(icon).size(px(15.)))
        .child(label.into())
}

pub fn panel(p: Palette) -> Div {
    div()
        .bg(p.raised.opacity(0.42))
        .border_1()
        .border_color(p.border_strong.opacity(0.45))
        .rounded(px(7.))
}

pub fn heading(title: impl Into<SharedString>, p: Palette) -> Div {
    div()
        .text_size(px(18.))
        .font_weight(FontWeight::MEDIUM)
        .text_color(p.text)
        .child(title.into())
}

pub fn small_label(label: impl Into<SharedString>, p: Palette) -> Div {
    div()
        .text_size(px(11.))
        .text_color(p.muted)
        .child(label.into())
}

pub fn tag(text: impl Into<SharedString>, p: Palette, emphasized: bool) -> Div {
    div()
        .rounded(px(4.))
        .px(px(7.))
        .py(px(3.))
        .bg(if emphasized { p.accent_soft } else { p.control })
        .text_color(if emphasized { p.accent } else { p.muted })
        .text_size(px(10.))
        .child(text.into())
}

pub fn divider(p: Palette) -> Div {
    div().h(px(1.)).w_full().bg(p.border)
}

pub fn progress(p: Palette, color: Hsla, percent: f32) -> Div {
    let percent = percent.clamp(0., 100.);
    div()
        .h(px(5.))
        .w_full()
        .rounded(px(3.))
        .bg(p.control)
        .overflow_hidden()
        .child(
            div()
                .h_full()
                .w(relative(percent / 100.))
                .rounded(px(3.))
                .bg(color),
        )
}

pub fn sparkline(id: &'static str, values: &[f32], color: Hsla) -> AnyElement {
    div()
        .id(id)
        .h(px(50.))
        .w_full()
        .child(
            LineChart::new(
                values
                    .iter()
                    .enumerate()
                    .map(|(index, value)| (index.to_string(), *value))
                    .collect::<Vec<_>>(),
            )
            .x(|point| point.0.clone())
            .y(|point| point.1)
            .stroke(color)
            .x_axis(false)
            .grid(false)
            .interactive(false)
            .appear(false),
        )
        .into_any_element()
}

pub struct ResourceMetric<'a> {
    pub title: &'static str,
    pub ceiling: &'static str,
    pub value: &'static str,
    pub unit: &'static str,
    pub note: &'static str,
    pub color: Hsla,
    pub border_right: bool,
    pub values: &'a [f32],
}

impl<'a> ResourceMetric<'a> {
    pub fn render(self, p: Palette) -> Div {
        let Self {
            title,
            ceiling,
            value,
            unit,
            note,
            color,
            border_right,
            values,
        } = self;
        div()
            .flex_1()
            .min_w_0()
            .h_full()
            .px(px(16.))
            .pt(px(12.))
            .pb(px(7.))
            .flex()
            .flex_col()
            .when(border_right, |d| d.border_r_1().border_color(p.border))
            .bg(p.surface.opacity(0.68))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(7.))
                            .child(div().size(px(7.)).rounded(px(4.)).bg(color))
                            .child(div().text_size(px(11.)).text_color(p.muted).child(title)),
                    )
                    .child(div().text_size(px(10.)).text_color(p.faint).child(ceiling)),
            )
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap(px(5.))
                    .pt(px(7.))
                    .child(
                        div()
                            .text_size(px(28.))
                            .font_weight(FontWeight::LIGHT)
                            .text_color(p.text)
                            .child(value),
                    )
                    .child(div().text_size(px(12.)).text_color(p.muted).child(unit)),
            )
            .child(div().text_size(px(10.)).text_color(p.muted).child(note))
            .child(div().flex_1())
            .child(sparkline(title, values, color))
    }
}

pub fn color_swatch(name: &'static str, color: Hsla, p: Palette) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(7.))
        .child(div().size(px(12.)).rounded(px(3.)).bg(color))
        .child(small_label(name, p))
}
