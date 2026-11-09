# 架构与扩展指南

## 目标

Argus UI 是一个以 **Rust + GPUI Kit** 编写的原生 Windows 桌面 UI 起点，重点抽取稳定的信息层级、Fluent 视觉语言和高密度布局。它是独立实现，不包含 Argus 的采集/服务/渲染补丁。

## 调用链

```text
main.rs
  gpui_kit::application()
    -> open_window()
       -> app::Shell (Render)
          ├─ sidebar()       [导航]
          ├─ header()        [工具栏]
          ├─ pages/*         [业务页面]
          └─ status bar
```

- **app.rs**：统一导航与小范围全局状态。保持稳定、避免随着新业务无限增长。
- **pages/**：领域页面，负责组合 UI 与维护本领域操作。新增复杂页面时拆出 `state.rs`、`model.rs`、`commands.rs`。
- **components.rs**：无业务依赖、可复用的视觉构件。采用 Builder 与语义参数；调用方绑定动作。
- **design.rs**：配色角色、外观模式、表格密度。
- **data.rs**：完全本地、纯函数的演示数据，替换时优先使用强类型快照。
- **platform/**：需要系统平台能力时再创建，不要将 Windows API 放进通用视觉组件。

## 扩展一个页面

1. 新建 `src/pages/your_page.rs` 并导出 `render(shell, window, cx)`（大型功能推荐独立 `Entity<YourPage>`）。
2. 在 `src/pages/mod.rs` 声明模块。
3. 在 `app.rs::Page` 新增一个类型安全的页面枚举成员及其标题、图标和路由分支。
4. 如果数据采集会阻塞，将它移出 UI 线程；采集结果应聚合为快照后通知 UI。
5. 测试搜索、排序、分页或交互，确保旧结果不会覆盖新的页面状态。

## 什么时候创建独立 Entity

| 情况 | 推荐方式 |
|---|---|
| 静态文字、按钮、标签、简单卡片 | Stateless Element / Builder |
| Input、可编辑控件、复杂数据表格 | 独立 Entity + Context |
| 页面需要自己的生命周期与异步任务 | Feature Entity |
| 长期持续动画 / 大量独立时序曲线 | 单独 Render 图层或缓存视图 |
| 极高频率的系统数据 | 后台采样器 + 最新值通道 |

真正需要局部绘制时再引入独立图表层和 GPU 缓存；不要照搬 Argus 的底层 GPUI 补丁。先有可靠的基准测试，再优化。

## 质量与边界

- `cargo check` 验证类型与构建。
- `cargo clippy --all-targets` 检查常见错误。
- `cargo test` 运行纯数据测试。
- `cargo build --release` 编译可执行程序。
- 当前进程、网络和硬件数值均为模拟数据，不访问系统敏感能力。
- 主题、密度和导航在当前运行期间有效。持久化是可扩展项，尚未宣称支持。
