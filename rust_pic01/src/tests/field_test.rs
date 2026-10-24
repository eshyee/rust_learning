use crate::{EMField, Vec3};

use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[test]
fn test_field_to_csv() -> std::io::Result<()> {

    // ====================================
    // 1. 定义电磁场
    // ====================================

    let field = EMField {
        magnetic: Vec3::new(0.0, 0.0, 0.07),
        electric_amplitude: Vec3::new(1000.0, 0.0, 0.0),
        frequency: 13.56e6,
        phase: 0.0,
    };

    // ====================================
    // 2. 设置模拟参数
    // ====================================

    let period = 1.0 / field.frequency;

    let num_periods = 4;
    let samples_per_period = 200;

    let total_samples = num_periods * samples_per_period;

    let dt = period / samples_per_period as f64;

    // ====================================
    // 3. 创建CSV文件
    // ====================================

    let project_dir = PathBuf::from(
        env!("CARGO_MANIFEST_DIR")
    );

    let output_dir = project_dir.join("results");

    fs::create_dir_all(&output_dir)?;

    let output_path = output_dir.join("rf_field_test.csv");

    let file = File::create(&output_path)?;

    let mut writer = BufWriter::new(file);

    // CSV表头
    writeln!(
        writer,
        "time_s,time_ns,Ex_V_m,Ey_V_m,Ez_V_m,Bx_T,By_T,Bz_T"
    )?;

    // ====================================
    // 4. 计算并保存电磁场
    // ====================================

    for i in 0..=total_samples {

        let time = i as f64 * dt;

        // 获取指定时刻的电磁场
        let (e, b) = field.at(time);

        writeln!(
            writer,
            "{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e}",
            time,
            time * 1e9,
            e.x,
            e.y,
            e.z,
            b.x,
            b.y,
            b.z
        )?;
    }

    writer.flush()?;

    // ====================================
    // 5. 验证电磁场
    // ====================================

    let (e0, b0) = field.at(0.0);

    let (e_half, _) = field.at(period / 2.0);

    let (e_full, _) = field.at(period);

    assert!((e0.x - 1000.0).abs() < 1e-9);

    assert!((e_half.x + 1000.0).abs() < 1e-9);

    assert!((e_full.x - 1000.0).abs() < 1e-9);

    assert!((b0.z - 0.07).abs() < 1e-12);

    // ====================================
    // 6. 输出信息
    // ====================================

    println!("\n====================================");

    println!("RF Field Test");

    println!(
        "Frequency: {:.2} MHz",
        field.frequency / 1e6
    );

    println!(
        "Period: {:.4} ns",
        period * 1e9
    );

    println!(
        "Sampling interval: {:.4} ns",
        dt * 1e9
    );

    println!(
        "Total data points: {}",
        total_samples + 1
    );

    println!(
        "CSV output: {}",
        output_path.display()
    );

    println!("====================================");

    Ok(())
}