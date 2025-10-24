# pendulum-poincare

单次运行的驱动阻尼摆 Poincaré 截面生成工具，使用 JSON 配置驱动，内部固定 θ ∈ [−π, π]，输出 PNG/SVG/HTML 图像。

## 环境要求

- 已安装稳定版 Rust 与 Cargo

## 快速开始

- `cargo run --release -- run.json`
- 输出文件：`fig3_9.png`、`fig3_9.svg`、`fig3_9.html`（位于项目根目录）

## 配置说明

- `dt` 通过 `omega_d` 自动转换为整数步数；省略 `dt_user` 即可自动求取。
- 每次运行只模拟一个参数集，可在 `phys` 中设置 `f_drive` 等物理量。
- `wrap_to_pi` 可在 JSON 中切换，用于决定采样时是否折叠角度。
- x 轴范围内部固定为 [−π, π]，可在 JSON 中通过 `omega_min`/`omega_max` 调整 y 轴范围。

## 示例配置

```json
{
  "phys": { "g": 9.8, "l": 9.8, "q": 0.5, "f_drive": 1.2, "omega_d": 0.6666666666666666 },
  "integrator": { "method": "RK4", "n_periods_warmup": 300, "n_periods_samples": 2500 },
  "init": { "theta0": 0.2, "omega0": 0.0, "t0": 0.0 },
  "poincare": { "wrap_to_pi": true },
  "plot": { "theta_min": -3.141592653589793, "theta_max": 3.141592653589793, "omega_min": -2.0, "omega_max": 3.0, "width_px": 1200, "height_px": 900, "title": "Poincaré Section (F_D = 1.2, RK4)" },
  "output": { "out_base": "fig3_9" }
}
```

## 参考资料

Reference: Computational Physics (2nd edition) by Nicholas J. Giordano and Hisao Nakanishi (Pearson/Addison-Wesley, 2005/2006). ISBN: 978-0131469907.
