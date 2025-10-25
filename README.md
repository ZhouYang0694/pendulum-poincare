# pendulum-poincare

驱动阻尼摆的 Poincaré 截面生成器。以单一 JSON 文件描述物理参数、积分设置与绘图选项，支持固定步长和自适应积分器，按驱动周期采样 θ ∈ [−π, π]，输出高分辨率 PNG/SVG/HTML 图像。

## 特性速览

- 物理、积分、采样、绘图与输出全部由 `run.json` 配置，运行命令保持不变。
- 固定步长：Euler–Cromer、RK4；自适应：Dormand–Prince RK45、Bulirsch–Stoer，自动对齐每个驱动周期的采样时刻。
- 热身与采样周期数可独立设置，自适应积分会根据驱动周期推导步长上下限及容差默认值。
- 标题、坐标轴、刻度字体可单独设定；系统会依据字体大小自动扩展四周边距，避免大字号被裁剪。
- 同时生成 `output/<out_base>.png`、`.svg` 与 `.html` 三种格式，方便离线和交互式查看。

## 环境要求

- Rust 稳定版工具链（包含 `cargo`）

## 快速开始

```bash
cargo run --release -- run.json
```

- 使用 `--release` 可获得更好的性能与数值稳定性。
- 默认输出：`output/fig3_9.{png,svg,html}`，可通过 `output.out_base` 修改前缀。

## 配置说明

JSON 顶层字段：

| 字段 | 作用 |
| --- | --- |
| `phys` | 摆长度 `l`、重力 `g`、阻尼 `q`、驱动幅值 `f_drive`、角频率 `omega_d`。|
| `integrator` | 选择积分器及控制参数。固定步长可提供 `dt_user`；自适应支持 `rtol` / `atol` 与 `dt_init` / `dt_min` / `dt_max`，若缺省则按驱动周期派生。|
| `init` | 初始相位 `theta0`、角速度 `omega0`、起始时间 `t0`。|
| `poincare` | `wrap_to_pi` 控制是否将采样点折叠到 (−π, π]。|
| `plot` | 画布 `side_px`、标题 `title`、可选 `marker_size` 与 `title_font_px`/`axis_label_font_px`/`tick_font_px`。未填字体时会应用内置默认值并强制最低字号。|
| `output` | `out_base` 为输出文件名前缀。|

### 积分器提示

- 固定步长：`dt_user` 省略时会根据驱动周期自动对齐整周期采样，`steps_for_*` 会确保采样时刻恰逢整周期。
- 自适应：`rtol` 默认 1e−8、`atol` 默认 1e−10，步长初值/上下界按驱动周期给出。`dt_user` 在自适应模式下会被忽略。
- `n_periods_warmup` 为热身周期数，`n_periods_samples` 为采样周期数，两者都会映射为对应的时间网格。

### 绘图与输出

- 图像恒为正方形；万级像素画布对显存和磁盘要求较高，请按需调整。
- 字体大小同时应用于 Plotters（PNG/SVG）与 Plotly（HTML），并驱动边距计算；过小值会被提升到最小限度。
- HTML 输出保留交互缩放和 hover 信息，PNG/SVG 适合论文或离线展示。

## 示例配置（自适应积分）

```json
{
  "phys": {
    "g": 9.8,
    "l": 9.8,
    "q": 0.5,
    "f_drive": 1.2,
    "omega_d": 0.6666666666666666
  },
  "integrator": {
    "method": "RK45",
    "n_periods_warmup": 300,
    "n_periods_samples": 100000,
    "rtol": 1e-8,
    "atol": 1e-10
  },
  "init": { "theta0": 0.2, "omega0": 0.0, "t0": 0.0 },
  "poincare": { "wrap_to_pi": true },
  "plot": {
    "side_px": 4000,
    "title": "Poincaré Section (F_D = 1.2, RK45)",
    "marker_size": 1,
    "title_font_px": 96,
    "axis_label_font_px": 64,
    "tick_font_px": 48
  },
  "output": { "out_base": "fig3_9" }
}
```

将 `integrator.method` 改为 `EulerCromer` 或 `RK4` 时，可选地设置 `dt_user` 来指定固定步长。

## 参考资料

Reference: Computational Physics (2nd edition) by Nicholas J. Giordano and Hisao Nakanishi (Pearson/Addison-Wesley, 2005/2006). ISBN: 978-0131469907.
