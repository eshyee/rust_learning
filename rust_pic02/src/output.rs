//! CSV 数据输出接口。

use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;

use crate::physics::field::EMField;
use crate::physics::particle::Particle;

/// 创建 CSV，自动创建其父目录并写入表头。
pub fn create_csv(path: &Path, header: &str) -> io::Result<BufWriter<File>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", header)?;
    Ok(writer)
}

/// 输出一个粒子在指定时间的数据（SI 单位，动能为 eV）。
pub fn write_particle(
    writer: &mut impl Write,
    step: usize,
    time: f64,
    particle: &Particle,
    field: &EMField,
) -> io::Result<()> {
    let p = particle.position;
    let v = particle.velocity;
    let (e, b) = field.at(time);

    writeln!(
        writer,
        "{},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e},{:.12e}",
        step, time, p.x, p.y, p.z,
        v.x, v.y, v.z, particle.kinetic_energy_ev(),
        e.x, e.y, e.z, b.x, b.y, b.z
    )
}
