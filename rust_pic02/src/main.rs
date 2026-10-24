//! 程序入口：只设置参数并启动模拟，不直接实现数值算法。

use std::path::PathBuf;

use rust_pic01::physics::field::EMField;
use rust_pic01::physics::particle::Particle;
use rust_pic01::physics::vector::Vec3;
use rust_pic01::simulation::{run, SimulationConfig};

fn main() -> std::io::Result<()> {
    // 初始位置 (m) 与速度 (m/s)。
    let electron = Particle::electron(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0e6, 0.0, 1.0e6),
    );

    // 700 G 静磁场（沿 Z）；13.56 MHz，1000 V/m 的线偏振 RF 电场（沿 X）。
    let field = EMField {
        magnetic: Vec3::new(0.0, 0.0, 0.07),
        electric_amplitude: Vec3::new(1000.0, 0.0, 0.0),
        frequency: 13.56e6,
        phase: 0.0,
    };

    let config = SimulationConfig {
        dt: 1.0e-12,
        steps: 300_000,
        output_interval: 10,
    };

    // 始终使用 Cargo 项目根目录，而不是当前终端所在目录。
    let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("results")
        .join("trajectories")
        .join("electron.csv");

    println!("开始模拟：B = 700 G, f_RF = 13.56 MHz, E0 = 1000 V/m");
    println!("模拟时长 = {:.3} ns", config.dt * config.steps as f64 * 1e9);
    run(electron, &field, &config, &output_path)?;
    println!("模拟完成：{}", output_path.display());
    Ok(())
}
