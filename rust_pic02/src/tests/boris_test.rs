//! 使用解析性质检查 Boris 粒子推进器。

use crate::physics::field::EMField;
use crate::physics::particle::Particle;
use crate::physics::vector::Vec3;
use crate::solver::boris::boris_push;

#[test]
fn test_free_particle() {
    let mut electron = Particle::electron(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0e6, 0.0, 0.0),
    );
    let field = EMField {
        magnetic: Vec3::new(0.0, 0.0, 0.0),
        electric_amplitude: Vec3::new(0.0, 0.0, 0.0),
        frequency: 0.0,
        phase: 0.0,
    };
    let dt = 1.0e-12;
    let steps = 1000;
    for i in 0..steps {
        boris_push(&mut electron, &field, i as f64 * dt, dt);
    }
    let expected_x = 1.0e6 * dt * steps as f64;
    assert!((electron.position.x - expected_x).abs() < 1e-11);
}

#[test]
fn test_magnetic_energy_conservation() {
    let mut electron = Particle::electron(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0e6, 0.0, 1.0e6),
    );
    let field = EMField {
        magnetic: Vec3::new(0.0, 0.0, 0.07),
        electric_amplitude: Vec3::new(0.0, 0.0, 0.0),
        frequency: 0.0,
        phase: 0.0,
    };
    let initial_energy = electron.kinetic_energy_ev();
    let dt = 1.0e-12;
    for i in 0..10_000 {
        boris_push(&mut electron, &field, i as f64 * dt, dt);
    }
    let final_energy = electron.kinetic_energy_ev();
    let relative_error = ((final_energy - initial_energy) / initial_energy).abs();
    println!("纯磁场动能相对误差: {:.6e}", relative_error);
    assert!(relative_error < 1e-8);
}

#[test]
fn test_uniform_electric_acceleration() {
    // E=100 V/m, B=0；静电场对应频率为0、初相位为0。
    let mut electron = Particle::electron(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );
    let field = EMField {
        magnetic: Vec3::new(0.0, 0.0, 0.0),
        electric_amplitude: Vec3::new(100.0, 0.0, 0.0),
        frequency: 0.0,
        phase: 0.0,
    };
    let dt = 1e-12;
    let steps = 1000;
    for i in 0..steps {
        boris_push(&mut electron, &field, i as f64 * dt, dt);
    }
    let total_time = dt * steps as f64;
    let acceleration = electron.charge * 100.0 / electron.mass;
    let expected_v = acceleration * total_time;
    let expected_x = 0.5 * acceleration * total_time * total_time;
    assert!(((electron.velocity.x - expected_v) / expected_v).abs() < 1e-9);
    assert!(((electron.position.x - expected_x) / expected_x).abs() < 1e-9);
}
