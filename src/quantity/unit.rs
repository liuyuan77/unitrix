//! Unit基础结构
//! 
//! 支持单位自动推导

use core::marker::PhantomData;
use core::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

use crate::sealed::Sealed;
use super::{Si, Sied};
use super::ratio::{NoRatio, Scaled};
use super::Dimensional;
use super::prefix::Prefixed;
use crate::number::{Numeric, Scalar, Var};
use super::Unitary;

// ========== 辅助trait和实现 ==========
// 辅助trait，用于规范类型
pub trait UnitOrSi {
    type Output;
    fn unit(self) -> Self::Output;
}

// 为所有Prefixed类型实现UnitOrSi
impl<S: Sied, R: Scaled> UnitOrSi for Unit<S, R> {
    type Output = Self;
    fn unit(self) -> Self::Output{
        self
    }
}

impl<S: Sied> UnitOrSi for Unit<S, NoRatio> {
    type Output = S;
    fn unit(self) -> Self::Output{
        self.0
    }
}

/// Unit基础结构
/// 
/// # 类型参数
/// - `R`: 比例因子类型
/// - `S`: SI基础类型
#[derive(Debug, Clone, Copy)]
pub struct Unit<S: Sied, R>(pub S,pub PhantomData<R>);

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

// ----- 取负运算符 -----
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

// ----- 加法运算符及其赋值 -----
// U + U
impl<S: Sied, R: Scaled> Add for Unit<S, R>
where
    S:  Sied + Add<S, Output = S>,
    R: Scaled,
{
    type Output = Self;
    
    /// 物理量乘法
    fn add(self, rhs: Self) -> Self::Output {
        Unit(self.0 + rhs.0, PhantomData)
    }
}

// U += U
impl<S: Sied, R: Scaled> AddAssign for Unit<S, R>
where
    S:  Sied + AddAssign,
    R: Scaled,
{
    /// 加法赋值
    fn add_assign(&mut self, rhs: Self){
        self.0 += rhs.0;
    }
}

// U += T
impl<T:Numeric, S: Sied, R: Scaled> AddAssign<T> for Unit<S, R>
where
    S:  Sied + AddAssign<T>,
    R: Scaled,
{
    /// 加法赋值
    fn add_assign(&mut self, rhs: T){
        self.0 += rhs;
    }
}

// U += Var<T>
impl<T:Numeric, S: Sied, R: Scaled> AddAssign<Var<T>> for Unit<S, R>
where
    S:  Sied + AddAssign<Var<T>>,
    R: Scaled,
{
    /// 加法赋值
    fn add_assign(&mut self, rhs: Var<T>){
        self.0 += rhs;
    }
}

// ----- 减法运算符及减法赋值 -----
// U - U
impl<S: Sied, R: Scaled> Sub for Unit<S, R>
where
    S:  Sied + Sub<S, Output = S>,
    R: Scaled,
{
    type Output = Self;
    
    /// 物理量乘法
    fn sub(self, rhs: Self) -> Self::Output {
        Unit(self.0 - rhs.0, PhantomData)
    }
}

// U -= U
impl<S: Sied, R: Scaled> SubAssign for Unit<S, R>
where
    S:  Sied + SubAssign,
    R: Scaled,
{
    /// 加法赋值
    fn sub_assign(&mut self, rhs: Self){
        self.0 -= rhs.0;
    }
}

// U -= T
impl<T:Numeric, S: Sied, R: Scaled> SubAssign<T> for Unit<S, R>
where
    S:  Sied + SubAssign<T>,
    R: Scaled,
{
    /// 加法赋值
    fn sub_assign(&mut self, rhs: T){
        self.0 -= rhs;
    }
}

// U -= Var<T>
impl<T:Numeric, S: Sied, R: Scaled> SubAssign<Var<T>> for Unit<S, R>
where
    S:  Sied + SubAssign<Var<T>>,
    R: Scaled,
{
    /// 加法赋值
    fn sub_assign(&mut self, rhs: Var<T>){
        self.0 -= rhs;
    }
}

// ----- 乘法运算符及乘法赋值 -----

// U * U
impl<S1, S2, R1, R2> Mul<Unit<S2, R2>> for Unit<S1, R1>
where
    S1: Sied + Mul<S2, Output: Sied>,
    S2: Sied,
    R1: Scaled + Mul<R2, Output: Scaled>,
    R2: Scaled,
    Unit<
        <S1 as Mul<S2>>::Output,
        <R1 as Mul<R2>>::Output
    >: UnitOrSi,
{
    type Output = <
        Unit<
            <S1 as Mul<S2>>::Output,
            <R1 as Mul<R2>>::Output
        > as UnitOrSi
    >::Output;
    
    /// 物理量乘法
    fn mul(self, rhs: Unit<S2, R2>) -> Self::Output {
        Unit(self.0 * rhs.0, PhantomData).unit()
    }
}

// U * SI
//因为编译器对U * T与U * Si无法区分，Si必须用Si<Var<T>, D, Pr>表示
impl<S, R, T, D, Pr> Mul<Si<Var<T>, D, Pr>> for Unit<S, R>
where
    T: Numeric,
    Var<T>: Scalar,
    D: Dimensional,
    Pr: Prefixed,
    S: Sied + Mul<Si<Var<T>, D, Pr>, Output: Sied>,
    Si<Var<T>, D, Pr>: Sied,
    R: Scaled,
{
    type Output = Unit<
        <S as Mul<Si<Var<T>, D, Pr>>>::Output,  // 单位相乘
        R
    >;
    
    /// 物理量乘法
    fn mul(self, rhs: Si<Var<T>, D, Pr>) -> Self::Output {
        Unit(self.0 * rhs, PhantomData)
    }
}

// U * Var<T>
impl<S, R, T> Mul<Var<T>> for Unit<S, R>
where
    T:Numeric,
    Var<T>: Scalar,
    S: Sied + Mul<Var<T>, Output: Sied>,
    R: Scaled,
{
    type Output = Unit<
        <S as Mul<Var<T>>>::Output,
        R
    >;
    
    /// 物理量乘法
    fn mul(self, rhs: Var<T>) -> Self::Output {
        Unit(self.0 * rhs, PhantomData)
    }
}

// U * T
impl<T, S, R> Mul<T> for Unit<S, R>
where
    T: Numeric,
    Var<T>: Scalar,
    S: Sied + Mul<T, Output: Sied>,
    R: Scaled,
{
    type Output = Unit<
        <S as Mul<T>>::Output,
        R
    >;
    
    /// 物理量乘法
    fn mul(self, rhs: T) -> Self::Output {
        Unit(self.0 * rhs, PhantomData)
    }
}

// U *= Var<T>
impl<T, S, R> MulAssign<Var<T>> for Unit<S, R>
where
    T: Numeric,
    S: Sied + MulAssign<Var<T>>,
    R: Scaled,
    Var<T>: Scalar,
{
    /// 标量乘法赋值 (*=)
    fn mul_assign(&mut self, rhs: Var<T>) {
        self.0 *= rhs;
    }
}

// U *= T
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

// ----- 除法运算符及除法赋值 -----

// U / U
impl<S1, S2, R1, R2> Div<Unit<S2, R2>> for Unit<S1, R1>
where
    S1: Sied + Div<S2, Output: Sied>,
    S2: Sied,
    R1: Scaled + Div<R2, Output: Scaled>,
    R2: Scaled,
    Unit<
        <S1 as Div<S2>>::Output,  // 单位相除
        <R1 as Div<R2>>::Output
    >: UnitOrSi,
{
    type Output = <
        Unit<
            <S1 as Div<S2>>::Output,  // 单位相除
            <R1 as Div<R2>>::Output
        > as UnitOrSi
    >::Output;
    
    /// 物理量除法
    fn div(self, rhs: Unit<S2, R2>) -> Self::Output {
        Unit(self.0 / rhs.0, PhantomData).unit()
    }
}

// U / SI
//因为编译器对U / T与U / Si无法区分，Si必须用Si<Var<T>, D, Pr>表示
impl<S, R, T, D, Pr> Div<Si<Var<T>, D, Pr>> for Unit<S, R>
where
    T: Numeric,
    Var<T>: Scalar,
    D: Dimensional,
    Pr: Prefixed,
    S: Sied + Div<Si<Var<T>, D, Pr>, Output: Sied>,
    Si<Var<T>, D, Pr>: Sied,
    R: Scaled,
{
    type Output = Unit<
        <S as Div<Si<Var<T>, D, Pr>>>::Output,  // 单位相除
        R
    >;
    
    /// 物理量除法
    fn div(self, rhs: Si<Var<T>, D, Pr>) -> Self::Output {
        Unit(self.0 / rhs, PhantomData)
    }
}

// U / Var<T>
impl<S, R, T> Div<Var<T>> for Unit<S, R>
where
    T:Numeric,
    Var<T>: Scalar,
    S: Sied + Div<Var<T>, Output: Sied>,
    R: Scaled,
{
    type Output = Unit<
        <S as Div<Var<T>>>::Output,
        R
    >;
    
    /// 物理量除法
    fn div(self, rhs: Var<T>) -> Self::Output {
        Unit(self.0 / rhs, PhantomData)
    }
}

// U / T
impl<T, S, R> Div<T> for Unit<S, R>
where
    T: Numeric,
    Var<T>: Scalar,
    S: Sied + Div<T, Output: Sied>,
    R: Scaled,
{
    type Output = Unit<
        <S as Div<T>>::Output,
        R
    >;
    
    /// 物理量除法
    fn div(self, rhs: T) -> Self::Output {
        Unit(self.0 / rhs, PhantomData)
    }
}

// U /= Var<T>
impl<T, S, R> DivAssign<Var<T>> for Unit<S, R>
where
    T: Numeric,
    S: Sied + DivAssign<Var<T>>,
    R: Scaled,
    Var<T>: Scalar,
{
    /// 标量除法赋值 (/=)
    fn div_assign(&mut self, rhs: Var<T>) {
        self.0 /= rhs;
    }
}

// U /= T
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