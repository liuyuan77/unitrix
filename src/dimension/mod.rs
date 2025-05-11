//! Physical Units Library with Type-Level Dimension Checking
//! 带类型级量纲检查的物理单位库
//!
//! This module provides type-safe physical unit representations using Rust's type system
//! to enforce dimensional correctness at compile time.
//! 本模块提供类型安全的物理单位表示，利用Rust类型系统在编译时强制量纲正确性
//!
//! # Examples | 示例
//! ```
//! use physunits::Dimension;
//! use typenum::{P1, Z0, N1, N2};
//! use  physunits::Unitary;
//!
//! // Define velocity unit (m/s)
//! // 定义速度单位 (米/秒)
//! type Velocity = Dimension<P1, Z0, N1, Z0, Z0, Z0, Z0>;
//!
//! // Define acceleration unit (m/s²)
//! // 定义加速度单位 (米/秒²)
//! type Acceleration = Dimension<P1, Z0, N2, Z0, Z0, Z0, Z0>;
//! ```
//!

use typenum::{Integer, Sum, Diff, Prod};
use core::marker::PhantomData;
use core::ops::{Add, Sub, Mul, Div};

use super::unitary::Unitary;

mod dimensional;
pub use self::dimensional::Dimensional;

/// Fundamental structure representing physical units with dimensional exponents
/// 表示带有量纲指数的物理单位的基础结构
///
/// # Type Parameters | 类型参数
/// - `METER`: Length dimension exponent | 长度量纲指数
/// - `KILOGRAM`: Mass dimension exponent | 质量量纲指数  
/// - `SECOND`: Time dimension exponent | 时间量纲指数
/// - `AMPERE`: Current dimension exponent | 电流量纲指数
/// - `KELVIN`: Temperature dimension exponent | 温度量纲指数
/// - `MOLE`: Amount dimension exponent | 物质量量纲指数
/// - `CANDELA`: Luminous intensity dimension exponent | 光强量纲指数
///
/// # Example | 示例
/// ```
/// use typenum::{P1, Z0, N1, N2};
/// use physunits::Dimension;
/// 
/// // Represents m¹·s⁻¹ (velocity) | 表示 m¹·s⁻¹ (速度)
/// type VelocityUnit = Dimension<P1, Z0, N1, Z0, Z0, Z0, Z0>;
/// 
/// // Represents m·kg·s⁻² (force) | 表示 m·kg·s⁻² (力)
/// type ForceUnit = Dimension<P1, P1, N2, Z0, Z0, Z0, Z0>;
/// ```
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

impl<M: Integer, KG: Integer, S: Integer, A: Integer, K: Integer, MOL: Integer, CD: Integer>
Dimension<M, KG, S, A, K, MOL, CD> {
    /// Creates a new unit instance
    /// 创建新的单位实例
    pub fn new() -> Self {
        Self(PhantomData)
    }

     /// 将单位提升到幂次 `N`
     pub fn pow<N>(self) -> Dimension<
     Prod<M, N>,
     Prod<KG, N>,
     Prod<S, N>,
     Prod<A, N>,
     Prod<K, N>,
     Prod<MOL, N>,
     Prod<CD, N>,
 >
 where
     N: Integer,
     M: Mul<N>,
     KG: Mul<N>,
     S: Mul<N>,
     A: Mul<N>,
     K: Mul<N>,
     MOL: Mul<N>,
     CD: Mul<N>,
     Prod<M, N>: Integer,
     Prod<KG, N>: Integer,
     Prod<S, N>: Integer,
     Prod<A, N>: Integer,
     Prod<K, N>: Integer,
     Prod<MOL, N>: Integer,
     Prod<CD, N>: Integer,
 {
     Dimension::new()
 }
 
}

// ========== Operator Implementations ==========
// ========== 运算符实现 ==========

impl<M1, M2, KG1, KG2, S1, S2, A1, A2, K1, K2, MOL1, MOL2, CD1, CD2> 
    Mul<Dimension<M2, KG2, S2, A2, K2, MOL2, CD2>> for Dimension<M1, KG1, S1, A1, K1, MOL1, CD1>
where
    M1: Integer + Add<M2>,
    M2: Integer,
    KG1: Integer + Add<KG2>,
    KG2: Integer,
    S1: Integer + Add<S2>,
    S2: Integer,
    A1: Integer + Add<A2>,
    A2: Integer,
    K1: Integer + Add<K2>,
    K2: Integer,
    MOL1: Integer + Add<MOL2>,
    MOL2: Integer,
    CD1: Integer + Add<CD2>,
    CD2: Integer,
    Sum<M1, M2>: Integer,
    Sum<KG1, KG2>: Integer,
    Sum<S1, S2>: Integer,
    Sum<A1, A2>: Integer,
    Sum<K1, K2>: Integer,
    Sum<MOL1, MOL2>: Integer,
    Sum<CD1, CD2>: Integer,
{
    type Output = Dimension<
        Sum<M1, M2>, Sum<KG1, KG2>, Sum<S1, S2>,
        Sum<A1, A2>, Sum<K1, K2>, Sum<MOL1, MOL2>, Sum<CD1, CD2>
    >;
    
    /// Multiplies two units by adding their dimensional exponents
    /// 通过相加量纲指数来相乘两个单位
    fn mul(self, _: Dimension<M2, KG2, S2, A2, K2, MOL2, CD2>) -> Self::Output {
        Dimension::new()
    }
}

impl<M1, M2, KG1, KG2, S1, S2, A1, A2, K1, K2, MOL1, MOL2, CD1, CD2> 
    Div<Dimension<M2, KG2, S2, A2, K2, MOL2, CD2>> for Dimension<M1, KG1, S1, A1, K1, MOL1, CD1>
where
    M1: Integer + Sub<M2>,
    M2: Integer,
    KG1: Integer + Sub<KG2>,
    KG2: Integer,
    S1: Integer + Sub<S2>,
    S2: Integer,
    A1: Integer + Sub<A2>,
    A2: Integer,
    K1: Integer + Sub<K2>,
    K2: Integer,
    MOL1: Integer + Sub<MOL2>,
    MOL2: Integer,
    CD1: Integer + Sub<CD2>,
    CD2: Integer,
    Diff<M1, M2>: Integer,
    Diff<KG1, KG2>: Integer,
    Diff<S1, S2>: Integer,
    Diff<A1, A2>: Integer,
    Diff<K1, K2>: Integer,
    Diff<MOL1, MOL2>: Integer,
    Diff<CD1, CD2>: Integer,
{
    type Output = Dimension<
        Diff<M1, M2>, Diff<KG1, KG2>, Diff<S1, S2>,
        Diff<A1, A2>, Diff<K1, K2>, Diff<MOL1, MOL2>, Diff<CD1, CD2>
    >;
    
    /// Divides two units by subtracting their dimensional exponents
    /// 通过相减量纲指数来相除两个单位
    fn div(self, _: Dimension<M2, KG2, S2, A2, K2, MOL2, CD2>) -> Self::Output {
        Dimension::new()
    }
}

impl<L: Integer, M: Integer, T: Integer, I: Integer, Th: Integer, N: Integer, J: Integer>
    Unitary for Dimension<L, M, T, I, Th, N, J>{
}

/// 量纲除法结果类型 / Dimension division result type
pub type DimensionDiv<A, B> = <A as Div<B>>::Output;

/// 量纲乘法结果类型 / Dimension multiplication result type
pub type DimensionMul<A, B> = <A as Mul<B>>::Output;

// ========== Debugging and Testing Code ==========
// ========== 调试和测试代码 ==========

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{P1, Z0, N1, N2};
    
    /// Test unit multiplication
    /// 测试单位乘法
    #[test]
    fn test_unit_multiplication() {
        type Meter = Dimension<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
        type Second = Dimension<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
        type Velocity = DimensionMul<Meter, Dimension<Z0, Z0, N1, Z0, Z0, Z0, Z0>>;
        
        let _meter = Meter::new();
        let _second = Second::new();
        let _velocity: Velocity = Meter::new().mul(Second::new().div(Second::new()).div(Second::new()));
        
        // Compile-time check that velocity has correct dimensions (m/s)
        // 编译时检查速度有正确的量纲 (米/秒)
        let _: Dimension<P1, Z0, N1, Z0, Z0, Z0, Z0> = _velocity;
    }
    
    /// Test unit division
    /// 测试单位除法
    #[test]
    fn test_unit_division() {
        type Meter = Dimension<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
        type Second = Dimension<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
        type Velocity = DimensionDiv<Meter, Second>;
        
        let _velocity: Velocity = Meter::new().div(Second::new());
        
        // Compile-time check that velocity has correct dimensions (m/s)
        // 编译时检查速度有正确的量纲 (米/秒)
        let _: Dimension<P1, Z0, N1, Z0, Z0, Z0, Z0> = _velocity;
    }
    
    /// Test compound units
    /// 测试复合单位
    #[test]
    fn test_compound_units() {
        type Meter = Dimension<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
        type Kilogram = Dimension<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
        type Second = Dimension<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
        
        // Force = kg·m/s²
        // 力 = 千克·米/秒²
        type Force = DimensionMul<Kilogram, DimensionDiv<Meter, DimensionMul<Second, Second>>>;
        
        let _force: Force = Kilogram::new().mul(Meter::new().div(Second::new().mul(Second::new())));
        
        // Compile-time check that force has correct dimensions (kg·m/s²)
        // 编译时检查力有正确的量纲 (千克·米/秒²)
        let _: Dimension<P1, P1, N2, Z0, Z0, Z0, Z0> = _force;
    }
}