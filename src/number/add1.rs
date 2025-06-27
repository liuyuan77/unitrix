//! 加一操作特质实现 / Increment operation trait implementation
//!
//! 说明：
//!     1. Z0、P1,、N1 + 1，常规计算
//!     2. B0<H> + 1，该位B1，无进位，原高位是N1时要规范格式，即H=N1时要特化，此时源码为B0<N1>
//!     3. B1<H> + 1，该位B0，有进位，当H+1 = Z0时要规范格式,即H=N1时要特化，此时源码为B1<N1>，不是简化格式

use crate::number::{FixedPoint, NonNegOne, NonZero, Primitive, Var, B0, B1, N1, P1, Z0};
/// 加一特质 / Increment trait
/// 
/// 为类型系统提供加一操作的计算能力
/// Provides increment operation capability for type system
pub trait Add1 {
    /// 加一后的输出类型 / Output type after increment
    type Output;
    fn add1(self) -> Self::Output;
}

// ========== 基础类型实现 / Basic Type Implementations ==========

/// Z0 (0) 加一实现 / Increment for Z0 (0)
/// 
/// 0 + 1 = 1 (B1<Z0>)
impl Add1 for Z0 {
    type Output = P1;  //P1替换B1<Z0>
    #[inline(always)]
    fn add1(self) -> Self::Output{
        P1::new()
    }
}

/// P1 (1) 加一实现 / Increment for P1 (+1)
/// 
/// 1 + 1 = 2 (B0<P1>)
impl Add1 for P1 {
    type Output = B0<P1>;
    #[inline(always)]
    fn add1(self) -> Self::Output{
        B0::new()
    }
}

/// N1 (-1) 加一实现 / Increment for N1 (-1)
/// 
/// -1 + 1 = 0 (Z0)
impl Add1 for N1 {
    type Output = Z0;
    #[inline(always)]
    fn add1(self) -> Self::Output{
        Z0::new()
    }
}

// ========== 递归类型实现 / Recursive Type Implementations ==========

/// B0<H> 加一实现 / Increment for B0<H>
/// 
/// 直接加一无需进位 / Direct increment without carry
/// ...0 + 1 = ...1 / ...0 + 1 = ...1
impl<H:NonZero + NonNegOne> Add1 for B0<H>{
    type Output = B1<H>;
    #[inline(always)]
    fn add1(self) -> Self::Output{
        B1::new()
    }
}

/// B1<H> 加一实现 / Increment for B1<H>
/// 
/// 处理进位情况 / Handles carry case
/// 0...1 + 1 = 0...(高位进位) / ...1 + 1 = ...0(with carry)
impl<H:NonZero + NonNegOne + Add1<Output: NonZero>> Add1 for B1<H>{//P1替代B1<Z0>后，H不可能为Z0
    type Output = B0<H::Output>;
    #[inline(always)]
    fn add1(self) -> Self::Output{
        B0::new()
    }
}

// 待Float加法完善后考虑其加一实现
/* impl<Mantissa, Exponent> Add1 for Float<Mantissa, Exponent> {
    type Output = <Float<Mantissa, Exponent> as Add<P1>>::out;
    #[inline(always)]
    fn add1(self) -> Self::Output{
        Float::new()
    }
} */
// ========== 特化实现 ==========
/// B0<N1> (-2) 加一特化实现 / Specialized increment for B0<N1> (-2)
impl Add1 for B0<N1> {
    type Output = N1;
    #[inline(always)]
    fn add1(self) -> Self::Output{
        N1::new()
    }
}

// B1<N1> (-1) 加一特化实现，本身不允许B1<N1>出现,其结果也是不规范的格式，目前取消
/* impl Add1 for B1<N1> {
    type Output = Z0;
} */

/// Val<T> 加一实现 / Increment for Val<T>
/// Val<T>
impl<T:Primitive + From<P1>> Add1 for Var<T> {
    type Output = Self;
    #[inline(always)]
    fn add1(self) -> Self::Output{
        Self(self.0 + T::from(P1))
    }
}

// ==============================================
// FixedPoint的Add1实现
// ==============================================

impl<IntPart: Add1, FracPart> Add1 for FixedPoint<IntPart, FracPart>{
    type Output = FixedPoint<IntPart::Output, FracPart>;
    
    fn add1(self) -> Self::Output {
        FixedPoint::new()
    }
}

/* // ==============================================
// Float的Add1实现 等Min 和Max方法完善后实现
// ==============================================

impl<IntPart, FracPart, Exponent: TypedInt> Add1 for Float<FixedPoint<IntPart, FracPart>, Exponent>
where
    Significand: Add1,
    // 这里可以添加更多约束以确保浮点数格式正确
{
    type Output = Float<Significand::Output, Exponent>;
    
    fn add1(self) -> Self::Output {
        Float::new()
    }
} */