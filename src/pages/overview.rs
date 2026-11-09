//! Dashboard fixture: layout hierarchy and restrained live-data visual language.
use gpui_kit::*;
use crate::{
    app::Shell, components,
    data::{CPU_SERIES, MEMORY_SERIES, DISK_SERIES, NET_SERIES, PROCESSES},
    design::Palette,
};

fn metric(
    p: Palette, label: &'static str, value: &'static str, foot: &'static str,
    color: Hsla, values: &[f32],
) -> Div {
    components::panel(p)
        .flex_1().min_w_0().p(px(17.))
        .child(
            div().flex().justify_between().items_center()
                .child(components::small_label(label,p))
                .child(div().size(px(8.)).rounded(px(4.)).bg(color))
        )
        .child(div().h(px(10.)))
        .child(div().text_size(px(28.)).font_weight(FontWeight::SEMIBOLD).text_color(p.text).child(value))
        .child(div().h(px(2.)))
        .child(components::small_label(foot,p))
        .child(div().h(px(14.)))
        .child(components::sparkline(values,color))
}
fn resource(p: Palette, title: &'static str, amount: &'static str, value: f32, color:Hsla) -> Div {
    div().flex().flex_col().gap(px(8.))
        .child(div().flex().justify_between().items_center()
            .child(div().text_color(p.text).child(title))
            .child(components::small_label(amount,p)))
        .child(components::progress(p,color,value))
}
pub fn render(shell: &mut Shell, _: &mut Window, _: &mut Context<Shell>) -> AnyElement {
    let p=shell.palette();
    let busiest=crate::data::filtered_sorted("",crate::data::SortBy::Cpu);
    div().flex().flex_col()
        .w_full().p(px(26.)).gap(px(19.))
        .child(
            div().flex().items_center().justify_between()
                .child(div().flex().flex_col().gap(px(6.))
                    .child(components::heading("系统概览",p))
                    .child(components::small_label("面向高密度桌面应用的 GPUI 原生视觉基线",p)))
                .child(components::tag("● 所有数值均为模拟数据",p,true))
        )
        .child(
            div().flex().flex_col().gap(px(12.))
                .child(div().flex().gap(px(12.))
                    .child(metric(p,"处理器 · CPU","42.8%","模拟使用率 / 28 个采样",p.cpu,CPU_SERIES))
                    .child(metric(p,"内存 · MEMORY","12.4 GB","已用 12.4 / 32 GB",p.memory,MEMORY_SERIES)))
                .child(div().flex().gap(px(12.))
                    .child(metric(p,"磁盘 · DISK","27.1 MB/s","当前读写吞吐量",p.disk,DISK_SERIES))
                    .child(metric(p,"网络 · NETWORK","16.8 Mbps","实时网络吞吐量",p.network,NET_SERIES)))
        )
        .child(
            div().flex().items_start().gap(px(14.))
                .child(
                    components::panel(p).flex_1().min_w_0().p(px(20.))
                        .child(div().flex().justify_between().items_center()
                            .child(div().text_size(px(14.)).font_weight(FontWeight::SEMIBOLD).child("资源利用率"))
                            .child(components::tag("PERFORMANCE",p,false)))
                        .child(div().h(px(9.)))
                        .child(components::small_label("展示色彩层级、间距与进度控件的复用方式",p))
                        .child(div().h(px(26.)))
                        .child(div().flex().flex_col().gap(px(22.))
                            .child(resource(p,"CPU","42.8%",42.8,p.cpu))
                            .child(resource(p,"物理内存","38.7%",38.7,p.memory))
                            .child(resource(p,"磁盘活动","27.1%",27.1,p.disk))
                            .child(resource(p,"GPU","12.5%",12.5,p.gpu))
                        )
                        .child(div().h(px(24.)))
                        .child(components::divider(p))
                        .child(div().h(px(13.)))
                        .child(components::small_label("图形使用原生 GPUI 元素绘制；可接入真实指标或替换为 TimePlot。",p))
                )
                .child(
                    components::panel(p).w(px(286.)).flex_shrink_0().p(px(18.))
                        .child(div().text_size(px(14.)).font_weight(FontWeight::SEMIBOLD).child("高占用进程"))
                        .child(div().h(px(8.)))
                        .child(components::small_label("按模拟 CPU 使用率排序",p))
                        .child(div().h(px(19.)))
                        .children(busiest.into_iter().take(5).map(|item| {
                            div().flex().flex_col().gap(px(7.)).pb(px(17.))
                                .child(div().flex().justify_between().items_center()
                                    .child(div().text_color(p.text).text_size(px(12.)).child(item.name))
                                    .child(div().text_color(p.muted).text_size(px(12.)).child(format!("{:.1}%",item.cpu))))
                                .child(components::progress(p,p.cpu,item.cpu*2.5))
                        }))
                        .child(components::divider(p))
                        .child(div().pt(px(13.)).text_size(px(11.)).text_color(p.faint)
                            .child(format!("{} 条演示进程 · 本地无特权",PROCESSES.len())))
                )
        )
        .into_any_element()
}
