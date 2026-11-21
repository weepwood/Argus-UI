//! Interactive visual catalog for the scaffold's reusable builders.
use crate::{
    app::Shell,
    components::{self, ButtonTone},
};
use gpui_kit::*;

pub fn render(shell: &mut Shell, cx: &mut Context<Shell>) -> AnyElement {
    let p = shell.palette();
    div().flex().flex_col().w_full().p(px(26.)).gap(px(19.))
        .child(
            div().flex().flex_col().gap(px(6.))
                .child(components::heading("组件展示",p))
                .child(components::small_label("原生控件的视觉令牌、交互状态与组合示例",p))
        )
        .child(
            components::panel(p).p(px(20.))
                .child(div().text_size(px(14.)).font_weight(FontWeight::SEMIBOLD).child("按钮 · Buttons"))
                .child(div().h(px(7.)))
                .child(components::small_label("所有按钮共享同一套形态、字体与反馈规则",p))
                .child(div().h(px(20.)))
                .child(div().flex().gap(px(9.)).items_center()
                    .child(components::button("primary","主操作",p,ButtonTone::Primary)
                        .on_click(cx.listener(|this,_,_,cx|{this.demo_clicks+=1;cx.notify();})))
                    .child(components::button("secondary","次要操作",p,ButtonTone::Normal)
                        .on_click(cx.listener(|this,_,_,cx|{this.demo_clicks+=1;cx.notify();})))
                    .child(components::button("ghost","幽灵按钮",p,ButtonTone::Ghost)
                        .on_click(cx.listener(|this,_,_,cx|{this.demo_clicks+=1;cx.notify();})))
                    .child(components::button("danger","危险操作",p,ButtonTone::Danger)
                        .on_click(cx.listener(|this,_,_,cx|{this.demo_clicks+=1;cx.notify();}))))
                .child(div().h(px(17.)))
                .child(components::small_label(format!("演示点击计数：{} 次",shell.demo_clicks),p))
        )
        .child(
            div().flex().gap(px(14.))
                .child(
                    components::panel(p).flex_1().min_w_0().p(px(19.))
                        .child(div().font_weight(FontWeight::SEMIBOLD).child("语义标签"))
                        .child(div().h(px(14.)))
                        .child(div().flex().gap(px(8.))
                            .child(components::tag("默认",p,false))
                            .child(components::tag("已选中",p,true)))
                        .child(div().h(px(23.)))
                        .child(div().font_weight(FontWeight::SEMIBOLD).child("微型进度控件"))
                        .child(div().h(px(13.)))
                        .child(components::progress(p,p.cpu,58.))
                        .child(div().h(px(13.)))
                        .child(components::progress(p,p.memory,35.))
                        .child(div().h(px(13.)))
                        .child(components::progress(p,p.network,76.))
                )
                .child(
                    components::panel(p).flex_1().min_w_0().p(px(19.))
                        .child(div().font_weight(FontWeight::SEMIBOLD).child("数据色板"))
                        .child(div().h(px(16.)))
                        .child(div().flex().flex_col().gap(px(13.))
                            .child(components::color_swatch("处理器 · Cyan",p.cpu,p))
                            .child(components::color_swatch("内存 · Violet",p.memory,p))
                            .child(components::color_swatch("磁盘 · Amber",p.disk,p))
                            .child(components::color_swatch("网络 · Green",p.network,p))
                            .child(components::color_swatch("GPU · Magenta",p.gpu,p)))
                )
        )
        .child(
            components::panel(p).p(px(20.))
                .child(div().font_weight(FontWeight::SEMIBOLD).child("设计系统约定"))
                .child(div().h(px(12.)))
                .child(components::small_label(
                    "13px 基准字号 · 1px 分隔线 · 4–8px 圆角 · 状态使用语义色 · 结构使用紧凑间距",p))
                .child(div().h(px(13.)))
                .child(components::divider(p))
                .child(div().h(px(13.)))
                .child(components::small_label(
                    "组件是独立的样式构造函数，不依赖页面业务。真实业务状态保存在各 Feature 或 Shell。",p))
        )
        .child(
            components::panel(p)
                .p(px(19.))
                .child(div().font_weight(FontWeight::SEMIBOLD).child("密集数据表 · Dense Table"))
                .child(div().h(px(10.)))
                .child(components::small_label(
                    "表头、表行、选中态和空状态可在其他桌面应用中直接复用；数据与事件仍由页面管理。",
                    p,
                ))
                .child(div().h(px(14.)))
                .child(components::dense_table_header(["模块", "状态", "类型"], p))
                .child(components::dense_table_row(
                    "showcase-row-1",
                    ["Core Service", "运行中", "后台任务"],
                    p,
                    shell.density.row_height(),
                    true,
                ))
                .child(components::dense_table_row(
                    "showcase-row-2",
                    ["Resource Monitor", "就绪", "面板"],
                    p,
                    shell.density.row_height(),
                    false,
                ))
                .child(components::table_empty_state(
                    "没有匹配的条目时，使用统一的空状态提示。",
                    p,
                )),
        )
        .into_any_element()
}
