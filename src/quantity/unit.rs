//! Unit基础结构
//! 
//! 支持单位自动推导

use core::marker::PhantomData;
use core::ops::{Neg, Add, Sub, Mul, Div, MulAssign, DivAssign};

use crate::sealed::Sealed;
use crate::constant::{Sum, Diff};
use super::{Si, Sied};
use super::ratio::Scaled;
use super::Dimensional;
use super::prefix::Prefixed;
use crate::variable::{Numeric, Scalar, Var};
use super::Unitary;

/// Unit基础结构
/// 
/// # 类型参数
/// - `R`: 比例因子类型
/// - `S`: SI基础类型
#[derive(Debug, Clone, Copy)]
pub struct Unit<S: Sied, R: Scaled>(pub S,pub PhantomData<R>);

impl<T, D, Pr, R> Unit<Si<Var<T>, D, Pr>, R>
where 
    T: Numeric,
    D: Dimensional,
    Pr: Prefixed,
    Var<T>: Scalar,
    R: Scaled,
{
    pub fn new(value: T) -> Self {
        Self(Si::new(value),PhantomData)
    }
}

impl< S: Sied, R: Scaled> Sealed for Unit<S, R>{}

impl<S: Sied, R: Scaled> Unitary for Unit<S, R>{}

// ================ 运算实现 ================

impl<S1:Sied, S2:Sied, R1: Scaled, R2: Scaled> Mul<Unit<S2, R2>> for Unit<S1, R1>
where
    R1: Scaled + Add<R2, Output: Scaled>,
    R2: Scaled,
    S1: Mul<S2, Output: Sied>,
    Sum<R1, R2>: Scaled, 
{
    type Output = Unit<
        <S1 as Mul<S2>>::Output,  // 单位相乘
        Sum<R1, R2>
    >;
    
    /// 物理量乘法
    fn mul(self, rhs: Unit<S2, R2>) -> Self::Output {
        Unit(self.0 * rhs.0,PhantomData)
    }
}

impl<R1: Scaled, R2: Scaled, S1:Sied, S2:Sied> Div<Unit<S2, R2>> for Unit<S1, R1>
where
    R1: Sub<R2, Output: Scaled>,  
    S1: Div<S2, Output: Sied>,
    //Diff<R1, R2>: Scaled,
{
    type Output = Unit<
        <S1 as Div<S2>>::Output,
        Diff<R1, R2>  // 相减
    >;
    
    /// 物理量除法
    fn div(self, rhs: Unit<S2, R2>) -> Self::Output {
        Unit(self.0 / rhs.0, PhantomData)
    }
}

// ========== 运算符重载 ==========

// ----- 一元运算符 -----
impl<S: Sied, R: Scaled> Neg for Unit<S, R>
where
    S: Neg<Output = S>,
{
    type Output = Self;
    
    /// 取负运算（保持单位和比例因子不变）
    fn neg(self) -> Self::Output {
        Unit(-self.0, PhantomData)
    }
}

// ----- 与标量的运算 -----
impl<T, S, R> Mul<T> for Unit<S, R>
where
    T: Numeric,
    S: Sied + Mul<Var<T>, Output = S>,
    R: Scaled,
    Var<T>: Scalar,
{
    type Output = Self;
    
    /// 标量乘法（保持单位和比例因子不变）
    fn mul(self, rhs: T) -> Self::Output {
        Unit(self.0 * Var(rhs), PhantomData)
    }
}

impl<T, S, R> Div<T> for Unit<S, R>
where
    T: Numeric,
    S: Sied + Div<Var<T>, Output = S>,
    R: Scaled,
    Var<T>: Scalar,
{
    type Output = Self;
    
    /// 标量除法（保持单位和比例因子不变）
    fn div(self, rhs: T) -> Self::Output {
        Unit(self.0 / Var(rhs), PhantomData)
    }
}

//Var
impl<T, S, R> Mul<Var<T>> for Unit<S, R>
where
    T: Numeric,
    S: Sied + Mul<Var<T>, Output = S>,
    R: Scaled,
    Var<T>: Scalar,
{
    type Output = Self;
    
    /// Var<T>乘法（保持单位和比例因子不变）
    fn mul(self, rhs: Var<T>) -> Self::Output {
        Unit(self.0 * rhs, PhantomData)
    }
}

impl<T, S, R> Div<Var<T>> for Unit<S, R>
where
    T: Numeric,
    S: Sied + Div<Var<T>, Output = S>,
    R: Scaled,
    Var<T>: Scalar,
{
    type Output = Self;
    
    /// Var<T>除法（保持单位和比例因子不变）
    fn div(self, rhs: Var<T>) -> Self::Output {
        Unit(self.0 / rhs, PhantomData)
    }
}

impl<S1, S2, R> Mul<S2> for Unit<S1, R>
where
    S1: Sied + Mul<S2, Output = Sied>,
    S2: Sied,
    Pr: Prefixed,
    R: Scaled,
{
    type Output = Unit<<S1 as Mul<S2>>::Output, R>;
    
    /// 与Si的乘法（单位相乘，比例因子不变）
    fn mul(self, rhs: S2) -> Self::Output {
        Unit(self.0 * rhs, PhantomData)
    }
}

impl<S1, S2, Pr, R1, R2> Div<Si<S2, Pr>> for Unit<S1, R1>
where
    S1: Sied + Div<S2, Output = Sied>,
    S2: Dimensional,
    Pr: Prefixed,
    R1: Scaled + Sub<R2>,
    R2: Scaled,
    Diff<R1, R2>: Scaled,
{
    type Output = Unit<<S1 as Div<S2>>::Output, Diff<R1, R2>>;
    
    /// 与Si的除法（单位相除，比例因子相减）
    fn div(self, rhs: Si<S2, Pr>) -> Self::Output {
        Unit(self.0 / rhs, PhantomData)
    }
}

// ----- 赋值运算符 -----
//标量
impl<T, S, R> MulAssign<T> for Unit<S, R>
where
    T: Numeric,
    S: Sied + MulAssign<Var<T>>,
    R: Scaled,
    Var<T>: Scalar,
{
    /// 标量乘法赋值 (*=)
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= Var(rhs);
    }
}

impl<T, S, R> DivAssign<T> for Unit<S, R>
where
    T: Numeric,
    S: Sied + DivAssign<Var<T>>,
    R: Scaled,
    Var<T>: Scalar,
{
    /// 标量除法赋值 (/=)
    fn div_assign(&mut self, rhs: T) {
        self.0 /= Var(rhs);
    }
}

//Var<T>赋值运算
impl<T, S, R> MulAssign<Var<T>> for Unit<S, R>
where
    T: Numeric,
    S: Sied + MulAssign<Var<T>>,
    R: Scaled,
    Var<T>: Scalar,
{
    fn mul_assign(&mut self, rhs: Var<T>) {
        self.0 *= rhs;
    }
}

impl<T, S, R> DivAssign<Var<T>> for Unit<S, R>
where
    T: Numeric,
    S: Sied + DivAssign<Var<T>>,
    R: Scaled,
    Var<T>: Scalar,
{
    fn div_assign(&mut self, rhs: Var<T>) {
        self.0 /= rhs;
    }
}



