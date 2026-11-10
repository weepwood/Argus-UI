//! Application shell: global navigation and a small typed state machine.
//! Domain views live in pages/; generic primitives live in components/.
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    *,
    assets::IconName,
    component::{Icon, Theme, ThemeMode, input::{InputEvent, InputState}},
};
use crate::{
    components::{self, ButtonTone},
    data::SortBy,
    design::{Appearance, Density, Palette},
    pages,
};
use gpui_kit::component::scroll::ScrollableElement as _;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Page {
    Overview,
    Processes,
    Components,
    Settings,
}
impl Page {
    pub const ALL: [Self; 4] = [Self::Overview, Self::Processes, Self::Components, Self::Settings];
    pub fn id(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Processes => "processes",
            Self::Components => "components",
            Self::Settings => "settings",
        }
    }
    pub fn title(self) -> &'static str {
        match self {
            Self::Overview => "概览",
            Self::Processes => "进程列表",
            Self::Components => "组件展示",
            Self::Settings => "外观设置",
        }
    }
    pub fn subtitle(self) -> &'static str {
        match self {
            Self::Overview => "桌面组件框架与实时数据视觉演示",
            Self::Processes => "可搜索、可排序的演示数据表格",
            Self::Components => "可复用控件与设计语言",
            Self::Settings => "主题、密度与窗口布局选项",
        }
    }
    fn icon(self) -> IconName {
        match self {
            Self::Overview => IconName::LayoutDashboard,
            Self::Processes => IconName::Inbox,
            Self::Components => IconName::GalleryVerticalEnd,
            Self::Settings => IconName::Settings,
        }
    }
}
pub struct Shell {
    pub(crate) page: Page,
    pub(crate) appearance: Appearance,
    pub(crate) density: Density,
    pub(crate) sidebar_compact: bool,
    pub(crate) sort: SortBy,
    pub(crate) query: String,
    pub(crate) search: Entity<InputState>,
    pub(crate) selected_pid: Option<u32>,
    pub(crate) demo_clicks: u32,
    pub(crate) animations: bool,
    subscriptions: Vec<Subscription>,
}
impl Shell {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search = cx.new(|cx| {
            InputState::new(window, cx).placeholder("搜索名称、类别或 PID…")
        });
        let reference = search.clone();
        let subscriptions = vec![cx.subscribe_in(
            &search, window,
            move |this, _, event: &InputEvent, _window, cx| {
                if matches!(event, InputEvent::Change) {
                    this.query = reference.read(cx).value().to_string();
                    cx.notify();
                }
            },
        )];
        Self {
            page: Page::Overview,
            appearance: Appearance::Light,
            density: Density::Comfortable,
            sidebar_compact: false,
            sort: SortBy::Cpu,
            query: String::new(),
            search,
            selected_pid: None,
            demo_clicks: 0,
            animations: true,
            subscriptions,
        }
    }
    pub(crate) fn palette(&self) -> Palette { Palette::new(self.appearance) }
    pub(crate) fn set_theme(&mut self, mode: Appearance, window: &mut Window, cx: &mut Context<Self>) {
        self.appearance = mode;
        let theme_mode = match mode {
            Appearance::Light => ThemeMode::Light,
            Appearance::Dark => ThemeMode::Dark,
        };
        Theme::change(theme_mode, Some(window), cx);
        cx.notify();
    }
    pub(crate) fn navigate(&mut self, page: Page, cx: &mut Context<Self>) {
        if self.page != page {
            self.page = page;
            cx.notify();
        }
    }
    fn nav_item(&self, page: Page, cx: &mut Context<Self>) -> AnyElement {
        let p = self.palette();
        let selected = self.page == page;
        let color = if selected { p.accent } else { p.muted };
        div()
            .id(format!("nav-{}", page.id()))
            .h(px(40.))
            .px(px(11.))
            .flex()
            .items_center()
            .gap(px(11.))
            .rounded(px(6.))
            .cursor_pointer()
            .bg(if selected { p.accent_soft } else { p.background })
            .text_color(color)
            .hover(move |s| s.bg(p.hover))
            .child(Icon::new(page.icon()).size(px(17.)))
            .when(!self.sidebar_compact, |d| {
                d.child(div().text_size(px(13.)).font_weight(if selected {
                    FontWeight::MEDIUM
                } else {FontWeight::NORMAL}).child(page.title()))
            })
            .on_click(cx.listener(move |this, _, _, cx| this.navigate(page,cx)))
            .into_any_element()
    }
    fn sidebar(&self, cx: &mut Context<Self>) -> AnyElement {
        let p = self.palette();
        let compact = self.sidebar_compact;
        let nav = Page::ALL.into_iter().map(|page| self.nav_item(page,cx)).collect::<Vec<_>>();
        div().flex().flex_col()
            .h_full()
            .w(px(if compact {68.} else {218.}))
            .flex_shrink_0()
            .border_r_1()
            .border_color(p.border)
            .bg(p.background)
            .child(
                div().flex().h(px(72.)).px(px(17.)).items_center().gap(px(11.))
                .child(
                    div().size(px(34.)).rounded(px(8.))
                        .bg(p.accent).text_color(p.accent_ink)
                        .flex().items_center().justify_center()
                        .font_weight(FontWeight::BOLD).text_size(px(17.)).child("A")
                )
                .when(!compact, |d| d.child(
                    div().flex().flex_col().gap(px(1.))
                        .child(div().text_size(px(15.)).font_weight(FontWeight::SEMIBOLD).text_color(p.text).child("Argus UI"))
                        .child(div().text_size(px(10.)).text_color(p.faint).child("NATIVE STARTER"))
                ))
            )
            .child(
                div().flex().flex_col().px(px(12.)).gap(px(4.))
                .when(!compact,|d|d.child(
                    div().px(px(10.)).pb(px(8.)).text_size(px(10.))
                      .text_color(p.faint).child("工作空间")
                ))
                .children(nav)
            )
            .child(div().flex_1())
            .child(
                div().flex().flex_col().p(px(12.)).gap(px(11.))
                    .child(components::divider(p))
                    .child(
                        div().flex().items_center().gap(px(9.))
                        .child(div().size(px(31.)).rounded(px(16.)).bg(p.control)
                            .flex().items_center().justify_center().text_color(p.muted).child("W"))
                        .when(!compact,|d|d.child(div().flex().flex_col().gap(px(2.))
                            .child(div().text_size(px(12.)).font_weight(FontWeight::MEDIUM).text_color(p.text).child("Workspace"))
                            .child(div().text_size(px(10.)).text_color(p.faint).child("Local demo"))))
                    )
            )
            .into_any_element()
    }
    fn header(&self, cx: &mut Context<Self>) -> AnyElement {
        let p=self.palette();
        let page=self.page;
        div().flex()
            .h(px(73.)).px(px(26.)).flex_shrink_0().items_center()
            .justify_between().border_b_1().border_color(p.border)
            .bg(p.surface)
            .child(
                div().flex().gap(px(14.)).items_center()
                .child(
                    components::button("sidebar-toggle",if self.sidebar_compact {"展开"} else {"收起"},p,ButtonTone::Ghost)
                       .on_click(cx.listener(|this,_,_,cx| {this.sidebar_compact=!this.sidebar_compact;cx.notify();}))
                )
                .child(div().flex().flex_col().gap(px(4.))
                    .child(div().text_size(px(19.)).font_weight(FontWeight::SEMIBOLD).text_color(p.text).child(page.title()))
                    .child(components::small_label(page.subtitle(),p)))
            )
            .child(
                div().flex().items_center().gap(px(10.))
                    .child(components::tag("DEMO DATA",p,false))
                    .child(components::button("header-theme",
                        if self.appearance==Appearance::Light {"深色模式"} else {"浅色模式"},p,ButtonTone::Normal)
                        .on_click(cx.listener(|this,_,window,cx|{
                            let next=if this.appearance==Appearance::Light {Appearance::Dark} else {Appearance::Light};
                            this.set_theme(next,window,cx);
                        })))
            )
            .into_any_element()
    }
}
impl Render for Shell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p=self.palette();
        let _keep_subscriptions_alive=&self.subscriptions;
        let sidebar=self.sidebar(cx);
        let header=self.header(cx);
        let content=match self.page {
            Page::Overview => pages::overview::render(self,window,cx),
            Page::Processes => pages::processes::render(self,window,cx),
            Page::Components => pages::showcase::render(self,window,cx),
            Page::Settings => pages::settings::render(self,window,cx),
        };
        div().flex().size_full().bg(p.background)
            .text_color(p.text)
            .font_family("Segoe UI Variable")
            .text_size(px(13.))
            .child(sidebar)
            .child(
                div().flex().flex_col().flex_1().min_w_0().h_full()
                    .child(header)
                    .child(
                        div().flex_1().min_h_0().overflow_y_scrollbar()
                            .child(content)
                    )
                    .child(
                        div().flex().h(px(32.)).px(px(22.)).flex_shrink_0().items_center()
                            .justify_between().border_t_1().border_color(p.border)
                            .bg(p.surface).text_size(px(11.)).text_color(p.faint)
                            .child("Argus UI  /  Rust + GPUI Kit")
                            .child(format!("{}  ·  {}布局  ·  本地演示",self.appearance.name(),self.density.name()))
                    )
            )
    }
}
