//! Boris 粒子推进器。此版本适合空间均匀的外加电磁场。

use crate::physics::field::EMField;
use crate::physics::particle::Particle;

/// 将粒子从 time 推进到 time + dt。
/// 使用时间步中点的电场，速度由 Boris 算法更新，位置由平均速度更新。
/// 未来扩展为标准 PIC 时，可改用位置、速度交错的 leapfrog 格式。
pub fn boris_push(particle: &mut Particle, field: &EMField, time: f64, dt: f64) {
    // 1. 获取时间步中点的外加电场、磁场。
    let (electric, magnetic) = field.at(time + 0.5 * dt);

    // 2. 因子 q Δt / (2m)；电子电荷为负。
    let factor = particle.charge * dt / (2.0 * particle.mass);

    // 3. 第一段电场半步加速。
    let v_minus = particle.velocity + electric * factor;

    // 4. 纯磁场旋转：t, s 是 Boris 旋转向量。
    let t = magnetic * factor;
    let s = t * (2.0 / (1.0 + t.norm_squared()));
    let v_prime = v_minus + v_minus.cross(t);
    let v_plus = v_minus + v_prime.cross(s);

    // 5. 第二段电场半步加速。
    let v_new = v_plus + electric * factor;

    // 6. 使用前后速度平均值更新粒子位置。
    let v_average = (particle.velocity + v_new) * 0.5;
    particle.position = particle.position + v_average * dt;
    particle.velocity = v_new;
}
