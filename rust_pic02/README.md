# Rust PIC V0.1：单电子在射频电场和静磁场中的运动

这是一个便于学习和逐步扩展为 PIC（Particle-in-Cell）的**最小 Rust 项目**。目前只模拟**一个电子在指定外加电磁场中的轨迹**，属于测试粒子模型，还没有空间网格、电荷沉积、Poisson/Maxwell 自洽求解及粒子碰撞。

源码带有中文注释；Rust 计算、Rust 测试、Python 数据分析分别存放。

## 1. 目录说明

```text
rust_pic01/
├── Cargo.toml
├── src/
│   ├── main.rs                # 程序入口
│   ├── lib.rs                 # 模块声明
│   ├── physics/
│   │   ├── mod.rs
│   │   ├── constants.rs       # 物理常数
│   │   ├── vector.rs          # 三维向量
│   │   ├── particle.rs        # 电子属性
│   │   └── field.rs           # E(t) 和静磁场
│   ├── solver/
│   │   ├── mod.rs
│   │   └── boris.rs           # Boris 推进器
│   ├── simulation.rs          # 时间循环
│   ├── output.rs              # CSV 输出
│   └── tests/
│       ├── mod.rs
│       ├── field_test.rs      # RF 电磁场测试并导出 CSV
│       ├── particle_test.rs   # 电子质量、电荷、动能测试
│       └── boris_test.rs      # 直线、回旋能量、匀电场加速测试
├── python/
│   ├── requirements.txt
│   ├── scripts/
│   │   ├── plot_rf_field.py
│   │   └── plot_trajectory.py
│   ├── notebooks/
│   │   ├── 01_field.ipynb
│   │   └── 02_particle.ipynb
│   └── utils/
│       └── __init__.py
├── results/
│   ├── fields/               # cargo test 生成 rf_field_test.csv
│   ├── trajectories/         # cargo run 生成 electron.csv
│   └── figures/              # Python 绘图生成 PNG
└── docs/
    └── physics.md
```

`results/` 内默认只包含占位文件 `.gitkeep`，**CSV 和绘图需要在本机运行后生成**。

## 2. 物理设定

- 电子初始位置 `(0, 0, 0)` m；初速度 `(1e6, 0, 1e6)` m/s。
- 均匀静磁场 `B=(0,0,0.07)` T，对应 `700 G`。
- 线偏振 RF 电场 `E(t)=(1000 cos(2π·13.56e6·t),0,0)` V/m。
- 时间步长 `dt=1e-12` s，步数 `300000`，模拟时间 `300 ns`。
- 每 10 步保存一条粒子轨迹。
- **1000 V/m 仅为示例幅值，并非测得的 Helicon 电场。**

## 3. 开始运行（macOS、Linux、Windows）

先安装 Rust 工具链（需要 `cargo`），然后在项目根目录执行：

```bash
cargo check
cargo test -- --list
cargo test -- --nocapture
cargo run --release
```

若只执行电磁场测试并查看输出：

```bash
cargo test test_field_to_csv -- --nocapture
```

正常情况下：

- `cargo test test_field_to_csv` → `results/fields/rf_field_test.csv`
- `cargo run --release` → `results/trajectories/electron.csv`

注意：`running 0 tests` **不代表验证通过**。运行 `cargo test -- --list` 应列出 5 个测试函数：

```text
tests::field_test::test_field_to_csv
tests::particle_test::test_electron_properties
tests::boris_test::test_free_particle
tests::boris_test::test_magnetic_energy_conservation
tests::boris_test::test_uniform_electric_acceleration
```

## 4. Python 数据分析

```bash
python -m pip install -r python/requirements.txt
python python/scripts/plot_rf_field.py
python python/scripts/plot_trajectory.py
```

绘图输出：

```text
results/figures/rf_field.png
results/figures/electron_trajectory.png
results/figures/electron_energy.png
```

用 Notebook 交互式分析：

```bash
jupyter lab python/notebooks
```

在 `python/notebooks/` 打开 `01_field.ipynb`、`02_particle.ipynb` 即可。Notebook 会自动向上寻找 `Cargo.toml` 定位项目根目录，不依赖固定的机器路径。

## 5. 如何更改模拟参数

编辑 `src/main.rs` 中的 `electron`、`field`、`config`：

- `field.magnetic`：磁场矢量 (T)
- `field.electric_amplitude`：RF 电场幅值矢量 (V/m)
- `field.frequency`：频率 (Hz)
- `field.phase`：初始相位 (rad)
- `config.dt`、`config.steps`：时间步长与步数

注意 `src/tests/field_test.rs` 使用固定测试参数（700 G、13.56 MHz、1000 V/m），确保测试基准不会随着主程序参数修改而变化。

## 6. 后续开发顺序

1. 验证纯磁场回旋、无场直线运动、纯电场加速；
2. 完善 RF 场与粒子运动分析；
3. 多粒子、初始速度分布；
4. 粒子网格插值、CIC 电荷沉积；
5. Poisson 自洽静电场、PIC；
6. Maxwell/FDTD 电磁 PIC，必要时增加 MCC 与 GPU 加速。

详见 `docs/physics.md`。
