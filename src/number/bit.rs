//! 类型级比特位实现
//!
//! 这些是基础的比特位类型，作为本库中其他数值类型的构建基础
//!
//! 已实现的**类型运算符**：
//!
//! - 来自 `core::ops` 的：`BitAnd`(与), `BitOr`(或), `BitXor`(异或) 和 `Not`(非)
//! - 比较操作：`PartialEq`, `Eq`
//! - 转换操作：`From<bool>`, `Into<bool>`
//!

use core::ops::{BitAnd, BitOr, BitXor, Not};

use crate::sealed::Sealed;
use crate::number::{Cmp, Max, Min, Equal, Less, Greater};

/// 编译时比特位的标记特征
///
/// 这个 trait 定义了类型级布尔值的基本操作和行为，
/// 包括构造、转换和常量值访问。
pub trait Boolean: Sealed + Copy + Default + 'static {
    /// 布尔值的编译时常量表示
    const BOOL: bool;

    /// 创建一个该类型的新实例
    fn new() -> Self;

    /// 获取当前实例对应的运行时布尔值
    fn to_bool(&self) -> bool {
        Self::BOOL
    }
}

/// 类型级比特位0（逻辑假）
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, Default)]
pub struct O;

impl O {
    /// 创建一个新的 `O` 实例
    #[inline(always)]
    pub const fn new() -> Self {
        O
    }
}

/// 类型级比特位1（逻辑真）
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, Default)]
pub struct I;

impl I {
    /// 创建一个新的 `I` 实例
    #[inline(always)]
    pub const fn new() -> Self {
        I
    }
}

// 为布尔类型实现密封标记
impl Sealed for O {}
impl Sealed for I {}

impl Boolean for O {
    const BOOL: bool = false;

    #[inline(always)]
    fn new() -> Self {
        Self
    }
}

impl Boolean for I {
    const BOOL: bool = true;

    #[inline(always)]
    fn new() -> Self {
        Self
    }
}

// 为bit时的功能
impl Cmp<O> for O {
    type Output = Equal;

    #[inline]
    fn compare(self, _: O) -> Self::Output {
        Equal::new()
    }
}

impl Cmp<I> for O {
    type Output = Less;

    #[inline]
    fn compare(self, _: I) -> Self::Output {
        Less::new()
    }
}

impl Cmp<O> for I {
    type Output = Greater;

    #[inline]
    fn compare(self, _: O) -> Self::Output {
        Greater::new()
    }
}

impl Cmp<I> for I {
    type Output = Equal;

    #[inline]
    fn compare(self, _: I) -> Self::Output {
        Equal::new()
    }
}

// Min
impl Min<O> for O {
    type Output = O;
    #[inline]
    fn min(self, _: O) -> O {
        self
    }
}
impl Min<I> for O {
    type Output = O;
    #[inline]
    fn min(self, _: I) -> O {
        self
    }
}
impl Min<O> for I {
    type Output = O;
    #[inline]
    fn min(self, rhs: O) -> O {
        rhs
    }
}
impl Min<I> for I {
    type Output = I;
    #[inline]
    fn min(self, _: I) -> I {
        self
    }
}

// Max
impl Max<O> for O {
    type Output = O;
    #[inline]
    fn max(self, _: O) -> O {
        self
    }
}
impl Max<I> for O {
    type Output = I;
    #[inline]
    fn max(self, rhs: I) -> I {
        rhs
    }
}
impl Max<O> for I {
    type Output = I;
    #[inline]
    fn max(self, _: O) -> I {
        self
    }
}
impl Max<I> for I {
    type Output = I;
    #[inline]
    fn max(self, _: I) -> I {
        self
    }
}

// 逻辑值
pub type False = O;
pub type True = I;

// 实现所有逻辑运算

/// 实现逻辑非运算
impl Not for O {
    type Output = I;
    #[inline(always)]
    fn not(self) -> Self::Output {
        I
    }
}

impl Not for I {
    type Output = O;
    #[inline(always)]
    fn not(self) -> Self::Output {
        O
    }
}

/// 实现逻辑与运算
impl<Rhs: Boolean> BitAnd<Rhs> for O {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, _: Rhs) -> Self::Output {
        Self
    }
}

impl<Rhs: Boolean> BitAnd<Rhs> for I {
    type Output = Rhs;
    #[inline(always)]
    fn bitand(self, rhs: Rhs) -> Self::Output {
        rhs
    }
}

/// 实现逻辑或运算
impl<Rhs: Boolean> BitOr<Rhs> for O {
    type Output = Rhs;
    #[inline(always)]
    fn bitor(self, rhs: Rhs) -> Self::Output {
        rhs
    }
}

impl<Rhs: Boolean> BitOr<Rhs> for I {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, _: Rhs) -> Self::Output {
        self
    }
}

/// 实现逻辑异或运算
impl BitXor<O> for O {
    type Output = O;
    #[inline(always)]
    fn bitxor(self, _: O) -> Self::Output {
        O
    }
}

impl BitXor<O> for I {
    type Output = I;
    #[inline(always)]
    fn bitxor(self, _: O) -> Self::Output {
        I
    }
}

impl BitXor<I> for O {
    type Output = I;
    #[inline(always)]
    fn bitxor(self, _: I) -> Self::Output {
        I
    }
}

impl BitXor<I> for I {
    type Output = O;
    #[inline(always)]
    fn bitxor(self, _: I) -> Self::Output {
        O
    }
}

// 实现转换操作
impl From<True> for bool {
    fn from(_: True) -> bool { true }
}

impl From<False> for bool {
    fn from(_: False) -> bool { false }
}



