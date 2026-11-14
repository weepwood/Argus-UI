//! Performance view built from the same chart, surface, and detail primitives.
use crate::{
    app::Shell,
    components,
    data::{CPU_SERIES, DISK_SERIES, GPU_SERIES, MEMORY_SERIES, NET_SERIES},
    design::Palette,
};
use gpui_kit::*;

fn chart_panel(
    p: Palette,
    title: &'static str,
    current: &'static str,
    color: Hsla,
    values: &[f32],
) -> Div {
    components::panel(p)
        .flex_1()
        .min_w_0()
        .h(px(224.))
        .flex_col()
        .px(px(16.))
        .pt(px(13.))
        .pb(px(8.))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(components::small_label(title, p))
                .child(
                    div()
                        .text_size(px(18.))
                        .font_weight(FontWeight::LIGHT)
                        .text_color(p.text)
                        .child(current),
                ),
        )
        .child(div().flex_1())
        .child(components::sparkline(title, values, color))
        .child(
            div()
                .flex()
                .justify_between()
                .text_size(px(10.))
                .text_color(p.faint)
                .child("最近 60 秒")
                .child("实时采样 · 模拟数据"),
        )
}

fn item(p: Palette, title: &'static str, value: &'static str) -> Div {
    div()
        .flex()
        .py(px(9.))
        .items_center()
        .justify_between()
        .border_b_1()
        .border_color(p.border.opacity(0.7))
        .child(components::small_label(title, p))
        .child(div().text_size(px(11.)).text_color(p.text).child(value))
}

pub fn render(shell: &mut Shell, _: &mut Window, _: &mut Context<Shell>) -> AnyElement {
    let p = shell.palette();
    let charts = div()
        .flex_1()
        .min_w_0()
        .flex_col()
        .gap(px(9.))
        .child(
            div()
                .flex()
                .gap(px(9.))
                .child(chart_panel(p, "CPU", "41%", p.cpu, CPU_SERIES))
                .child(chart_panel(p, "内存", "12.3 GB", p.memory, MEMORY_SERIES)),
        )
        .child(
            div()
                .flex()
                .gap(px(9.))
                .child(chart_panel(p, "磁盘 I/O", "7.1 MB/s", p.disk, DISK_SERIES))
                .child(chart_panel(p, "网络", "22.7 MB/s", p.network, NET_SERIES)),
        )
        .child(
            div()
                .flex()
                .gap(px(9.))
                .child(chart_panel(p, "GPU", "42%", p.gpu, GPU_SERIES))
                .child(
                    components::panel(p)
                        .flex_1()
                        .min_w_0()
                        .h(px(224.))
                        .px(px(16.))
                        .py(px(13.))
                        .flex_col()
                        .child(components::small_label("资源利用率范围", p))
                        .child(div().flex_1())
                        .child(item(p, "CPU 使用率", "0–100%"))
                        .child(item(p, "物理内存", "32.0 GB"))
                        .child(item(p, "磁盘传输率", "100 MB/s"))
                        .child(item(p, "网络传输率", "50 MB/s")),
                ),
        );
    let details = components::panel(p)
        .w(px(300.))
        .h_full()
        .flex_shrink_0()
        .p(px(17.))
        .flex_col()
        .gap(px(7.))
        .child(components::heading("系统性能", p))
        .child(components::small_label("硬件读数与活跃设备", p))
        .child(div().h(px(5.)))
        .child(item(p, "处理器", "Intel Core i5-13500HX"))
        .child(item(p, "逻辑处理器", "20"))
        .child(item(p, "基础频率", "2.50 GHz"))
        .child(item(p, "内存", "12.3 / 32.0 GB"))
        .child(item(p, "磁盘活动", "6%"))
        .child(item(p, "GPU 引擎", "3D · 42%"))
        .child(div().flex_1())
        .child(components::tag("所有数值均为模拟数据", p, false));
    div()
        .flex()
        .size_full()
        .gap(px(9.))
        .child(charts)
        .child(details)
        .into_any_element()
}
