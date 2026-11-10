//! Shared dense-table template for secondary Argus-style pages.
use crate::{
    app::{Page, Shell},
    components,
    design::Palette,
};
use gpui_kit::{
    assets::IconName,
    component::{Icon, scroll::ScrollableElement as _},
    *,
};

type Record = [&'static str; 5];

fn records(
    page: Page,
) -> (
    &'static str,
    &'static str,
    &'static [&'static str],
    &'static [Record],
) {
    match page {
        Page::Services => (
            "服务",
            "显示本地服务的状态、启动类型和服务账户。",
            &["名称", "状态", "启动类型", "进程", "描述"],
            &[
                [
                    "Windows Update",
                    "运行中",
                    "手动",
                    "svchost.exe",
                    "检测、下载和安装 Windows 更新",
                ],
                [
                    "Windows Search",
                    "运行中",
                    "自动",
                    "SearchHost.exe",
                    "为文件、邮件和其他内容建立索引",
                ],
                [
                    "Print Spooler",
                    "已停止",
                    "自动",
                    "spoolsv.exe",
                    "将文件加载到内存中以供稍后打印",
                ],
                [
                    "Bluetooth Support",
                    "运行中",
                    "手动",
                    "svchost.exe",
                    "支持蓝牙设备发现和关联",
                ],
                [
                    "Workstation",
                    "运行中",
                    "自动",
                    "svchost.exe",
                    "创建并维护到远程服务器的连接",
                ],
            ],
        ),
        Page::Startup => (
            "启动应用",
            "管理登录时启动的应用和快捷方式。",
            &["名称", "状态", "启动影响", "发布者", "启动位置"],
            &[
                ["Argus UI", "已启用", "低", "Argus UI", "当前用户"],
                [
                    "OneDrive",
                    "已启用",
                    "中",
                    "Microsoft Corporation",
                    "注册表",
                ],
                [
                    "Windows Security notification",
                    "已启用",
                    "低",
                    "Microsoft Corporation",
                    "系统启动",
                ],
                ["Teams", "已禁用", "高", "Microsoft Corporation", "当前用户"],
                ["Adobe Updater", "已禁用", "低", "Adobe Inc.", "注册表"],
            ],
        ),
        Page::Connections => (
            "连接与端口",
            "查看应用和服务正在使用的网络端点。",
            &["进程", "协议", "本地地址", "远程地址", "状态"],
            &[
                [
                    "msedge.exe",
                    "TCP",
                    "192.168.1.18:51182",
                    "142.250.72.14:443",
                    "已建立",
                ],
                [
                    "Code.exe",
                    "TCP",
                    "192.168.1.18:51203",
                    "140.82.112.5:443",
                    "已建立",
                ],
                [
                    "OneDrive.exe",
                    "TCP",
                    "192.168.1.18:51310",
                    "20.190.151.130:443",
                    "已建立",
                ],
                [
                    "svchost.exe",
                    "UDP",
                    "0.0.0.0:5353",
                    "224.0.0.251:5353",
                    "侦听",
                ],
                ["System", "TCP", "0.0.0.0:135", "0.0.0.0:0", "侦听"],
            ],
        ),
        Page::History => (
            "资源历史记录",
            "按时间回看 CPU、内存、磁盘、网络与 GPU 的采样概况。",
            &["时间", "CPU", "内存", "磁盘", "网络"],
            &[
                ["16:42:00", "41%", "39%", "7.1 MB/s", "22.7 MB/s"],
                ["16:41:00", "38%", "39%", "1.4 MB/s", "18.2 MB/s"],
                ["16:40:00", "52%", "40%", "4.8 MB/s", "31.5 MB/s"],
                ["16:39:00", "33%", "38%", "0.8 MB/s", "12.6 MB/s"],
                ["16:38:00", "29%", "38%", "2.2 MB/s", "9.4 MB/s"],
            ],
        ),
        _ => ("", "", &[], &[]),
    }
}

fn details(p: Palette, title: &'static str, text: &'static str) -> Div {
    div()
        .w(px(360.))
        .h_full()
        .flex_shrink_0()
        .flex_col()
        .border_l_1()
        .border_color(p.border)
        .px(px(19.))
        .py(px(17.))
        .gap(px(12.))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(10.))
                .child(
                    Icon::new(IconName::PanelRight)
                        .size(px(20.))
                        .text_color(p.muted),
                )
                .child(components::heading("详细信息", p)),
        )
        .child(components::divider(p))
        .child(components::small_label(title, p))
        .child(div().text_size(px(13.)).text_color(p.text).child(text))
        .child(div().flex_1())
        .child(components::tag("本地模拟数据", p, false))
}

pub fn render(shell: &mut Shell, _cx: &mut Context<Shell>) -> AnyElement {
    let p = shell.palette();
    let (title, subtitle, columns, items) = records(shell.page);
    let table_rows = items.iter().map(|row| {
        div()
            .flex()
            .h(px(43.))
            .px(px(13.))
            .items_center()
            .border_b_1()
            .border_color(p.border.opacity(0.68))
            .hover(move |s| s.bg(p.hover.opacity(0.8)))
            .children(row.iter().enumerate().map(|(index, value)| {
                div()
                    .flex_1()
                    .min_w_0()
                    .text_size(px(11.))
                    .text_color(if index == 0 { p.text } else { p.muted })
                    .child(*value)
                    .into_any_element()
            }))
            .into_any_element()
    });
    components::panel(p)
        .flex()
        .size_full()
        .overflow_hidden()
        .child(
            div()
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
                                .flex()
                                .flex_col()
                                .gap(px(3.))
                                .child(
                                    div()
                                        .text_size(px(15.))
                                        .font_weight(FontWeight::MEDIUM)
                                        .child(title),
                                )
                                .child(components::small_label(subtitle, p)),
                        )
                        .child(components::tag(format!("{} 项", items.len()), p, true)),
                )
                .child(
                    div()
                        .flex()
                        .h(px(36.))
                        .px(px(13.))
                        .items_center()
                        .border_b_1()
                        .border_color(p.border)
                        .bg(p.surface.opacity(0.25))
                        .children(columns.iter().map(|column| {
                            div()
                                .flex_1()
                                .text_size(px(10.))
                                .text_color(p.muted)
                                .child(*column)
                                .into_any_element()
                        })),
                )
                .child(
                    div()
                        .flex_1()
                        .min_h_0()
                        .overflow_y_scrollbar()
                        .children(table_rows),
                ),
        )
        .child(details(p, title, subtitle))
        .into_any_element()
}
