//! 减一操作特质实现 / Decrement operation trait implementation
//! 说明：
//!     1. Z0、P1,、N1 - 1，常规计算
//!     2. B0<H> - 1，该位B1，有借位，当H-1 = N1时要规范格式,即H=Z0时要特化，此时源码为B0<Z0>，不是简化格式
//!     3. B1<H> - 1，该位B0，无借位，原高位是Z0时要规范格式，即H=Z0时要特化，此时源码为B1<Z0>，已经被P1替代
//!     4. 目前B1<Z0>已经用P1替换，高位H已经不可能为Z0，2条格式已经不存在，3条在H=P1时提前特化

use super::basic::{B0, B1, Z0, P1, N1, NonZero, NonOne};
use crate::variable::{Var,Numeric};
/// 减一特质 / Decrement trait
/// 
/// 为类型系统提供减一操作的计算能力
/// Provides decrement operation capability for type system
pub trait Sub1 {
    /// 减一后的输出类型 / Output type after decrement
    type Output;
    fn sub1(self) -> Self::Output;
}

// ========== 基础类型实现 / Basic Type Implementations ==========

/// Z0 (0) 减一实现 / Decrement for Z0 (0)
/// 
/// 0 - 1 = -1 (N1)
impl Sub1 for Z0 {
    type Output = N1;
    #[inline(always)]
    fn sub1(self) -> Self::Output{
        N1::new()
    }
}

/// P1（原B1<Z0>）(+1) 减一特化实现 / Specialized decrement for P1 (+1)
/// 
impl Sub1 for P1 {
    type Output = Z0;
    #[inline(always)]
    fn sub1(self) -> Self::Output{
        Z0::new()
    }
}

/// N1 (-1) 减一实现 / Decrement for N1 (-1)
/// 
/// -1 - 1 = -2 (B0<N1>)
impl Sub1 for N1 {
    type Output = B0<N1>;
    #[inline(always)]
    fn sub1(self) -> Self::Output{
        B0::new()
    }
}
// ========== 递归类型实现 / Recursive Type Implementations ==========

/// B0<H> 减一实现 / Decrement for B0<H>
/// 
/// 处理借位情况 / Handles borrow case
/// ...0 -1 = ...1(高位借位) / ...0 -1 = ...1(with borrow)
impl<H: NonZero + NonOne + Sub1> Sub1 for B0<H>{//引入P1后，高位不可能是Z0
    type Output = B1<H::Output>;
    #[inline(always)]
    fn sub1(self) -> Self::Output{
        B1::new()
    }
}

/// B1<H> 减一实现 / Decrement for B1<H>
/// 
/// 直接减一无需借位 / Direct decrement without borrow
/// ...1 -1 = ...0 / ...1 -1 = ...0
impl<H: NonZero + NonOne> Sub1 for B1<H>{//引入P1后，高位不可能是Z0
    type Output = B0<H>;
    #[inline(always)]
    fn sub1(self) -> Self::Output{
        B0::new()
    }
}

// ========== 特化实现 ==========
// B1<P1>特化
impl Sub1 for B1<P1>{
    type Output = B0<P1>;
    #[inline(always)]
    fn sub1(self) -> Self::Output{
        B0::new()
    }
}

// B0<P1>特化
impl Sub1 for B0<P1>{
    type Output = P1;
    #[inline(always)]
    fn sub1(self) -> Self::Output{
        P1::new()
    }
}

#[allow(dead_code)]
pub type SubOne<I> = <I as Sub1>::Output;

/// Val<T> - 1
impl<T:Numeric> Sub1 for Var<T> {
    type Output = Self;
    #[inline(always)]
    fn sub1(self) -> Self::Output{
        Self(self.0 - 1.into())
    }
}
