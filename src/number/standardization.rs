//! 二进制数字标准化模块 / Binary Number Normalization Module
//! 
//! 提供将二进制数字(B0/B1)标准化为更简洁表示形式的功能
//! Provides functionality to normalize binary numbers (B0/B1) into more concise representations
//! 
//! 例如 B0<Z0> => Z0, B1<Z0> => P1, B1<N1> => N1
//! e.g. B0<Z0> => Z0, B1<Z0> => P1, B1<N1> => N1

use crate::number::{Z0, P1, N1, B0, B1, NonNegOne, NonZero};

// ==================== Int ====================

/// 处理 B0<H> 类型的标准化 / Standardization for B0<H> types
///
/// 这个 trait 定义了将 B0<H> 类型数字标准化的行为。
/// This trait defines the behavior for normalizing B0<H> type numbers.
/// 
/// 当高位 H 为 Z0 时，将 B0<Z0> 转换为 Z0；
/// Converts B0<Z0> to Z0 when higher bit H is Z0;
/// 对于其他非零高位，保持 B0<H> 结构不变。
/// maintains B0<H> structure for other non-zero higher bits.
pub trait IfB0 {
    type Output;
    fn b0() -> Self::Output;
}

/// 处理 B1<H> 类型的标准化 / Standardization for B1<H> types
///
/// 这个 trait 定义了将 B1<H> 类型数字标准化的行为。
/// This trait defines the behavior for normalizing B1<H> type numbers.
/// 
/// 当高位 H 为 N1 时，将 B1<N1> 转换为 N1；
/// Converts B1<N1> to N1 when higher bit H is N1;
/// 当高位 H 为 Z0 时，将 B1<Z0> 转换为 P1；
/// Converts B1<Z0> to P1 when higher bit H is Z0;
/// 对于其他非零非负一高位，保持 B1<H> 结构不变。
/// maintains B1<H> structure for other non-zero non-negative-one higher bits.
pub trait IfB1 {
    type Output;
    fn b1() -> Self::Output;
}

// ==================== IfB0 实现 / IfB0 Implementations ====================

/// 为所有非零类型实现 IfB0 / IfB0 implementation for all non-zero types
///
/// 保持 B0<H> 结构不变，其中 H 是非零类型
/// Maintains B0<H> structure where H is non-zero type
impl<I: NonZero + Default> IfB0 for I {
    type Output = B0<I>;
    #[inline(always)]
    fn b0() -> Self::Output {
        B0::new()
    }
}

/// 为零类型 Z0 实现 IfB0 / IfB0 implementation for zero type Z0
///
/// 将 B0<Z0> 转换为 Z0
/// Converts B0<Z0> to Z0
impl IfB0 for Z0 {// B0<Z0> => Z0
    type Output = Z0;
    #[inline(always)]
    fn b0() -> Self::Output {
        Z0::new()
    }
}

// ==================== IfB1 实现 / IfB1 Implementations ====================

/// 为非零非负一类型实现 IfB1 / IfB1 implementation for non-zero non-negative-one types
///
/// 保持 B1<H> 结构不变，其中 H 是非零非负一类型
/// Maintains B1<H> structure where H is non-zero non-negative-one type
impl<I: NonZero + NonNegOne + Default> IfB1 for I {
    type Output = B1<I>;
    #[inline(always)]
    fn b1() -> Self::Output {
        B1::new()
    }
}

/// 为负一类型 N1 实现 IfB1 / IfB1 implementation for negative-one type N1
///
/// 将 B1<N1> 转换为 N1
/// Converts B1<N1> to N1
impl IfB1 for N1 {// B1<N1> => N1
    type Output = N1;
    #[inline(always)]
    fn b1() -> Self::Output {
        N1::new()
    }
}

/// 为零类型 Z0 实现 IfB1 / IfB1 implementation for zero type Z0
///
/// 将 B1<Z0> 转换为 P1
/// Converts B1<Z0> to P1
impl IfB1 for Z0 {// B1<Z0> => P1
    type Output = P1;
    #[inline(always)]
    fn b1() -> Self::Output {
        P1::new()
    }
}

// ==================== Float ====================
