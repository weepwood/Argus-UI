//! Dense process table and device/detail rail, composed from reusable primitives.
use crate::{
    app::Shell,
    components,
    data::{PROCESSES, Process, SortBy, filtered_sorted},
    design::Palette,
};
use gpui_kit::{
    assets::IconName,
    component::{Icon, scroll::ScrollableElement as _},
    prelude::FluentBuilder as _,
    *,
};

const COLUMNS: [(f32, &str); 7] = [
    (62., "PID"),
    (78., "CPU"),
    (96., "内存"),
    (88., "磁盘读取"),
    (88., "磁盘写入"),
    (84., "网络"),
    (62., "GPU"),
];

fn numeric_cell(width: f32, value: impl Into<SharedString>, color: Hsla, _p: Palette) -> Div {
    div()
        .w(px(width))
        .flex_shrink_0()
        .px(px(4.))
        .flex()
        .items_center()
        .justify_end()
        .text_size(px(11.))
        .text_color(color)
        .font_family("Segoe UI Variable")
        .child(value.into())
}

fn table_header(p: Palette, totals: (String, String, String, String, String, String)) -> Div {
    div()
        .flex()
        .h(px(38.))
        .w_full()
        .px(px(11.))
        .items_center()
        .border_b_1()
        .border_color(p.border)
        .bg(p.surface.opacity(0.25))
        .text_color(p.muted)
        .child(div().flex_1().min_w_0().text_size(px(10.)).child("名称  ↑"))
        .child(numeric_cell(COLUMNS[0].0, "PID", p.muted, p))
        .child(numeric_cell(COLUMNS[1].0, totals.0, p.muted, p))
        .child(numeric_cell(COLUMNS[2].0, totals.1, p.muted, p))
        .child(numeric_cell(COLUMNS[3].0, totals.2, p.muted, p))
        .child(numeric_cell(COLUMNS[4].0, totals.3, p.muted, p))
        .child(numeric_cell(COLUMNS[5].0, totals.4, p.muted, p))
        .child(numeric_cell(COLUMNS[6].0, totals.5, p.muted, p))
}

fn process_row(shell: &Shell, process: Process, cx: &mut Context<Shell>) -> AnyElement {
    let p = shell.palette();
    let selected = shell.selected_pid == Some(process.id);
    let icon = match process.category {
        "应用" => IconName::AppWindow,
        "后台进程" => IconName::SquareTerminal,
        _ => IconName::Computer,
    };
    div()
        .id(format!("pid-{}", process.id))
        .flex()
        .h(px(shell.density.row_height()))
        .w_full()
        .px(px(10.))
        .items_center()
        .cursor_pointer()
        .border_b_1()
        .border_color(p.border.opacity(0.7))
        .bg(if selected {
            p.accent_soft
        } else {
            p.surface.opacity(0.52)
        })
        .hover(move |s| s.bg(p.hover.opacity(0.82)))
        .text_size(px(11.))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .items_center()
                .gap(px(8.))
                .child(
                    div()
                        .flex()
                        .size(px(23.))
                        .flex_shrink_0()
                        .items_center()
                        .justify_center()
                        .rounded(px(5.))
                        .bg(p.control)
                        .text_color(p.accent)
                        .child(Icon::new(icon).size(px(14.))),
                )
                .child(
                    div()
                        .flex()
                        .min_w_0()
                        .items_baseline()
                        .gap(px(7.))
                        .child(div().text_color(p.text).child(process.name))
                        .child(
                            div()
                                .text_color(p.faint)
                                .text_size(px(10.))
                                .overflow_hidden()
                                .child(process.executable),
                        ),
                ),
        )
        .child(numeric_cell(
            COLUMNS[0].0,
            process.id.to_string(),
            p.muted,
            p,
        ))
        .child(numeric_cell(
            COLUMNS[1].0,
            format!("{:.1}%", process.cpu),
            p.text,
            p,
        ))
        .child(
            numeric_cell(
                COLUMNS[2].0,
                format!("{:.1} MB", process.memory_mb as f32),
                p.text,
                p,
            )
            .bg(if process.memory_mb > 1000 {
                p.disk.opacity(0.16)
            } else {
                p.surface.opacity(0.)
            }),
        )
        .child(numeric_cell(
            COLUMNS[3].0,
            format!("{} KB/s", process.disk_read_kb),
            p.muted,
            p,
        ))
        .child(numeric_cell(
            COLUMNS[4].0,
            format!("{} KB/s", process.disk_write_kb),
            p.muted,
            p,
        ))
        .child(numeric_cell(
            COLUMNS[5].0,
            format!("{} KB/s", process.network_kb),
            p.muted,
            p,
        ))
        .child(numeric_cell(
            COLUMNS[6].0,
            format!("{:.0}%", process.gpu),
            p.muted,
            p,
        ))
        .on_click(cx.listener(move |this, _, _, cx| {
            this.selected_pid = Some(process.id);
            cx.notify();
        }))
        .into_any_element()
}

fn group_header(category: &'static str, rows: &[Process], p: Palette) -> Div {
    let cpu = rows.iter().map(|process| process.cpu).sum::<f32>();
    let memory = rows.iter().map(|process| process.memory_mb).sum::<u32>();
    let read = rows.iter().map(|process| process.disk_read_kb).sum::<u32>();
    let write = rows
        .iter()
        .map(|process| process.disk_write_kb)
        .sum::<u32>();
    div()
        .flex()
        .h(px(33.))
        .w_full()
        .px(px(10.))
        .items_center()
        .border_b_1()
        .border_color(p.border)
        .bg(p.surface.opacity(0.16))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .items_center()
                .gap(px(8.))
                .child(
                    Icon::new(IconName::ChevronDown)
                        .size(px(13.))
                        .text_color(p.muted),
                )
                .child(div().font_weight(FontWeight::MEDIUM).child(category))
                .child(components::tag(rows.len().to_string(), p, true)),
        )
        .child(numeric_cell(COLUMNS[0].0, "", p.faint, p))
        .child(numeric_cell(COLUMNS[1].0, format!("{cpu:.0}%"), p.faint, p))
        .child(numeric_cell(
            COLUMNS[2].0,
            format!("{memory:.2} GB", memory = memory as f32 / 1024.),
            p.faint,
            p,
        ))
        .child(numeric_cell(
            COLUMNS[3].0,
            format!("{read} KB/s"),
            p.faint,
            p,
        ))
        .child(numeric_cell(
            COLUMNS[4].0,
            format!("{write} KB/s"),
            p.faint,
            p,
        ))
        .child(numeric_cell(COLUMNS[5].0, "", p.faint, p))
        .child(numeric_cell(COLUMNS[6].0, "", p.faint, p))
}

fn hardware_item(
    p: Palette,
    icon: IconName,
    label: &'static str,
    title: &'static str,
    detail: &'static str,
    color: Hsla,
) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(11.))
        .py(px(8.))
        .child(
            div()
                .flex()
                .size(px(38.))
                .flex_shrink_0()
                .items_center()
                .justify_center()
                .rounded(px(8.))
                .bg(color.opacity(0.15))
                .text_color(color)
                .child(Icon::new(icon).size(px(17.))),
        )
        .child(
            div()
                .flex()
                .min_w_0()
                .flex_col()
                .gap(px(3.))
                .child(div().text_size(px(10.)).text_color(p.muted).child(label))
                .child(div().text_size(px(12.)).text_color(p.text).child(title))
                .child(div().text_size(px(10.)).text_color(p.faint).child(detail)),
        )
}

fn detail_rail(p: Palette, selected: Option<Process>) -> Div {
    let header = div()
        .flex()
        .h(px(56.))
        .px(px(20.))
        .items_center()
        .justify_between()
        .border_b_1()
        .border_color(p.border)
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(11.))
                .child(
                    Icon::new(IconName::Computer)
                        .size(px(25.))
                        .text_color(p.text),
                )
                .child(components::heading(
                    if selected.is_some() {
                        "进程详情"
                    } else {
                        "本机配置"
                    },
                    p,
                )),
        )
        .child(
            div()
                .flex()
                .gap(px(14.))
                .text_color(p.muted)
                .child(Icon::new(IconName::Copy).size(px(16.)))
                .child(Icon::new(IconName::Eye).size(px(16.))),
        );
    let body = if let Some(process) = selected {
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .p(px(17.))
            .child(components::tag(process.category, p, true))
            .child(
                div()
                    .text_size(px(16.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(p.text)
                    .child(process.name),
            )
            .child(
                div()
                    .text_size(px(11.))
                    .text_color(p.muted)
                    .child(process.executable),
            )
            .child(components::divider(p))
            .child(hardware_item(
                p,
                IconName::SquareTerminal,
                "PID",
                "进程标识",
                "模拟进程",
                p.cpu,
            ))
            .child(hardware_item(
                p,
                IconName::Activity,
                "CPU",
                "处理器占用",
                "采样值随演示数据变化",
                p.memory,
            ))
            .child(hardware_item(
                p,
                IconName::MemoryStick,
                "内存",
                "内存工作集",
                "进程级资源读数",
                p.disk,
            ))
            .child(hardware_item(
                p,
                IconName::HardDrive,
                "I/O",
                "磁盘与网络",
                "读取、写入和发送速率",
                p.network,
            ))
            .child(
                div()
                    .pt(px(8.))
                    .text_size(px(10.))
                    .text_color(p.faint)
                    .child(format!("当前状态：{}", process.state)),
            )
    } else {
        div()
            .flex()
            .flex_1()
            .min_h_0()
            .flex_col()
            .gap(px(5.))
            .p(px(17.))
            .child(
                div()
                    .flex()
                    .gap(px(9.))
                    .p(px(11.))
                    .rounded(px(7.))
                    .border_1()
                    .border_color(p.border)
                    .bg(p.surface.opacity(0.32))
                    .child(Icon::new(IconName::Info).size(px(15.)).text_color(p.accent))
                    .child(components::small_label(
                        "选择进程行可查看对应的统计信息。此视图仅使用本地模拟数据。",
                        p,
                    )),
            )
            .child(hardware_item(
                p,
                IconName::Cpu,
                "处理器",
                "Intel Core i5-13500HX",
                "14 核（6 性能核 + 8 能效核）· 20 线程 · 2.50 GHz",
                p.cpu,
            ))
            .child(hardware_item(
                p,
                IconName::MemoryStick,
                "内存",
                "32 GB DDR5",
                "5600 MT/s · 已用插槽 2 / 2 · SODIMM",
                p.memory,
            ))
            .child(hardware_item(
                p,
                IconName::HardDrive,
                "存储",
                "Samsung SSD 990 PRO 2TB",
                "1.8 TB · SSD (NVMe) · C:",
                p.disk,
            ))
            .child(hardware_item(
                p,
                IconName::HardDrive,
                "存储",
                "WDC WD40EZAZ-00SF3B0",
                "3.6 TB · HDD (SATA) · D: E:",
                p.disk,
            ))
            .child(hardware_item(
                p,
                IconName::Network,
                "Wi-Fi",
                "Intel(R) Wi-Fi 6E AX211 160MHz",
                "WLAN · 1.2 Gbps",
                p.network,
            ))
            .child(hardware_item(
                p,
                IconName::Monitor,
                "显卡",
                "NVIDIA GeForce RTX",
                "8 GB · 独立显存",
                p.gpu,
            ))
    };
    div()
        .flex()
        .w(px(420.))
        .max_w(px(460.))
        .min_w(px(340.))
        .h_full()
        .flex_shrink_0()
        .flex_col()
        .border_l_1()
        .border_color(p.border)
        .bg(p.surface.opacity(0.28))
        .child(header)
        .child(div().flex_1().min_h_0().overflow_y_scrollbar().child(body))
}

pub fn render(shell: &mut Shell, cx: &mut Context<Shell>) -> AnyElement {
    let p = shell.palette();
    let rows = filtered_sorted(&shell.query, shell.sort);
    let selected = rows
        .iter()
        .find(|row| Some(row.id) == shell.selected_pid)
        .copied();
    let count = rows.len();
    let totals = (
        format!("{:.0}%", rows.iter().map(|row| row.cpu).sum::<f32>()),
        format!(
            "{:.1} GB",
            rows.iter().map(|row| row.memory_mb).sum::<u32>() as f32 / 1024.
        ),
        format!(
            "{} KB/s",
            rows.iter().map(|row| row.disk_read_kb).sum::<u32>()
        ),
        format!(
            "{} KB/s",
            rows.iter().map(|row| row.disk_write_kb).sum::<u32>()
        ),
        format!(
            "{} KB/s",
            rows.iter().map(|row| row.network_kb).sum::<u32>()
        ),
        format!("{:.0}%", rows.iter().map(|row| row.gpu).sum::<f32>()),
    );
    let mut table_rows = Vec::new();
    for category in ["应用", "后台进程", "Windows 进程"] {
        let mut group: Vec<_> = rows
            .iter()
            .copied()
            .filter(|row| row.category == category)
            .collect();
        if !group.is_empty() {
            group.sort_by(|a, b| match shell.sort {
                SortBy::Cpu => b.cpu.total_cmp(&a.cpu),
                SortBy::Memory => b.memory_mb.cmp(&a.memory_mb),
                SortBy::Name => a.name.cmp(b.name),
            });
            table_rows.push(group_header(category, &group, p).into_any_element());
            table_rows.extend(
                group
                    .into_iter()
                    .map(|process| process_row(shell, process, cx)),
            );
        }
    }
    let visible_rows = PROCESSES
        .iter()
        .filter(|process| shell.query.is_empty() || rows.iter().any(|row| row.id == process.id))
        .count();

    let list = div()
        .flex()
        .flex_1()
        .min_w_0()
        .h_full()
        .flex_col()
        .child(
            div()
                .flex()
                .h(px(56.))
                .px(px(14.))
                .items_center()
                .justify_between()
                .border_b_1()
                .border_color(p.border)
                .child(
                    div()
                        .text_size(px(15.))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(p.text)
                        .child(format!("{visible_rows} 个进程")),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(14.))
                        .text_size(px(10.))
                        .text_color(p.muted)
                        .child(format!("{} 条记录", count))
                        .child(
                            div()
                                .id("sort-toggle")
                                .cursor_pointer()
                                .child(components::tag(
                                    format!("排序：{}", shell.sort.title()),
                                    p,
                                    false,
                                ))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.sort = this.sort.next();
                                    cx.notify();
                                })),
                        )
                        .child(components::tag("模拟数据", p, false)),
                ),
        )
        .child(table_header(p, totals))
        .child(
            div()
                .flex_1()
                .min_h_0()
                .overflow_y_scrollbar()
                .children(table_rows),
        );

    components::panel(p)
        .flex()
        .size_full()
        .overflow_hidden()
        .child(list)
        .when(shell.show_details, |d| d.child(detail_rail(p, selected)))
        .into_any_element()
}
