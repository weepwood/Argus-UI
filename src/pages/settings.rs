//! Compact appearance and density settings using the shared panel grammar.
use crate::{
    app::Shell,
    backdrop::{self, Backdrop},
    components::{self, ButtonTone},
    design::{Appearance, Density, MaterialClarity},
};
use gpui_kit::*;

fn setting_row(
    p: crate::design::Palette,
    title: &'static str,
    description: impl Into<SharedString>,
    control: impl IntoElement,
) -> Div {
    div()
        .flex()
        .min_h(px(66.))
        .px(px(15.))
        .items_center()
        .justify_between()
        .border_b_1()
        .border_color(p.border.opacity(0.7))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(3.))
                .child(div().text_size(px(12.)).text_color(p.text).child(title))
                .child(components::small_label(description, p)),
        )
        .child(control.into_element())
}

pub fn render(shell: &mut Shell, _window: &mut Window, cx: &mut Context<Shell>) -> AnyElement {
    let p = shell.palette();
    let theme = div()
        .flex()
        .gap(px(6.))
        .child(
            components::button(
                "light-mode",
                "浅色",
                p,
                if shell.appearance == Appearance::Light {
                    ButtonTone::Primary
                } else {
                    ButtonTone::Normal
                },
            )
            .on_click(
                cx.listener(|this, _, window, cx| this.set_theme(Appearance::Light, window, cx)),
            ),
        )
        .child(
            components::button(
                "dark-mode",
                "深色",
                p,
                if shell.appearance == Appearance::Dark {
                    ButtonTone::Primary
                } else {
                    ButtonTone::Normal
                },
            )
            .on_click(
                cx.listener(|this, _, window, cx| this.set_theme(Appearance::Dark, window, cx)),
            ),
        );
    let density = div()
        .flex()
        .gap(px(6.))
        .child(
            components::button(
                "comfortable-density",
                "舒适",
                p,
                if shell.density == Density::Comfortable {
                    ButtonTone::Primary
                } else {
                    ButtonTone::Normal
                },
            )
            .on_click(cx.listener(|this, _, _, cx| {
                this.set_density(Density::Comfortable, cx);
            })),
        )
        .child(
            components::button(
                "compact-density",
                "紧凑",
                p,
                if shell.density == Density::Compact {
                    ButtonTone::Primary
                } else {
                    ButtonTone::Normal
                },
            )
            .on_click(cx.listener(|this, _, _, cx| {
                this.set_density(Density::Compact, cx);
            })),
        );
    let clarity = div()
        .flex()
        .gap(px(6.))
        .children(MaterialClarity::ALL.map(|mode| {
            components::button(
                format!("clarity-{}", mode.id()),
                mode.title(),
                p,
                if shell.clarity == mode {
                    ButtonTone::Primary
                } else {
                    ButtonTone::Normal
                },
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                this.set_clarity(mode, window, cx);
            }))
        }));
    let mut backdrop_rows: Vec<AnyElement> = Vec::new();
    for chunk in Backdrop::ALL.chunks(3) {
        let mut row = div().flex().gap(px(6.));
        for mode in chunk {
            let mode = *mode;
            let tone = if shell.clarity == MaterialClarity::Sharp {
                ButtonTone::Ghost
            } else if shell.backdrop == mode {
                ButtonTone::Primary
            } else if mode.is_available() {
                ButtonTone::Normal
            } else {
                ButtonTone::Ghost
            };
            row = row.child(
                components::button(format!("backdrop-{}", mode.id()), mode.title(), p, tone)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.set_backdrop(mode, window, cx);
                    })),
            );
        }
        backdrop_rows.push(row.into_any_element());
    }
    let backdrop_buttons = div()
        .flex()
        .flex_col()
        .gap(px(6.))
        .items_end()
        .children(backdrop_rows);

    let panel = components::panel(p)
        .w(px(760.))
        .max_w_full()
        .flex_col()
        .child(setting_row(
            p,
            "文字渲染",
            shell.clarity.hint(),
            clarity,
        ))
        .child(setting_row(
            p,
            "窗口背景材质",
            shell.backdrop.hint(),
            backdrop_buttons,
        ))
        .child(setting_row(
            p,
            "系统材质支持",
            backdrop::support_note(),
            components::tag(backdrop::material_tag(), p, false),
        ))
        .child(setting_row(
            p,
            "抗锯齿机制",
            backdrop::antialiasing_note(),
            components::tag(backdrop::antialiasing_tag(), p, false),
        ))
        .child(setting_row(
            p,
            "颜色模式",
            "切换整个界面的浅色或深色语义色。",
            theme,
        ))
        .child(setting_row(
            p,
            "进程行密度",
            "调整进程表行高，同时保留稳定的列对齐。",
            density,
        ))
        .child(setting_row(
            p,
            "右侧详情栏",
            "在进程页显示硬件摘要或选中进程详情。",
            components::button(
                "details-setting",
                if shell.show_details {
                    "显示"
                } else {
                    "隐藏"
                },
                p,
                ButtonTone::Normal,
            )
            .on_click(cx.listener(|this, _, _, cx| {
                this.toggle_details(cx);
            })),
        ))
        .child(setting_row(
            p,
            "配置持久化",
            shell
                .preferences_error
                .clone()
                .unwrap_or_else(|| shell.preferences_file_location()),
            components::tag("自动保存", p, true),
        ))
        .child(setting_row(
            p,
            "图表动效",
            "控制示例图表进入页面时的绘制动效。",
            components::button(
                "motion-setting",
                if shell.animations { "开启" } else { "关闭" },
                p,
                ButtonTone::Normal,
            )
            .on_click(cx.listener(|this, _, _, cx| {
                this.toggle_animations(cx);
            })),
        ));
    div()
        .flex()
        .size_full()
        .items_start()
        .justify_center()
        .pt(px(18.))
        .child(panel)
        .into_any_element()
}
