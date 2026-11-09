# 视觉规范 · Fluent-inspired Native UI

本项目从 Argus 的公开设计中借鉴信息密度、层级、稳定读数和 Windows 原生环境融合的思路，但不复制任何 Argus 代码、图标资源和驱动实现。

## 视觉原则

- 让内容比容器更突出：背景与卡片的对比度克制，避免巨大的玻璃反光和装饰性阴影。
- 视觉层级来源于字号、字重、对齐与间距，不依赖过多强调色。
- 色彩按语义分配：CPU 青、内存紫、磁盘琥珀、网络绿、GPU 粉。
- 行数据保持等宽数值、稳定列宽与清晰选中状态。
- 控件优先 32px 高、4-8px 圆角、1px 边框，避免大圆角胶囊堆叠。
- Windows 下首选 Segoe UI Variable 字族，中文由系统字体回退。
- 明暗主题都维护相同的语义色角色，而不是对浅色界面直接反色。

## 组件语义

| 构件 | 源码 | 说明 |
|---|---|---|
| Button | `components::button` | Normal / Primary / Ghost / Danger |
| Panel | `components::panel` | 普通表面、细边框、8px 圆角 |
| Tag | `components::tag` | 轻强调 / 普通说明 |
| Progress | `components::progress` | 语义色填充轨道 |
| Sparkline | `components::sparkline` | 轻量静态示例，非全功能图表 |
| Shell | `app::Shell` | 侧栏、标题区、页面和状态栏 |

## 默认配色

所有真实定义均以 `src/design.rs` 为准。以下是主要语义角色。

| 角色 | 浅色 | 深色 |
|---|---|---|
| Background | #F3F3F5 | #202124 |
| Surface | #FFFFFF | #2A2B2F |
| Text | #23252C | #F1F2F5 |
| Accent | #0067C0 | #59BAFF |
| CPU | #168CB3 | #49C7E8 |
| Memory | #7655DA | #A695FF |
| Network | #249A68 | #64D69F |

## 非目标

- 不把静态迷你柱状图宣传为 Argus 的 DirectComposition 曲线。
- 不内置特权服务、内核传感器、进程操作或驱动。
- 不使用 Argus 的 AGPL 源码或本地 GPUI vendor 补丁。
