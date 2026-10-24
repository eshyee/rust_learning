use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::{Add, Mul};

// ============================================================
// 1. 物理常数
// ============================================================

const E_CHARGE: f64 = 1.602176634e-19;  // 元电荷绝对值 C
const E_MASS: f64 = 9.1093837139e-31;  // 电子质量 kg

// ============================================================
// 2. 三维向量
// ============================================================

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // 向量叉乘
    fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // 向量模长平方
    fn norm_squared(self) -> f64 {
        self.x * self.x
            + self.y * self.y
            + self.z * self.z
    }
}

// 向量加法
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

// 向量乘标量
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        )
    }
}

// ============================================================
// 3. 粒子结构体
// ============================================================

#[derive(Debug)]
struct Particle {
    position: Vec3,
    velocity: Vec3,
    charge: f64,
    mass: f64,
}

impl Particle {
    // 创建电子
    fn electron(position: Vec3, velocity: Vec3) -> Self {
        Self {
            position,
            velocity,
            charge: -E_CHARGE,
            mass: E_MASS,
        }
    }

    // 计算电子动能，单位 eV
    fn kinetic_energy_ev(&self) -> f64 {
        0.5 * self.mass * self.velocity.norm_squared()
            / E_CHARGE
    }
}

// ============================================================
// 4. 外加电磁场
// ============================================================

struct EMField {
    // 均匀静磁场
    magnetic: Vec3,

    // 射频电场幅值矢量
    electric_amplitude: Vec3,

    // 射频频率 Hz
    frequency: f64,

    // 初始相位 rad
    phase: f64,
}

impl EMField {
    // 获取指定时间的电磁场
    fn at(&self, time: f64) -> (Vec3, Vec3) {
        let omega = 2.0 * PI * self.frequency;

        let factor = (omega * time + self.phase).cos();

        let electric = self.electric_amplitude * factor;

        (electric, self.magnetic)
    }
}

// ============================================================
// 5. Boris 粒子推进算法
// ============================================================

fn boris_push(
    particle: &mut Particle,
    field: &EMField,
    time: f64,
    dt: f64,
) {
    // 在时间步中点计算电磁场
    let (electric, magnetic) = field.at(time + 0.5 * dt);

    // qΔt/(2m)
    let factor =
        particle.charge * dt / (2.0 * particle.mass);

    // 第一次电场半步加速
    let v_minus =
        particle.velocity + electric * factor;

    // 磁场旋转参数
    let t = magnetic * factor;

    let s = t * (2.0 / (1.0 + t.norm_squared()));

    // Boris 磁场旋转
    let v_prime = v_minus + v_minus.cross(t);

    let v_plus = v_minus + v_prime.cross(s);

    // 第二次电场半步加速
    let v_new = v_plus + electric * factor;

    // 使用旧、新速度的平均值更新位置
    // 对本例空间均匀电磁场构成二阶时间推进
    particle.position =
        particle.position
            + (particle.velocity + v_new) * (0.5 * dt);

    particle.velocity = v_new;
}

// ============================================================
// 6. CSV 数据输出
// ============================================================

fn write_data(
    writer: &mut BufWriter<File>,
    step: usize,
    time: f64,
    particle: &Particle,
    field: &EMField,
) -> std::io::Result<()> {
    let (e, b) = field.at(time);

    writeln!(
        writer,
        "{},{:.12e},{:.12e},{:.12e},{:.12e},\
         {:.12e},{:.12e},{:.12e},{:.12e},\
         {:.12e},{:.12e},{:.12e},\
         {:.12e},{:.12e},{:.12e}",
        step,
        time,
        particle.position.x,
        particle.position.y,
        particle.position.z,
        particle.velocity.x,
        particle.velocity.y,
        particle.velocity.z,
        particle.kinetic_energy_ev(),
        e.x,
        e.y,
        e.z,
        b.x,
        b.y,
        b.z
    )
}

// ============================================================
// 7. 主程序
// ============================================================

fn main() -> std::io::Result<()> {

    // ----------------------------
    // 模拟参数
    // ----------------------------

    let dt = 1.0e-12;

    let steps = 300_000;

    // 每10步输出一次数据
    let output_interval = 10;

    // ----------------------------
    // 初始化电子
    // ----------------------------

    let mut electron = Particle::electron(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0e6, 0.0, 1.0e6),
    );

    // ----------------------------
    // 定义电磁场
    // ----------------------------

    let field = EMField {
        magnetic: Vec3::new(
            0.0,
            0.0,
            0.07,
        ),

        electric_amplitude: Vec3::new(
            1000.0,
            0.0,
            0.0,
        ),

        frequency: 13.56e6,

        phase: 0.0,
    };

    // ----------------------------
    // 计算理论回旋参数
    // ----------------------------

    let b = field.magnetic.z.abs();

    let omega_ce = E_CHARGE * b / E_MASS;

    let f_ce = omega_ce / (2.0 * PI);

    let v_perp = (
        electron.velocity.x.powi(2)
            + electron.velocity.y.powi(2)
    ).sqrt();

    let r_larmor = v_perp / omega_ce;

    println!("================================");
    println!("Rust 单电子电磁场运动模拟");
    println!("================================");

    println!("磁场: {:.1} G", b * 1.0e4);

    println!(
        "射频频率: {:.2} MHz",
        field.frequency / 1.0e6
    );

    println!(
        "电子回旋频率: {:.4} GHz",
        f_ce / 1.0e9
    );

    println!(
        "理论回旋半径: {:.6} mm",
        r_larmor * 1.0e3
    );

    println!(
        "模拟时间: {:.3} ns",
        dt * steps as f64 * 1.0e9
    );

    // ----------------------------
    // 创建输出文件
    // ----------------------------

    let file = File::create("trajectory.csv")?;

    let mut writer = BufWriter::new(file);

    writeln!(
        writer,
        "step,time_s,x_m,y_m,z_m,\
         vx_m_s,vy_m_s,vz_m_s,\
         energy_eV,Ex_V_m,Ey_V_m,Ez_V_m,\
         Bx_T,By_T,Bz_T"
    )?;

    // 输出初始状态
    write_data(
        &mut writer,
        0,
        0.0,
        &electron,
        &field,
    )?;

    // ----------------------------
    // 时间推进
    // ----------------------------

    for i in 1..=steps {

        let time = (i - 1) as f64 * dt;

        // 推进一个时间步
        boris_push(
            &mut electron,
            &field,
            time,
            dt,
        );

        // 定期记录轨迹
        if i % output_interval == 0 || i == steps {
            write_data(
                &mut writer,
                i,
                i as f64 * dt,
                &electron,
                &field,
            )?;
        }
    }

    writer.flush()?;

    println!("================================");
    println!("模拟完成");
    println!("输出文件: trajectory.csv");

    println!(
        "最终电子动能: {:.6} eV",
        electron.kinetic_energy_ev()
    );

    Ok(())
}

// ============================================================
// 8. 单元测试
// ============================================================

// 引入测试模块
#[cfg(test)]
mod tests;