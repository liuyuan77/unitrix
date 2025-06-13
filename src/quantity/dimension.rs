//! Physical Units Library with Type-Level Dimension Checking
//! 带类型级量纲检查的物理单位库
//!
//! 文件位置: src/quantity/dimension.rs
//! 本文件定义物理量纲系统的核心结构
//!
//! This module provides type-safe physical unit representations using Rust's type system
//! to enforce dimensional correctness at compile time.
//! 本模块提供类型安全的物理单位表示，利用Rust类型系统在编译时强制量纲正确性

use crate::sealed::Sealed;
use crate::number::{Z0, Integer};
use core::marker::PhantomData;
use core::ops::{Add, Sub, Mul, Div};

/// Fundamental structure representing physical units with dimensional exponents
/// 表示带有量纲指数的物理单位的基础结构
///
/// 这是整个物理单位系统中最基础的结构，仅包含量纲信息，不包含任何词头(如kilo-, milli-等)。
/// 具体物理类型(如Meter, Second等)在其它文件中通过类型别名定义。
///
/// 注意：这是内部基础结构，用户代码应该使用其它文件中定义的具体物理类型。
///
/// # Type Parameters | 类型参数
/// - `METER`: Length dimension exponent (P1 for meter, Z0 for dimensionless) | 长度量纲指数
/// - `KILOGRAM`: Mass dimension exponent | 质量量纲指数  
/// - `SECOND`: Time dimension exponent | 时间量纲指数
/// - `AMPERE`: Current dimension exponent | 电流量纲指数
/// - `KELVIN`: Temperature dimension exponent | 温度量纲指数
/// - `MOLE`: Amount dimension exponent | 物质量量纲指数
/// - `CANDELA`: Luminous intensity dimension exponent | 光强量纲指数

#[derive(Debug, Clone, Copy)]
pub struct Dimension<
    METER: Integer,
    KILOGRAM: Integer,
    SECOND: Integer,
    AMPERE: Integer,
    KELVIN: Integer,
    MOLE: Integer,
    CANDELA: Integer
>(
    PhantomData<(METER, KILOGRAM, SECOND, AMPERE, KELVIN, MOLE, CANDELA)>
);

/// Trait marking valid Dimension types
/// 标记有效单位类型的Trait
///
/// # Safety 安全性
/// This trait is sealed and cannot be implemented outside this crate
/// 该trait是密封的，不能在本crate外实现
pub trait Dimensional: Sealed+Sized {}

impl<M: Integer, KG: Integer, S: Integer, A: Integer, K: Integer, MOL: Integer, CD: Integer>
    Sealed for Dimension<M, KG, S, A, K, MOL, CD>{
}

impl<M: Integer, KG: Integer, S: Integer, A: Integer, K: Integer, MOL: Integer, CD: Integer>
    Dimensional for Dimension<M, KG, S, A, K, MOL, CD>{
}

/// Creates a new unit instance
/// 创建新的单位实例
///
impl<M: Integer, KG: Integer, S: Integer, A: Integer, K: Integer, MOL: Integer, CD: Integer>
Dimension<M, KG, S, A, K, MOL, CD>{
    /// Creates a new unit instance
    /// 创建新的单位实例
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<M: Integer, KG: Integer, S: Integer, A: Integer, K: Integer, MOL: Integer, CD: Integer>
Dimension<M, KG, S, A, K, MOL, CD>{
    /// Raises the unit to the power of `N`
    /// 将单位提升到幂次 `N`
    pub fn pow<N>(self) -> Dimension<
        <M as Mul<N>>::Output,
        <KG as Mul<N>>::Output,
        <S as Mul<N>>::Output,
        <A as Mul<N>>::Output,
        <K as Mul<N>>::Output,
        <MOL as Mul<N>>::Output,
        <CD as Mul<N>>::Output,
    >
    where
        N: Integer,
        M: Mul<N, Output: Integer>,
        KG: Mul<N, Output: Integer>,
        S: Mul<N, Output: Integer>,
        A: Mul<N, Output: Integer>,
        K: Mul<N, Output: Integer>,
        MOL: Mul<N, Output: Integer>,
        CD: Mul<N, Output: Integer>,
    {
        Dimension::new()
    }
 
}

impl Default for Dimension<Z0, Z0, Z0, Z0, Z0, Z0, Z0> {
    fn default() -> Self {
        Self::new()  // 无量纲默认值
    }
}

// ========== Operator Implementations ==========
// ========== 运算符实现 ==========
impl<M1, M2, KG1, KG2, S1, S2, A1, A2, K1, K2, MOL1, MOL2, CD1, CD2> 
    Mul<Dimension<M2, KG2, S2, A2, K2, MOL2, CD2>> for Dimension<M1, KG1, S1, A1, K1, MOL1, CD1>
where
    M1: Integer + Add<M2, Output:Integer>,
    M2: Integer,
    KG1: Integer + Add<KG2, Output:Integer>,
    KG2: Integer,
    S1: Integer + Add<S2, Output:Integer>,
    S2: Integer,
    A1: Integer + Add<A2, Output:Integer>,
    A2: Integer,
    K1: Integer + Add<K2, Output:Integer>,
    K2: Integer,
    MOL1: Integer + Add<MOL2, Output:Integer>,
    MOL2: Integer,
    CD1: Integer + Add<CD2, Output:Integer>,
    CD2: Integer,
{
    type Output = Dimension<
        <M1 as Add<M2>>::Output,
        <KG1 as Add<KG2>>::Output,
        <S1 as Add<S2>>::Output,
        <A1 as Add<A2>>::Output,
        <K1 as Add<K2>>::Output,
        <MOL1 as Add<MOL2>>::Output,
        <CD1 as Add<CD2>>::Output
    >;
    
    /// Multiplies two units by adding their dimensional exponents
    /// 通过相加量纲指数来相乘两个单位
    fn mul(self, _rhs: Dimension<M2, KG2, S2, A2, K2, MOL2, CD2>) -> Self::Output {
        Dimension::new()
    }
}

impl<M1, M2, KG1, KG2, S1, S2, A1, A2, K1, K2, MOL1, MOL2, CD1, CD2> 
    Div<Dimension<M2, KG2, S2, A2, K2, MOL2, CD2>> for Dimension<M1, KG1, S1, A1, K1, MOL1, CD1>
where
    M1: Integer + Sub<M2, Output: Integer>,
    M2: Integer,
    KG1: Integer + Sub<KG2, Output: Integer>,
    KG2: Integer,
    S1: Integer + Sub<S2, Output: Integer>,
    S2: Integer,
    A1: Integer + Sub<A2, Output: Integer>,
    A2: Integer,
    K1: Integer + Sub<K2, Output: Integer>,
    K2: Integer,
    MOL1: Integer + Sub<MOL2, Output: Integer>,
    MOL2: Integer,
    CD1: Integer + Sub<CD2, Output: Integer>,
    CD2: Integer,
{
    type Output = Dimension<
        <M1 as Sub<M2>>::Output,
        <KG1 as Sub<KG2>>::Output,
        <S1 as Sub<S2>>::Output,
        <A1 as Sub<A2>>::Output,
        <K1 as Sub<K2>>::Output,
        <MOL1 as Sub<MOL2>>::Output,
        <CD1 as Sub<CD2>>::Output,
    >;
    
    /// Divides two units by subtracting their dimensional exponents
    /// 通过相减量纲指数来相除两个单位
    fn div(self, _rhs: Dimension<M2, KG2, S2, A2, K2, MOL2, CD2>) -> Self::Output {
        Dimension::new()
    }
}