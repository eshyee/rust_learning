//! Rust PIC V0.1：单电子在外加静磁场和射频电场中的测试粒子运动。
//! 本版本不是自洽 PIC；尚未计算电荷沉积、空间网格或自洽场。

pub mod physics;
pub mod solver;
pub mod simulation;
pub mod output;

#[cfg(test)]
mod tests;
