//! 模拟时间循环，保持与具体求解器和数据输出的职责分离。

use std::io::{self, Write};
use std::path::Path;

use crate::output::{create_csv, write_particle};
use crate::physics::field::EMField;
use crate::physics::particle::Particle;
use crate::solver::boris::boris_push;

pub struct SimulationConfig {
    /// 时间步长，单位 s。
    pub dt: f64,
    /// 推进步数。
    pub steps: usize,
    /// 每隔多少步输出一次结果。
    pub output_interval: usize,
}

pub fn run(
    mut particle: Particle,
    field: &EMField,
    config: &SimulationConfig,
    output_path: &Path,
) -> io::Result<()> {
    if !config.dt.is_finite() || config.dt <= 0.0 || config.output_interval == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "dt 必须为有限正数，output_interval 必须大于 0",
        ));
    }

    let header = concat!(
        "step,time_s,x_m,y_m,z_m,",
        "vx_m_s,vy_m_s,vz_m_s,",
        "energy_eV,Ex_V_m,Ey_V_m,Ez_V_m,",
        "Bx_T,By_T,Bz_T"
    );
    let mut writer = create_csv(output_path, header)?;
    write_particle(&mut writer, 0, 0.0, &particle, field)?;

    for step in 1..=config.steps {
        let time = (step - 1) as f64 * config.dt;
        boris_push(&mut particle, field, time, config.dt);

        if step % config.output_interval == 0 || step == config.steps {
            write_particle(
                &mut writer, step, step as f64 * config.dt, &particle, field,
            )?;
        }
    }
    writer.flush()?;
    Ok(())
}
