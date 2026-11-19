//! Native shell: command bar, Argus-style page tabs, resource strip, and content.
use crate::{
    backdrop::Backdrop,
    components,
    data::{CPU_SERIES, DISK_SERIES, GPU_SERIES, MEMORY_SERIES, NET_SERIES, SortBy},
    design::{Appearance, Density, MaterialClarity, Palette},
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
    pub(crate) backdrop: Backdrop,
    pub(crate) clarity: MaterialClarity,
    pub(crate) density: Density,
    pub(crate) view: ProcessView,
    pub(crate) sort: SortBy,
    pub(crate) query: String,
    pub(crate) search: Entity<InputState>,
    pub(crate) selected_pid: Option<u32>,
    pub(crate) demo_clicks: u32,
    pub(crate) animations: bool,
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
            backdrop: Backdrop::auto(),
            clarity: MaterialClarity::Crisp,
            density: Density::Comfortable,
            view: ProcessView::Apps,
            sort: SortBy::Cpu,
            query: String::new(),
            search,
            selected_pid: None,
            demo_clicks: 0,
            animations: true,
            show_details: true,
            subscriptions,
        }
    }

    /// `Sharp` trades the DWM material for an opaque window so GPUI can use
    /// ClearType; every other mode keeps the material the user picked.
    pub(crate) fn effective_backdrop(&self) -> Backdrop {
        if self.clarity == MaterialClarity::Sharp {
            Backdrop::Opaque
        } else {
            self.backdrop
        }
    }

    pub(crate) fn palette(&self) -> Palette {
        Palette::new(self.appearance, self.effective_backdrop(), self.clarity)
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

    /// Change how much material is allowed behind text. `Sharp` drops the
    /// backdrop entirely: GPUI only enables ClearType on opaque windows.
    pub(crate) fn set_clarity(
        &mut self,
        clarity: MaterialClarity,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.clarity == clarity {
            return;
        }
        self.clarity = clarity;
        window.set_background_appearance(self.effective_backdrop().appearance());
        cx.set_text_rendering_mode(if clarity == MaterialClarity::Sharp {
            TextRenderingMode::Subpixel
        } else {
            TextRenderingMode::PlatformDefault
        });
        cx.notify();
    }

    /// Switch the DWM material at runtime and repaint the translucent layers.
    pub(crate) fn set_backdrop(
        &mut self,
        mode: Backdrop,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.backdrop == mode || !mode.is_available() {
            return;
        }
        self.backdrop = mode;
        // Picking a material explicitly leaves the ClearType-only mode.
        if self.clarity == MaterialClarity::Sharp {
            self.clarity = MaterialClarity::Crisp;
            cx.set_text_rendering_mode(TextRenderingMode::PlatformDefault);
        }
        window.set_background_appearance(self.effective_backdrop().appearance());
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
            this.page = Page::Processes;
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
            .bg(p.text_plate(p.surface, 0.58))
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
                    .bg(p.text_plate(p.surface, 0.78))
                    .text_color(p.muted)
                    .child(Icon::new(IconName::Search).ml(px(10.)).size(px(16.)))
                    .child(search),
            )
            .child(div().flex_1())
            .child(components::tag("DEMO · 本地模拟", p, false))
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
            .bg(p.text_plate(p.surface, 0.4))
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
            .bg(p.text_plate(p.surface, 0.52))
            .text_size(px(10.))
            .text_color(p.faint)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(14.))
                    .child(components::tag("UI 0.1 · Demo", p, false))
                    .child("全部指标为静态演示数据")
                    .child("不读取系统进程或资源")
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(13.))
                    .child(components::tag("不自动刷新", p, false))
                    .child(components::tag(self.effective_backdrop().title(), p, true))
                    .child(components::tag("只读演示", p, false)),
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
            .bg(p.background)
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
