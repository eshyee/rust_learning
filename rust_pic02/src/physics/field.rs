//! 均匀静磁场 B 与时间变化的线偏振射频电场 E(t)。

use std::f64::consts::PI;
use super::vector::Vec3;

pub struct EMField {
    /// 恒定磁感应强度，单位 T。
    pub magnetic: Vec3,
    /// 余弦射频电场幅值矢量，单位 V/m。
    pub electric_amplitude: Vec3,
    /// 射频频率，单位 Hz。
    pub frequency: f64,
    /// 初相位，单位 rad。
    pub phase: f64,
}

impl EMField {
    /// 获取指定时间（单位 s）的瞬时电场与磁场：
    /// E(t) = E0 cos(2π f t + φ)，B(t) = B0。
    pub fn at(&self, time: f64) -> (Vec3, Vec3) {
        let omega = 2.0 * PI * self.frequency;
        let factor = (omega * time + self.phase).cos();
        let electric = self.electric_amplitude * factor;
        (electric, self.magnetic)
    }
}
