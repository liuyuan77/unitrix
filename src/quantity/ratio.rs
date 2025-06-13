// ratio.rs
//! 比例因子模块，处理所有非10的幂次的比例关系，包括时间单位、角度单位等
use crate::sealed::Sealed;
use crate::number::{Integer, NonZero, Z0};

use core::marker::PhantomData;
use core::ops::{Add, Sub, Mul, Div};
// use core::f64::consts::PI;

/// 比例因子结构体，使用质因数分解表示
/// Exp5: 5的幂次
/// Exp3: 3的幂次
/// ExpPi: PI的幂次
/// Exp2: 2的幂次
#[derive(Debug, Clone, Copy, Default)]
pub struct Ratio<Exp5: Integer, ExpPi:Integer, Exp3: Integer, Exp2: Integer>(
    PhantomData<(Exp5, ExpPi, Exp3, Exp2)>
);

/* impl<E5: Integer, Pi: Integer, E3: Integer, E2: Integer> Ratio<E5, Pi, E3, E2>{
    /// 计算比例因子的实际值
    pub fn value() -> f64 {
        5f64.powi(E5::to_i32()) * PI.powi(Pi::to_i32()) * 3f64.powi(E3::to_i32()) * 2f64.powi(E2::to_i32())
    }

    /* /// 获取比例因子的符号表示
    pub fn symbol() -> &'static str {
        // 检查是否是时间单位
        if let Some(sym) = Self::time_symbol() {
            return sym;
        }
    } */ 
} */

// 实现比例因子的乘法
impl<E5a, EPia, E3a, E2a, E5b, EPib, E3b, E2b> Mul<Ratio<E5b, EPib, E3b, E2b>> for Ratio<E5a, EPia, E3a, E2a>
where
    E5a: Integer + Add<E5b, Output: Integer>,
    EPia: Integer + Add<EPib, Output: Integer>,
    E3a: Integer + Add<E3b, Output: Integer>,
    E2a: Integer + Add<E2b, Output: Integer>,
    E5b: Integer,
    EPib: Integer,
    E3b: Integer,
    E2b: Integer,
{
    type Output = Ratio<
        <E5a as Add<E5b>>::Output,
        <EPia as Add<EPib>>::Output,
        <E3a as Add<E3b>>::Output,
        <E2a as Add<E2b>>::Output
    >;
    
    fn mul(self, _: Ratio<E5b, EPib, E3b, E2b>) -> Self::Output {
        Ratio(PhantomData)
    }
}

// 实现比例因子的除法
impl<E5a, EPia, E3a, E2a, E5b, EPib, E3b, E2b> Div<Ratio<E5b, EPib, E3b, E2b>> for Ratio<E5a, EPia, E3a, E2a>
where
    E5a: Integer + Sub<E5b, Output: Integer>,
    EPia: Integer + Sub<EPib, Output: Integer>,
    E3a: Integer + Sub<E3b, Output: Integer>,
    E2a: Integer + Sub<E2b, Output: Integer>,
    E5b: Integer,
    EPib: Integer,
    E3b: Integer,
    E2b: Integer,
{
    type Output = Ratio<
        <E5a as Sub<E5b>>::Output,
        <EPia as Sub<EPib>>::Output,
        <E3a as Sub<E3b>>::Output,
        <E2a as Sub<E2b>>::Output
    >;
    
    fn div(self, _: Ratio<E5b, EPib, E3b, E2b>) -> Self::Output {
        Ratio(PhantomData)
    }
}

pub trait Scaled: Sealed{}// 不含NoRatio
impl<Exp5: Integer, ExpPi:Integer, Exp3: Integer, Exp2: Integer> Sealed for Ratio<Exp5, ExpPi, Exp3, Exp2>{}

impl<Exp5: Integer, ExpPi: Integer, Exp3: Integer, Exp2: NonZero> Scaled for Ratio<Exp5, ExpPi, Exp3, Exp2>{}
impl<Exp5: Integer, ExpPi: Integer, Exp3: NonZero> Scaled for Ratio<Exp5, ExpPi, Exp3, Z0>{}
impl<Exp5: Integer, ExpPi: NonZero> Scaled for Ratio<Exp5, ExpPi, Z0, Z0>{}
impl<Exp5: NonZero> Scaled for Ratio<Exp5, Z0, Z0, Z0>{}

// ========== 常用比例定义 ==========

/// 单位1 (无比例)
pub type NoRatio = Ratio<Z0, Z0, Z0, Z0>;