//! SI基础结构
//! 
//! 支持单位自动推导和SI前缀转换
//! 
use core::marker::PhantomData;
use core::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

use crate::sealed::Sealed;
use crate::constant::Prod;
use super::Dimensional;
use super::prefix::Prefixed;
use crate::variable::{Numeric, Scalar, Var};
use super::Unitary;
use super::Unit;
use super::ratio::{NoRatio, Scaled};
/// SI基础结构
/// 
/// # 类型参数
/// - `Pr`: SI前缀类型
/// - `D`: 量纲类型
#[derive(Debug, Clone, Copy)]
pub struct Si<
    Value: Scalar,
    D:Dimensional,
    Pr:Prefixed,
>(
    pub Value,
    pub PhantomData<(D, Pr)>
);

// ========== 构造函数 ==========

impl<T, D, Pr> Si<Var<T>, D, Pr>
where 
    T: Numeric,
    D: Dimensional,
    Pr: Prefixed,
    Var<T>: Scalar,
{
    /// 创建新的SI量
    pub fn new(value: T) -> Self {
        Si(Var(value),PhantomData)
    }
}

// ========== trait实现 ==========

impl<V: Scalar, Pr: Prefixed, D: Dimensional> Sealed for Si<V, D, Pr>{}
impl<V: Scalar, Pr: Prefixed, D: Dimensional> Unitary for Si<V, D, Pr>{}

/// 标记trait
pub trait Sied: Sealed{}
impl<V: Scalar, Pr: Prefixed, D: Dimensional> Sied for Si<V, D, Pr>{}

// ========== 运算符重载 ==========

// ----- 取负运算符 -----

impl<V: Scalar, D: Dimensional, Pr: Prefixed> Neg for Si<V, D, Pr>
where
    V: Neg<Output = V>,
{
    type Output = Self;
    
    /// 取负运算（保持前缀和量纲不变）
    fn neg(self) -> Self::Output {
        Si(-self.0, PhantomData)
    }
}

// ----- 加法运算符及加法赋值 -----
// Si + Si
impl<V, D, Pr> Add for Si<V, D, Pr>
where
    V: Scalar + Add<V, Output = V>,
    D: Dimensional,
    Pr: Prefixed,
{
    type Output = Self;
    
    /// 加法（要求相同前缀和量纲）
    fn add(self, rhs: Self) -> Self::Output {
        Si(self.0 + rhs.0, PhantomData)
    }
}

// Si += Si
impl<V: Scalar, D: Dimensional, Pr: Prefixed> AddAssign for Si<V, D, Pr>
where
    V: AddAssign<V>,
{
    /// 加法赋值（要求相同前缀和量纲）
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

// Si += T
impl<T, D, Pr> AddAssign<T> for Si<Var<T>, D, Pr>
where
    T: Numeric,
    D: Dimensional ,
    Pr: Prefixed,
    Var<T>: Scalar + AddAssign<Var<T>>,
{
    /// 标量加法赋值 (+=)
    fn add_assign(&mut self, rhs: T) {
        self.0 += Var(rhs);
    }
}

// Si += Var<T>
impl<T, D, Pr> AddAssign<Var<T>> for Si<Var<T>, D, Pr>
where
    T: Numeric,
    D: Dimensional,
    Pr: Prefixed,
    Var<T>: Scalar + AddAssign<Var<T>>,
{
    fn add_assign(&mut self, rhs: Var<T>) {
        self.0 += rhs;
    }
}

// ----- 减法运算符及减法赋值 -----

// Si - Si
impl<V, D, Pr> Sub for Si<V, D, Pr>
where
    V: Scalar + Sub<V, Output = V>,
    D: Dimensional,
    Pr: Prefixed,
{
    type Output = Self;
    
    /// 减法（要求相同前缀和量纲）
    fn sub(self, rhs: Self) -> Self::Output {
        Si(self.0 - rhs.0, PhantomData)
    }
}

// Si -= Si
impl<V: Scalar, D: Dimensional, Pr: Prefixed> SubAssign for Si<V, D, Pr>
where
    V: SubAssign<V>,
{
    /// 减法赋值（要求相同前缀和量纲）
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

// Si -= T
impl<T, D, Pr> SubAssign<T> for Si<Var<T>, D, Pr>
where
    T: Numeric,
    D: Dimensional ,
    Pr: Prefixed,
    Var<T>: Scalar + SubAssign<Var<T>>,
{
    /// 标量加法赋值 (+=)
    fn sub_assign(&mut self, rhs: T) {
        self.0 -= Var(rhs);
    }
}

// Si -= Var<T>
impl<T, D, Pr> SubAssign<Var<T>> for Si<Var<T>, D, Pr>
where
    T: Numeric,
    D: Dimensional,
    Pr: Prefixed,
    Var<T>: Scalar + SubAssign<Var<T>>,
{
    fn sub_assign(&mut self, rhs: Var<T>) {
        self.0 -= rhs;
    }
}

// ----- 乘法运算符及乘法赋值 -----
// Si * Si
impl<V, D1, D2, Pr1, Pr2> Mul<Si<V, D2, Pr2>> for Si<V, D1, Pr1>
where
    V: Scalar + Mul<V,Output: Scalar>,
    D1: Dimensional + Mul<D2,Output: Dimensional>,
    D2: Dimensional,
    Pr1: Prefixed + Mul<Pr2,Output: Prefixed>,
    Pr2: Prefixed,
{
    type Output = Si< <V as Mul>::Output, Prod<D1, D2>, Prod<Pr1, Pr2> >;
    
    /// 乘法（量纲相乘，前缀相加）
    fn mul(self, rhs: Si<V, D2, Pr2>) -> Self::Output {
        Si(self.0 * rhs.0, PhantomData)
    }
}

// Si * T
impl<T, D, Pr> Mul<T> for Si<Var<T>, D, Pr>
where
    T:Numeric,
    Var<T>: Scalar + Mul<Var<T>,Output: Scalar>,
    D: Dimensional,
    Pr: Prefixed,
{
    type Output = Si< <Var<T> as Mul>::Output, D, Pr >;
    
    /// 乘法（量纲、前缀不变）
    fn mul(self, rhs: T) -> Self::Output {
        Si(self.0 * Var(rhs), PhantomData)
    }
}

// Si * Var<T>
impl<T, D, Pr> Mul<Var<T>> for Si<Var<T>, D, Pr>
where
    T:Numeric,
    Var<T>: Scalar + Mul<Var<T>,Output: Scalar>,
    D: Dimensional,
    Pr: Prefixed,
{
    type Output = Si< <Var<T> as Mul>::Output, D, Pr >;
    
    /// 乘法（量纲、前缀不变）
    fn mul(self, rhs: Var<T>) -> Self::Output {
        Si(self.0 * rhs, PhantomData)
    }
}

// Si *= T
impl<T, D, Pr> MulAssign<T> for Si<Var<T>, D, Pr>
where
    T: Numeric,
    D: Dimensional,
    Pr: Prefixed,
    Var<T>: Scalar + MulAssign<Var<T>>,
{
    /// 标量乘法赋值 (*=)
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= Var(rhs);
    }
}

// Si *= Var<T>
impl<T, D, Pr> MulAssign<Var<T>> for Si<Var<T>, D, Pr>
where
    T: Numeric,
    D: Dimensional,
    Pr: Prefixed,
    Var<T>: Scalar + MulAssign<Var<T>>,
{
    fn mul_assign(&mut self, rhs: Var<T>) {
        self.0 *= rhs;
    }
}

// Si * Unit
//因为编译器要求，需要拆解Si
impl<S, R, T, D, Pr> Mul<Unit<S, R>> for Si<Var<T>, D, Pr>
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
    fn mul(self, rhs: Unit<S, R>) -> Self::Output {
        Unit(rhs.0 * self, PhantomData)
    }
}

// ----- 除法运算符及除法赋值 -----
// Si / Si
impl<V, D1, D2, Pr1, Pr2> Div<Si<V, D2, Pr2>> for Si<V, D1, Pr1>
where
    V: Scalar + Div<V,Output: Scalar>,
    D1: Dimensional + Div<D2,Output: Dimensional>,
    D2: Dimensional,
    Pr1: Prefixed + Div<Pr2,Output: Prefixed>,
    Pr2: Prefixed,
{
    type Output = Si< <V as Div>::Output, <D1 as Div<D2>>::Output, <Pr1 as Div<Pr2>>::Output >;
    
    /// 除法
    fn div(self, rhs: Si<V, D2, Pr2>) -> Self::Output {
        Si(self.0 / rhs.0, PhantomData)
    }
}

// Si /= T
impl<T, D, Pr> DivAssign<T> for Si<Var<T>, D, Pr>
where
    T: Numeric,
    D: Dimensional,
    Pr: Prefixed,
    Var<T>: Scalar + DivAssign<Var<T>>,
{
    /// 标量除法赋值 (/=)
    fn div_assign(&mut self, rhs: T) {
        self.0 /= Var(rhs);
    }
}

// Si /= Var<T>
impl<T, D, Pr> DivAssign<Var<T>> for Si<Var<T>, D, Pr>
where
    T: Numeric,
    D: Dimensional,
    Pr: Prefixed,
    Var<T>: Scalar + DivAssign<Var<T>>,
{
    fn div_assign(&mut self, rhs: Var<T>) {
        self.0 /= rhs;
    }
}

// Si / U
//因为编译器对Si要求，必须拆解后打包
impl<T, D1, Pr1, D2, Pr2, R> Div<Unit<Si<Var<T>, D2, Pr2>, R>> for Si<Var<T>, D1, Pr1>
where
    T: Numeric,
    Var<T>: Scalar,
    D1: Dimensional,
    D2: Dimensional,
    Pr1: Prefixed,
    Pr2: Prefixed,
    R: Scaled,
    NoRatio: Div<R>,
    Si<Var<T>, D1, Pr1>: Div<Si<Var<T>, D2, Pr2>,Output: Sied>,
{
    type Output = Unit<
        < Si<Var<T>, D1, Pr1> as Div<Si<Var<T>, D2, Pr2>> >::Output,
        <NoRatio as Div<R>>::Output,
    >;
    
    /// 物理量除法
    fn div(self, rhs: Unit<Si<Var<T>, D2, Pr2>, R>) -> Self::Output {
        Unit(self / rhs.0, PhantomData)
    }
}