//! 电子（后续可以推广到离子与宏粒子）的状态和动能。

use super::constants::{E_CHARGE, E_MASS};
use super::vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    /// 位置，单位 m。
    pub position: Vec3,
    /// 速度，单位 m/s。
    pub velocity: Vec3,
    /// 电荷，单位 C，电子为负值。
    pub charge: f64,
    /// 质量，单位 kg。
    pub mass: f64,
}

impl Particle {
    /// 创建电子。
    pub fn electron(position: Vec3, velocity: Vec3) -> Self {
        Self {
            position,
            velocity,
            charge: -E_CHARGE,
            mass: E_MASS,
        }
    }

    /// 非相对论动能，单位 eV。
    pub fn kinetic_energy_ev(&self) -> f64 {
        0.5 * self.mass * self.velocity.norm_squared() / E_CHARGE
    }
}
