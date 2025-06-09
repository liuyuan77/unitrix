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
    fn as_bool(&self) -> bool {
        Self::BOOL
    }
}

/// 类型级比特位0（逻辑假）
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, Default)]
pub struct False;

impl False {
    /// 创建一个新的 `False` 实例
    #[inline(always)]
    pub const fn new() -> Self {
        False
    }
}

/// 类型级比特位1（逻辑真）
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, Default)]
pub struct True;

impl True {
    /// 创建一个新的 `True` 实例
    #[inline(always)]
    pub const fn new() -> Self {
        True
    }
}

// 为布尔类型实现密封标记
impl Sealed for False {}
impl Sealed for True {}

impl Boolean for False {
    const BOOL: bool = false;

    #[inline(always)]
    fn new() -> Self {
        Self
    }
}

impl Boolean for True {
    const BOOL: bool = true;

    #[inline(always)]
    fn new() -> Self {
        Self
    }
}

// 实现所有逻辑运算

/// 实现逻辑非运算
impl Not for False {
    type Output = True;
    #[inline(always)]
    fn not(self) -> Self::Output {
        True
    }
}

impl Not for True {
    type Output = False;
    #[inline(always)]
    fn not(self) -> Self::Output {
        False
    }
}

/// 实现逻辑与运算
impl<Rhs: Boolean> BitAnd<Rhs> for False {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, _: Rhs) -> Self::Output {
        Self
    }
}

impl<Rhs: Boolean> BitAnd<Rhs> for True {
    type Output = Rhs;
    #[inline(always)]
    fn bitand(self, rhs: Rhs) -> Self::Output {
        rhs
    }
}

/// 实现逻辑或运算
impl<Rhs: Boolean> BitOr<Rhs> for False {
    type Output = Rhs;
    #[inline(always)]
    fn bitor(self, rhs: Rhs) -> Self::Output {
        rhs
    }
}

impl<Rhs: Boolean> BitOr<Rhs> for True {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, _: Rhs) -> Self::Output {
        self
    }
}

/// 实现逻辑异或运算
impl BitXor<False> for False {
    type Output = False;
    #[inline(always)]
    fn bitxor(self, _: False) -> Self::Output {
        False
    }
}

impl BitXor<False> for True {
    type Output = True;
    #[inline(always)]
    fn bitxor(self, _: False) -> Self::Output {
        True
    }
}

impl BitXor<True> for False {
    type Output = True;
    #[inline(always)]
    fn bitxor(self, _: True) -> Self::Output {
        True
    }
}

impl BitXor<True> for True {
    type Output = False;
    #[inline(always)]
    fn bitxor(self, _: True) -> Self::Output {
        False
    }
}

// 实现转换操作

impl<T: Boolean> From<T> for bool {
    /// 将类型级布尔值转换为运行时布尔值
    fn from(b: T) -> Self {
        b.as_bool()
    }
}