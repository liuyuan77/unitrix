//! Physical Quantity Calculation Library
//! 物理量计算库
//!
//! Provides type-safe physical quantity calculations with automatic unit deduction and SI prefix conversion.
//! 提供类型安全的物理量计算功能，支持单位自动推导和SI前缀转换

// use core::marker::PhantomData;
// use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};
// use crate::variable::{Numeric, Scalar, Var};

mod unitary;
pub use unitary::Unitary;

mod prefix;
pub use prefix::*;

mod dimension;
pub use dimension::*;

mod si;
pub use si::*;

mod ratio;
pub use ratio::*;

mod unit;
pub use unit::*;

/* //????????????????????????????????????????????/
/// Base structure for physical quantities (without type constraints)
/// 物理量基础结构（无类型约束）
/// 
/// # Type Parameters / 类型参数
/// - `V`: Value storage type / 存储的数值类型
/// - `U`: Unit type / 单位类型
#[derive(Debug, Clone, Copy)]
pub struct Quantity<S: Scalar, U: Unitary>(pub S,PhantomData<U>);

impl<T: Numeric, U: Unitary> Quantity<Var<T>, U>
where
    Var<T>:Scalar,
{
    pub fn new(value: T) -> Self {
        Quantity(Var(value), PhantomData)
    }
}

impl<S1: Scalar, S2: Scalar, U: Unitary> Add<Quantity<S2, U>> for Quantity<S1, U>
where
    S1: Add<S2, Output = S1>,  // 确保 S1 和 S2 可以相加，且输出类型是 S1
    S1: From<S2>,     // 确保 S2 可以转换为 S1
{
    type Output = Self;  // 输出类型保持和左操作数一致
    
    fn add(self, rhs: Quantity<S2, U>) -> Self::Output {
        // 转换 rhs 的 Var<T2> 为 Var<T1>，然后相加
        Quantity(self.0 + S1::from(rhs.0), PhantomData)
    }
}


impl<S1: Scalar, S2: Scalar, U: Unitary> Sub<Quantity<S2, U>> for Quantity<S1, U>
where
    S1: Sub<S2, Output = S1>,      // 确保底层类型可减
    S1: From<S2>,        // 确保类型可转换
{
    type Output = Self; // 输出类型与左操作数一致

    fn sub(self, rhs: Quantity<S2, U>) -> Self::Output {
        Quantity(self.0 - S1::from(rhs.0), PhantomData)
    }
}

impl<S1: Scalar, S2: Scalar, U: Unitary> AddAssign<Quantity<S2, U>> for Quantity<S1, U>
where
    S1: AddAssign<S2>,      // 确保底层类型支持 +=
    S1: From<S2>, // 确保包装类型可转换
{
    fn add_assign(&mut self, rhs: Quantity<S2, U>) {
        self.0 += S1::from(rhs.0);
    }
}

impl<S1: Scalar, S2: Scalar, U: Unitary> SubAssign<Quantity<S2, U>> for Quantity<S1, U>
where
    S1: SubAssign<S2>,      // 确保底层类型支持 -=
    S1: From<S2>, // 确保包装类型可转换
{
    fn sub_assign(&mut self, rhs: Quantity<S2, U>) {
        self.0 -= S1::from(rhs.0);  // 调用 Var<T> 的 -=
    }
}

impl<S1: Scalar, S2: Scalar, U1: Unitary, U2: Unitary> Mul<Quantity<S2, U2>> for Quantity<S1, U1>
where
    S1: Mul<S2,Output = S1>,
    <S1 as Mul<S2>>::Output: Scalar,
    U1: Mul<U2>,
    <U1 as Mul<U2>>::Output: Dimensional,
{
    type Output = Quantity<S1, <U1 as Mul<U2>>::Output>;
    
    fn mul(self, rhs: Quantity<S2, U2>) -> Self::Output {
        Quantity(self.0 * rhs.0, PhantomData) // 需要定义单位相乘逻辑
    }
}




？？？？？？？？？？？？？？？？？？？？？？？？？？
impl<V1, V2, U1, U2> Div<Quantity<V2, U2>> for Quantity<V1, U1>
where
    V1: Div<V2>,
    U1: Units + Div<U2>,
    U2: Units,
    <V1 as Div<V2>>::Output: Clone,
    <U1 as Div<U2>>::Output: Units,
{
    type Output = Quantity<
        <V1 as Div<V2>>::Output,
        <U1 as Div<U2>>::Output
    >;
    
    /// Physical quantity division / 物理量除法
    /// 
    /// # Note / 注意
    /// Automatically handles unit and prefix division / 自动处理单位和前缀的除法关系
    fn div(self, rhs: Quantity<V2, U2>) -> Self::Output {
        Quantity {
            value: self.value / rhs.value,
            _marker: PhantomData,
        }
    }
}

// ========== Quantity * Unit Implementation ==========
// ========== Quantity * Unit 实现 ==========

impl<V,U1, U2> Mul<U2> for Quantity<V, U1>
where
    V: Clone,
    U1: Units + Mul<U2>,
    U2: Units,
    <U1 as Mul<U2>>::Output: Units,
{
    type Output = Quantity<V, <U1 as Mul<U2>>::Output>;
    
    /// Multiplies quantity by unit (type-level operation) / 物理量乘以单位（类型级别操作）
    fn mul(self, _: U2) -> Self::Output {
        Quantity {
            value: self.value,
            _marker: PhantomData,
        }
    }
}

// ========== Quantity / Unit Implementation ==========
// ========== Quantity / Unit 实现 ==========

impl<V, U1, U2> Div<U2> for Quantity<V,  U1>
where
    V: Clone,
    U1: Units + Div<U2>,
    U2: Units,
    <U1 as Div<U2>>::Output: Units,
{
    type Output = Quantity<V, <U1 as Div<U2>>::Output>;
    
    /// Divides quantity by unit (type-level operation) / 物理量除以单位（类型级别操作）
    fn div(self, _: U2) -> Self::Output {
        Quantity {
            value: self.value,
            _marker: PhantomData,
        }
    }
}

impl<V, FromPrefix, ToPrefix, U> 
    From<Quantity<V, FromPrefix, U>> 
    for Quantity<V, ToPrefix, U>
where
    V: Clone + From<i64> + Div<Output = V>,
    FromPrefix: Prefix,
    ToPrefix: Prefix,
    U: Units,

{
    fn from(src: Quantity<V, ToPrefix, U>) -> Self {
        Quantity {
            value: src.value * PrefixDiv<FromPrefix, ToPrefix>:factor(),
            _marker: PhantomData,
        }
    }
} */