use super::basic::{Z0, P1, N1, B0, B1, NonNegOne, NonZero};

/// 处理 B0<H> 类型的标准化
/// Standardization for B0<H> types
///
/// 当高位 H 为 Z0 时，将 B0<Z0> 转换为 Z0
/// Converts B0<Z0> to Z0 when higher bit H is Z0
pub trait IfB0 {
    type Output;
    fn b0() -> Self::Output;
}

/// 处理 B1<H> 类型的标准化  
/// Standardization for B1<H> types
///
/// 当高位 H 为 N1 时，将 B1<N1> 转换为 N1
/// Converts B1<N1> to N1 when higher bit H is N1
/// 
/// 当高位 H 为 Z0 时，将 B1<Z0> 转换为 P1
/// Converts B1<Z0> to P1 when higher bit H is Z0

pub trait IfB1 {
    type Output;
    fn b1() -> Self::Output;
}

// ==================== IfB0 实现 ====================
impl<I: NonZero> IfB0 for I {
    type Output = B0<I>;
    #[inline(always)]
    fn b0() -> Self::Output {
        B0::new()
    }
}

impl IfB0 for Z0 {// B0<Z0> => Z0
    type Output = Z0;
    #[inline(always)]
    fn b0() -> Self::Output {
        Z0::new()
    }
}

// ==================== IfB1 实现 ====================
impl<I: NonZero + NonNegOne> IfB1 for I {
    type Output = B1<I>;
    #[inline(always)]
    fn b1() -> Self::Output {
        B1::new()
    }
}

impl IfB1 for N1 {// B1<N1> => N1
    type Output = N1;
    #[inline(always)]
    fn b1() -> Self::Output {
        N1::new()
    }
}

impl IfB1 for Z0 {// B1<Z0> => P1
    type Output = P1;
    #[inline(always)]
    fn b1() -> Self::Output {
        P1::new()
    }
}