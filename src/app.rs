//! Native shell: command bar, Argus-style page tabs, resource strip, and content.
use crate::{
    components,
    data::{CPU_SERIES, DISK_SERIES, GPU_SERIES, MEMORY_SERIES, NET_SERIES, SortBy},
    design::{Appearance, Density, Palette},
    pages,
};
use gpui_kit::{
    assets::IconName,
    component::scroll::ScrollableElement as _,
    component::{
        Icon, Theme, ThemeMode,
        input::{Input, InputEvent, InputState},
    },
    *,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Page {
    Processes,
    Performance,
    Services,
    Startup,
    Connections,
    History,
    Settings,
    Components,
}

impl Page {
    pub const ALL: [Self; 8] = [
        Self::Processes,
        Self::Performance,
        Self::Services,
        Self::Startup,
        Self::Connections,
        Self::History,
        Self::Settings,
        Self::Components,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Self::Processes => "processes",
            Self::Performance => "performance",
            Self::Services => "services",
            Self::Startup => "startup",
            Self::Connections => "connections",
            Self::History => "history",
            Self::Settings => "settings",
            Self::Components => "components",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Processes => "进程",
            Self::Performance => "性能",
            Self::Services => "服务",
            Self::Startup => "启动",
            Self::Connections => "连接",
            Self::History => "历史",
            Self::Settings => "设置",
            Self::Components => "组件库",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessView {
    Apps,
    Tree,
    List,
}

impl ProcessView {
    fn title(self) -> &'static str {
        match self {
            Self::Apps => "按应用",
            Self::Tree => "进程树",
            Self::List => "列表",
        }
    }
}

pub struct Shell {
    pub(crate) page: Page,
    pub(crate) appearance: Appearance,
    pub(crate) density: Density,
    pub(crate) view: ProcessView,
    pub(crate) sort: SortBy,
    pub(crate) query: String,
    pub(crate) search: Entity<InputState>,
    pub(crate) selected_pid: Option<u32>,
    pub(crate) demo_clicks: u32,
    pub(crate) animations: bool,
    pub(crate) refresh_paused: bool,
    pub(crate) locked: bool,
    pub(crate) show_details: bool,
    subscriptions: Vec<Subscription>,
}

impl Shell {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search =
            cx.new(|cx| InputState::new(window, cx).placeholder("搜索名称、PID、路径或服务"));
        let reference = search.clone();
        let subscriptions = vec![cx.subscribe_in(
            &search,
            window,
            move |this, _, event: &InputEvent, _window, cx| {
                if matches!(event, InputEvent::Change) {
                    this.query = reference.read(cx).value().to_string();
                    cx.notify();
                }
            },
        )];
        Self {
            page: Page::Processes,
            appearance: Appearance::Light,
            density: Density::Comfortable,
            view: ProcessView::Apps,
            sort: SortBy::Cpu,
            query: String::new(),
            search,
            selected_pid: None,
            demo_clicks: 0,
            animations: true,
            refresh_paused: false,
            locked: false,
            show_details: true,
            subscriptions,
        }
    }

    pub(crate) fn palette(&self) -> Palette {
        Palette::new(self.appearance)
    }

    pub(crate) fn set_theme(
        &mut self,
        mode: Appearance,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.appearance = mode;
        Theme::change(
            match mode {
                Appearance::Light => ThemeMode::Light,
                Appearance::Dark => ThemeMode::Dark,
            },
            Some(window),
            cx,
        );
        cx.notify();
    }

    pub(crate) fn navigate(&mut self, page: Page, cx: &mut Context<Self>) {
        if self.page != page {
            self.page = page;
            cx.notify();
        }
    }

    fn view_button(&self, view: ProcessView, p: Palette, cx: &mut Context<Self>) -> AnyElement {
        let selected = self.view == view;
        components::toolbar_button(
            match view {
                ProcessView::Apps => "view-apps",
                ProcessView::Tree => "view-tree",
                ProcessView::List => "view-list",
            },
            match view {
                ProcessView::Apps => IconName::Grid2x2,
                ProcessView::Tree => IconName::ListTree,
                ProcessView::List => IconName::Menu,
            },
            view.title(),
            p,
            selected,
        )
        .on_click(cx.listener(move |this, _, _, cx| {
            this.view = view;
            cx.notify();
        }))
        .into_any_element()
    }

    fn toolbar(&self, cx: &mut Context<Self>) -> AnyElement {
        let p = self.palette();
        let search = Input::new(&self.search)
            .appearance(false)
            .w_full()
            .h_full()
            .px(px(9.))
            .text_size(px(12.));
        div()
            .flex()
            .h(px(58.))
            .w_full()
            .px(px(14.))
            .flex_shrink_0()
            .items_center()
            .gap(px(10.))
            .border_b_1()
            .border_color(p.border)
            .bg(p.surface.opacity(0.58))
            .child(
                div()
                    .flex()
                    .w(px(75.))
                    .flex_shrink_0()
                    .items_center()
                    .gap(px(7.))
                    .child(
                        div()
                            .flex()
                            .size(px(28.))
                            .items_center()
                            .justify_center()
                            .rounded(px(6.))
                            .bg(rgb(0x172c52))
                            .text_color(rgb(0x63d8ff))
                            .child(Icon::new(IconName::Eye).size(px(17.))),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .font_weight(FontWeight::MEDIUM)
                            .child("Argus"),
                    ),
            )
            .child(div().flex().flex_shrink_0().gap(px(3.)).children([
                self.view_button(ProcessView::Apps, p, cx),
                self.view_button(ProcessView::Tree, p, cx),
                self.view_button(ProcessView::List, p, cx),
            ]))
            .child(div().flex_1())
            .child(
                div()
                    .flex()
                    .h(px(36.))
                    .max_w(px(450.))
                    .min_w(px(190.))
                    .flex_1()
                    .items_center()
                    .gap(px(5.))
                    .overflow_hidden()
                    .rounded(px(6.))
                    .border_1()
                    .border_color(p.border)
                    .bg(p.surface.opacity(0.78))
                    .text_color(p.muted)
                    .child(Icon::new(IconName::Search).ml(px(10.)).size(px(16.)))
                    .child(search),
            )
            .child(div().flex_1())
            .child(
                components::toolbar_button(
                    "new-task",
                    IconName::SquareTerminal,
                    "运行新任务",
                    p,
                    false,
                )
                .on_click(cx.listener(|this, _, _, cx| {
                    this.demo_clicks += 1;
                    cx.notify();
                })),
            )
            .child(
                components::toolbar_button(
                    "refresh-rate",
                    IconName::ChevronDown,
                    "每 1 秒",
                    p,
                    false,
                )
                .on_click(cx.listener(|this, _, _, cx| {
                    this.refresh_paused = !this.refresh_paused;
                    cx.notify();
                })),
            )
            .child(
                components::toolbar_button(
                    "refresh-toggle",
                    if self.refresh_paused {
                        IconName::Play
                    } else {
                        IconName::Pause
                    },
                    "",
                    p,
                    self.refresh_paused,
                )
                .w(px(36.))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.refresh_paused = !this.refresh_paused;
                    cx.notify();
                })),
            )
            .child(
                components::toolbar_button(
                    "elevation-lock",
                    if self.locked {
                        IconName::LockKeyhole
                    } else {
                        IconName::LockKeyholeOpen
                    },
                    "",
                    p,
                    self.locked,
                )
                .w(px(36.))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.locked = !this.locked;
                    cx.notify();
                })),
            )
            .child(
                components::toolbar_button(
                    "details-toggle",
                    IconName::PanelRight,
                    "",
                    p,
                    self.show_details,
                )
                .w(px(36.))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.show_details = !this.show_details;
                    cx.notify();
                })),
            )
            .child(
                components::toolbar_button(
                    "theme-toggle",
                    if self.appearance == Appearance::Light {
                        IconName::Moon
                    } else {
                        IconName::Sun
                    },
                    "",
                    p,
                    false,
                )
                .w(px(36.))
                .on_click(cx.listener(|this, _, window, cx| {
                    let next = if this.appearance == Appearance::Light {
                        Appearance::Dark
                    } else {
                        Appearance::Light
                    };
                    this.set_theme(next, window, cx);
                })),
            )
            .into_any_element()
    }

    fn page_tabs(&self, cx: &mut Context<Self>) -> AnyElement {
        let p = self.palette();
        div()
            .flex()
            .h(px(40.))
            .w_full()
            .px(px(18.))
            .flex_shrink_0()
            .items_stretch()
            .gap(px(4.))
            .border_b_1()
            .border_color(p.border)
            .bg(p.surface.opacity(0.4))
            .children(Page::ALL.into_iter().map(|page| {
                let selected = self.page == page;
                div()
                    .id(format!("tab-{}", page.id()))
                    .min_w(px(62.))
                    .px(px(10.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .border_b_2()
                    .border_color(if selected {
                        p.accent
                    } else {
                        p.surface.opacity(0.)
                    })
                    .text_size(px(12.))
                    .text_color(if selected { p.text } else { p.muted })
                    .cursor_pointer()
                    .hover(move |s| s.bg(p.hover.opacity(0.55)))
                    .child(page.title())
                    .on_click(cx.listener(move |this, _, _, cx| this.navigate(page, cx)))
                    .into_any_element()
            }))
            .into_any_element()
    }

    fn resource_strip(&self) -> AnyElement {
        let p = self.palette();
        div()
            .flex()
            .h(px(160.))
            .w_full()
            .px(px(14.))
            .py(px(8.))
            .flex_shrink_0()
            .bg(p.background.opacity(0.45))
            .child(
                div()
                    .flex()
                    .w_full()
                    .h_full()
                    .overflow_hidden()
                    .rounded(px(8.))
                    .border_1()
                    .border_color(p.border)
                    .children([
                        components::ResourceMetric {
                            title: "CPU",
                            ceiling: "100%",
                            value: "41",
                            unit: "%",
                            note: "3.94 GHz  ·  75 °C  ·  88 W",
                            color: p.cpu,
                            values: CPU_SERIES,
                            border_right: true,
                        }
                        .render(p),
                        components::ResourceMetric {
                            title: "内存",
                            ceiling: "32.0 GB",
                            value: "12.3",
                            unit: "/ 32.0 GB",
                            note: "39% 已用",
                            color: p.memory,
                            values: MEMORY_SERIES,
                            border_right: true,
                        }
                        .render(p),
                        components::ResourceMetric {
                            title: "磁盘",
                            ceiling: "100 MB/s",
                            value: "7.1",
                            unit: "MB/s",
                            note: "活动 6%",
                            color: p.disk,
                            values: DISK_SERIES,
                            border_right: true,
                        }
                        .render(p),
                        components::ResourceMetric {
                            title: "网络",
                            ceiling: "50 MB/s",
                            value: "22.7",
                            unit: "MB/s",
                            note: "收 11.5 MB/s  发 11.2 MB/s",
                            color: p.network,
                            values: NET_SERIES,
                            border_right: true,
                        }
                        .render(p),
                        components::ResourceMetric {
                            title: "GPU",
                            ceiling: "100%",
                            value: "42",
                            unit: "%",
                            note: "52 °C  ·  26 W",
                            color: p.gpu,
                            values: GPU_SERIES,
                            border_right: false,
                        }
                        .render(p),
                    ]),
            )
            .into_any_element()
    }

    fn status_bar(&self) -> AnyElement {
        let p = self.palette();
        div()
            .flex()
            .h(px(31.))
            .w_full()
            .px(px(14.))
            .flex_shrink_0()
            .items_center()
            .justify_between()
            .border_t_1()
            .border_color(p.border)
            .bg(p.surface.opacity(0.52))
            .text_size(px(10.))
            .text_color(p.faint)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(14.))
                    .child(components::tag("UI 0.1", p, false))
                    .child("线程 2,536")
                    .child("句柄 101,267")
                    .child("缓存 7.7 GB")
                    .child("已运行 5 天 14 小时"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(13.))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(5.))
                            .child(div().size(px(6.)).rounded(px(3.)).bg(p.network))
                            .child(if self.refresh_paused {
                                "刷新已暂停"
                            } else {
                                "每 1 秒刷新"
                            }),
                    )
                    .child(components::tag("标准权限", p, false)),
            )
            .into_any_element()
    }
}

impl Render for Shell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette();
        let _keep_subscriptions_alive = &self.subscriptions;
        let content = match self.page {
            Page::Processes => pages::processes::render(self, cx),
            Page::Performance => pages::overview::render(self, window, cx),
            Page::Services | Page::Startup | Page::Connections | Page::History => {
                pages::catalog::render(self, cx)
            }
            Page::Settings => pages::settings::render(self, window, cx),
            Page::Components => pages::showcase::render(self, cx),
        };
        div()
            .flex()
            .flex_col()
            .size_full()
            .font_family("Segoe UI Variable")
            .text_size(px(12.))
            .text_color(p.text)
            .bg(p.background.opacity(0.16))
            .child(self.toolbar(cx))
            .child(self.page_tabs(cx))
            .child(self.resource_strip())
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .px(px(14.))
                    .pb(px(8.))
                    .overflow_y_scrollbar()
                    .child(content),
            )
            .child(self.status_bar())
    }
}
