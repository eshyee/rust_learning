//! 检查给定时间的 RF 电场与静磁场，输出 4 个周期的 CSV。

use std::io::{self, Write};
use std::path::PathBuf;

use crate::output::create_csv;
use crate::physics::field::EMField;
use crate::physics::vector::Vec3;

#[test]
fn test_field_to_csv() -> io::Result<()> {
    let field = EMField {
        magnetic: Vec3::new(0.0, 0.0, 0.07),
        electric_amplitude: Vec3::new(1000.0, 0.0, 0.0),
        frequency: 13.56e6,
        phase: 0.0,
    };

    let period = 1.0 / field.frequency;
    let num_periods = 4;
    let samples_per_period = 200;
    let total_samples = num_periods * samples_per_period;
    let dt = period / samples_per_period as f64;

    let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("results")
        .join("fields")
        .join("rf_field_test.csv");

    let mut writer = create_csv(
        &output_path,
        "time_s,time_ns,Ex_V_m,Ey_V_m,Ez_V_m,Bx_T,By_T,Bz_T",
    )?;

    for i in 0..=total_samples {
        let time = i as f64 * dt;
        let (e, b) = field.at(time);
        writeln!(
            writer,
            "{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e}",
            time, time * 1e9, e.x, e.y, e.z, b.x, b.y, b.z
        )?;
    }
    writer.flush()?;

    // 验证电场 E(0)=+E0、E(T/2)=-E0、E(T)=+E0。
    let (e0, b0) = field.at(0.0);
    let (e_half, _) = field.at(period / 2.0);
    let (e_full, _) = field.at(period);
    assert!((e0.x - 1000.0).abs() < 1e-9);
    assert!((e_half.x + 1000.0).abs() < 1e-9);
    assert!((e_full.x - 1000.0).abs() < 1e-9);
    assert!((b0.z - 0.07).abs() < 1e-12);

    println!("RF 周期: {:.4} ns", period * 1e9);
    println!("共 {} 个数据点", total_samples + 1);
    println!("CSV 输出: {}", output_path.display());
    Ok(())
}
