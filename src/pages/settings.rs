//! Appearance preferences are live state; persistence can be added via a service.
use gpui_kit::*;
use crate::{
    app::Shell,
    components::{self, ButtonTone},
    design::{Appearance, Density},
};

pub fn render(shell:&mut Shell, _: &mut Window,cx:&mut Context<Shell>) -> AnyElement {
    let p=shell.palette();
    v_flex().w_full().p(px(26.)).gap(px(19.))
        .child(v_flex().gap(px(6.))
            .child(components::heading("外观设置",p))
            .child(components::small_label("修改后立即生效；当前版本不持久化这些偏好",p)))
        .child(
            components::panel(p).p(px(21.))
                .child(div().text_size(px(14.)).font_weight(FontWeight::SEMIBOLD).child("主题"))
                .child(div().h(px(7.)))
                .child(components::small_label("统一切换背景、边框、文字、状态色和组件底色",p))
                .child(div().h(px(16.)))
                .child(
                    h_flex().gap(px(9.))
                        .child(components::button("light-mode","浅色 Light",p,
                            if shell.appearance==Appearance::Light {ButtonTone::Primary} else {ButtonTone::Normal})
                            .on_click(cx.listener(|this,_,window,cx|this.set_theme(Appearance::Light,window,cx))))
                        .child(components::button("dark-mode","深色 Dark",p,
                            if shell.appearance==Appearance::Dark {ButtonTone::Primary} else {ButtonTone::Normal})
                            .on_click(cx.listener(|this,_,window,cx|this.set_theme(Appearance::Dark,window,cx))))
                )
        )
        .child(
            components::panel(p).p(px(21.))
                .child(div().text_size(px(14.)).font_weight(FontWeight::SEMIBOLD).child("表格密度"))
                .child(div().h(px(7.)))
                .child(components::small_label("决定演示数据行高度，方便高信息密度工作台复用",p))
                .child(div().h(px(16.)))
                .child(h_flex().gap(px(9.))
                    .child(components::button("comfortable","舒适 43px",p,
                        if shell.density==Density::Comfortable {ButtonTone::Primary} else {ButtonTone::Normal})
                        .on_click(cx.listener(|this,_,_,cx|{this.density=Density::Comfortable;cx.notify();})))
                    .child(components::button("compact","紧凑 34px",p,
                        if shell.density==Density::Compact {ButtonTone::Primary} else {ButtonTone::Normal})
                        .on_click(cx.listener(|this,_,_,cx|{this.density=Density::Compact;cx.notify();}))))
        )
        .child(
            components::panel(p).p(px(21.))
                .child(div().text_size(px(14.)).font_weight(FontWeight::SEMIBOLD).child("导航布局"))
                .child(div().h(px(7.)))
                .child(components::small_label("切换全宽侧边栏与 68px 图标导轨",p))
                .child(div().h(px(16.)))
                .child(h_flex().gap(px(9.))
                    .child(components::button("sidebar-wide","完整侧栏",p,
                        if !shell.sidebar_compact {ButtonTone::Primary} else {ButtonTone::Normal})
                        .on_click(cx.listener(|this,_,_,cx|{this.sidebar_compact=false;cx.notify();})))
                    .child(components::button("sidebar-narrow","紧凑导轨",p,
                        if shell.sidebar_compact {ButtonTone::Primary} else {ButtonTone::Normal})
                        .on_click(cx.listener(|this,_,_,cx|{this.sidebar_compact=true;cx.notify();}))))
        )
        .child(
            components::panel(p).p(px(21.))
                .child(div().text_size(px(14.)).font_weight(FontWeight::SEMIBOLD).child("动画策略 · 预留"))
                .child(div().h(px(7.)))
                .child(components::small_label("用于后续 Motion 组件集成的状态；当前示例没有持续动画",p))
                .child(div().h(px(16.)))
                .child(components::button("motion-toggle",
                    if shell.animations {"状态：允许动效"} else {"状态：减少动效"},p,ButtonTone::Normal)
                    .on_click(cx.listener(|this,_,_,cx|{this.animations=!this.animations;cx.notify();})))
        )
        .into_any_element()
}
