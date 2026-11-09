//! Search, sort, and selection demonstrate proper separation of list state and cells.
use gpui_kit::{*, component::input::Input};
use crate::{
    app::Shell, components::{self, ButtonTone},
    data::{Process, filtered_sorted},
    design::Palette,
};

fn header(p:Palette) -> Div {
    div().flex().h(px(39.)).px(px(15.)).items_center()
        .bg(p.control).text_color(p.muted).text_size(px(11.))
        .child(div().flex_1().min_w_0().child("名称"))
        .child(div().w(px(76.)).child("PID"))
        .child(div().w(px(82.)).child("CPU"))
        .child(div().w(px(105.)).child("内存"))
        .child(div().w(px(82.)).child("磁盘"))
        .child(div().w(px(70.)).child("状态"))
}
fn row(shell:&Shell, process: Process, cx: &mut Context<Shell>) -> AnyElement {
    let p=shell.palette();
    let selected=shell.selected_pid==Some(process.id);
    div().flex().id(format!("pid-{}",process.id))
        .h(px(shell.density.row_height())).px(px(15.))
        .items_center().cursor_pointer()
        .border_b_1().border_color(p.border)
        .bg(if selected {p.accent_soft} else {p.surface})
        .hover(move |s|s.bg(p.hover))
        .text_size(px(12.))
        .child(
            div().flex().flex_1().min_w_0().gap(px(9.)).items_center()
                .child(div().size(px(25.)).rounded(px(5.)).bg(p.control)
                    .flex().items_center().justify_center().text_color(p.accent)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(process.name.chars().next().unwrap_or('A').to_string()))
                .child(div().flex().flex_col().min_w_0().gap(px(3.))
                    .child(div().text_color(p.text).font_weight(FontWeight::MEDIUM).child(process.name))
                    .child(div().text_color(p.faint).text_size(px(10.)).child(process.description)))
        )
        .child(div().w(px(76.)).text_color(p.muted).child(process.id.to_string()))
        .child(div().w(px(82.)).text_color(p.cpu).child(format!("{:.1}%",process.cpu)))
        .child(div().w(px(105.)).text_color(p.text).child(format!("{} MB",process.memory_mb)))
        .child(div().w(px(82.)).text_color(p.muted).child(format!("{:.1} MB",process.disk_mb)))
        .child(div().w(px(70.)).text_color(p.muted).child(process.state))
        .on_click(cx.listener(move |this,_,_,cx|{
            this.selected_pid=Some(process.id);
            cx.notify();
        }))
        .into_any_element()
}
fn detail(p: Palette, selected:Option<Process>) -> Div {
    let body=match selected {
        Some(process)=> div().flex().flex_col().gap(px(13.))
            .child(div().text_size(px(17.)).font_weight(FontWeight::SEMIBOLD).child(process.name))
            .child(components::tag(process.category,p,true))
            .child(components::divider(p))
            .child(components::small_label("应用说明",p))
            .child(div().text_size(px(12.)).text_color(p.text).child(process.description))
            .child(components::small_label("PID / 进程标识",p))
            .child(div().text_size(px(13.)).text_color(p.text).child(process.id.to_string()))
            .child(components::small_label("CPU 占用率",p))
            .child(components::progress(p,p.cpu,process.cpu))
            .child(components::small_label("内存占用",p))
            .child(div().text_size(px(13.)).child(format!("{} MB",process.memory_mb)))
            .child(div().h(px(10.)))
            .child(div().text_size(px(11.)).text_color(p.faint)
                .child("演示数据不支持结束进程或调整系统优先级。"))
            .into_any_element(),
        None=>div().flex().flex_col().gap(px(12.))
            .child(div().text_size(px(16.)).font_weight(FontWeight::SEMIBOLD).child("进程详情"))
            .child(components::small_label("选择左侧任意一行，查看模拟进程详情。",p))
            .into_any_element()
    };
    components::panel(p).w(px(245.)).flex_shrink_0().p(px(18.))
        .child(body)
}
pub fn render(shell:&mut Shell, _: &mut Window,cx:&mut Context<Shell>)->AnyElement{
    let p=shell.palette();
    let rows=filtered_sorted(&shell.query,shell.sort);
    let active=rows.iter().find(|row|Some(row.id)==shell.selected_pid).copied();
    let count=rows.len();
    let table_rows=rows.into_iter().map(|process|row(shell,process,cx)).collect::<Vec<_>>();
    div().flex().flex_col().w_full().p(px(26.)).gap(px(17.))
        .child(
            div().flex().justify_between().items_center()
                .child(div().flex().flex_col().gap(px(6.))
                    .child(components::heading("进程列表",p))
                    .child(components::small_label("筛选、选择与列排序的交互演示",p)))
                .child(components::tag(format!("{} 项结果",count),p,true))
        )
        .child(
            div().flex().gap(px(10.)).items_center()
                .child(
                    div().w(px(320.)).h(px(34.)).border_1().border_color(p.border)
                        .rounded(px(5.)).bg(p.surface).overflow_hidden()
                        .child(Input::new(&shell.search).appearance(false).w_full().h_full()
                            .px(px(9.)).text_size(px(12.)))
                )
                .child(components::button("sort",format!("排序：{}  ↻",shell.sort.title()),p,ButtonTone::Normal)
                    .on_click(cx.listener(|this,_,_,cx|{this.sort=this.sort.next();cx.notify();})))
                .child(components::tag("本地模拟数据",p,false))
        )
        .child(
            div().flex().items_start().gap(px(12.))
                .child(
                    components::panel(p).flex_1().min_w_0().overflow_hidden()
                        .child(header(p))
                        .children(table_rows)
                        .child(div().py(px(11.)).px(px(15.)).text_color(p.faint).text_size(px(11.))
                            .child(format!("共 {} 条记录 · 点击表格行查看详情",count)))
                )
                .child(detail(p,active))
        )
        .into_any_element()
}
