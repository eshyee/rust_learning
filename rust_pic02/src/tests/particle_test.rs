//! 验证电子基本属性与非相对论动能。

use crate::physics::constants::{E_CHARGE, E_MASS};
use crate::physics::particle::Particle;
use crate::physics::vector::Vec3;

#[test]
fn test_electron_properties() {
    let electron = Particle::electron(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0e6, 0.0, 0.0),
    );
    assert_eq!(electron.charge, -E_CHARGE);
    assert_eq!(electron.mass, E_MASS);

    let expected = 0.5 * E_MASS * (1.0e6_f64).powi(2) / E_CHARGE;
    assert!((electron.kinetic_energy_ev() - expected).abs() < 1e-12);
}
