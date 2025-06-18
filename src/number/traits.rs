use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};
use crate::sealed::Sealed;
use crate::number::{Special, Z0, P1, N1, B0, B1, Float};
// =============================================
// ============ 标记特质定义 ====================
// ========== Marker Traits Definition ==========
// =============================================

/// 所有类型化数字的基标记特质
/// Base marker trait for all typed numbers in the system
pub trait TypedNum: Sealed {}

// 非特殊值（既不是NaN也不是无穷大）的标记特质
/// Marker trait for non-special values (neither NaN nor infinity)
pub trait NonSpecial: TypedNum {}

/// 类型化整数的标记特质
/// Marker trait for typed integers
///
/// # 要求
/// - 必须可复制且具有'static生命周期
/// - 提供到i32的转换(临时方案，待Var实现完成后取消)
///
/// # Requirements
/// - Must be copyable and have 'static lifetime
/// - Provides conversion to i32 (temporary until Var is implemented)
pub trait TypedInt: TypedNum + Copy + 'static {
    /// 将类型级整数转换为运行时i32值
    /// Converts the type-level integer to a runtime i32 value
    ///
    /// # 注意
    /// 这是临时方案，Var实现完成后将取消
    ///
    /// # Note
    /// This is a temporary solution until Var implementation is complete
    fn to_i32() -> i32;
}

/// 非零类型化整数的标记特质
/// Marker trait for non-zero typed integers
pub trait NonZero: TypedInt {}

/// 不等于1的类型化整数的标记特质
/// Marker trait for typed integers not equal to 1
pub trait NonOne: TypedInt {}

/// 不等于-1的类型化整数的标记特质
/// Marker trait for typed integers not equal to -1
pub trait NonNegOne: TypedInt {}

/// 无符号类型化整数的标记特质
/// Marker trait for unsigned typed integers
pub trait Unsigned: TypedInt {}

/// 浮点数表示的标记特质
/// Marker trait for floating-point number representations
///
/// # 实现者
/// - 标准浮点类型(f32/f64)
/// - Float<M,E>自定义浮点表示
/// - 特殊浮点值:
///   - NotANumber
///   - PositiveInfinity
///   - NegativeInfinity
///
/// # Implementors
/// - Standard float types (f32/f64)
/// - Float<M,E> custom float representation
/// - Special float values:
///   - NotANumber
///   - PositiveInfinity
///   - NegativeInfinity
pub trait TypedFloat: TypedNum {}

/// 正数的标记特质（适用于整数和浮点数）
/// Marker trait for positive numbers (both integers and floats)
pub trait Positive: TypedNum {}

/// 负数的标记特质（适用于整数和浮点数）
/// Marker trait for negative numbers (both integers and floats)
pub trait Negative: TypedNum {}

/// 基本整数类型的标记特质
/// Marker trait for primitive integer types
///
/// # 要求
/// 必须实现基本算术运算和常见特质
///
/// # Requirements
/// Must implement basic arithmetic operations and common traits
pub trait PrimitiveInt:
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    AddAssign +
    SubAssign +
    Copy +
    Clone +
    Default +
    Sized +
    'static
{}

pub trait PrimitiveFloat:
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    AddAssign +
    SubAssign +
    Copy +
    Clone +
    Default +
    Sized +
    'static
{}

// =============================================
// ========== TypedNum 实现 ====================
// ======== TypedNum Implementations ===========
// =============================================

impl TypedNum for Special {}
impl<I: TypedInt> TypedNum for I {}
impl<Mantissa: NonZero, Exponent: TypedInt> TypedNum for Float<Mantissa, Exponent> {}

// =============================================
// ========== NonSpecial 实现 ==================
// ======= NonSpecial Implementations ==========
// =============================================
impl<I: TypedInt> NonSpecial for I {}
impl<Mantissa: NonZero, Exponent: TypedInt> NonSpecial for Float<Mantissa, Exponent> {}

// =============================================
// ========== Positive 实现（整数 & 浮点数） =============
// ======= Positive Implementations (Int & Float) ======
// =============================================

impl Positive for P1 {}  // +1是正数 | +1 is positive
impl<H: NonZero + Positive> Positive for B0<H> {}  // 当H为正时，H*2是正数 | H*2 is positive when H is positive
impl<H: NonZero + Positive> Positive for B1<H> {}  // 当H为正时，H*2+1是正数 | H*2+1 is positive when H is positive
// 为浮点正数实现Positive
// Implement Positive for floating-point positive numbers
impl<Mantissa: NonZero + Positive, Exponent: TypedInt> Positive for Float<Mantissa, Exponent> {}
// 为正无穷大实现Positive
// Implement Positive for positive infinity
//impl Positive for Special::Infinity {}

// =============================================
// ========== Negative 实现（整数 & 浮点数） =============
// ======= Negative Implementations (Int & Float) ======
// =============================================

impl Negative for N1 {}  // -1是负数 | -1 is negative
impl<H: NonZero + Negative> Negative for B0<H> {}  // 当H为负时，H*2是负数 | H*2 is negative when H is negative
impl<H: NonZero + Negative> Negative for B1<H> {}  // 当H为负时，H*2+1是负数 | H*2+1 is negative when H is negative

// Implement Negative for floating-point negative numbers
impl<Mantissa: NonZero + Negative, Exponent: TypedInt> Negative for Float<Mantissa, Exponent> {}

// 为特殊负无穷大实现Negative
// Implement Negative for negative infinity
//impl Negative for Special::NegInfinity {}

// =============================================
// ========== NonZero 实现 =====================
// ======== NonZero Implementations ============
// =============================================

impl NonZero for P1 {}   // +1是非零 | +1 is non-zero
impl NonZero for N1 {}   // -1是非零 | -1 is non-zero
impl<H: NonZero> NonZero for B0<H> {}  // B0保持非零属性 | B0 preserves non-zero property
impl<H: NonZero> NonZero for B1<H> {}  // B1保持非零属性 | B1 preserves non-zero property

// =============================================
// ========== TypedInt 实现 ====================
// ======== TypedInt Implementations ==========
// =============================================

// Z0表示0 | Z0 represents 0
impl TypedInt for Z0 {
    #[inline(always)]
    fn to_i32() -> i32 {
        0
    }
}

// P1表示+1 | P1 represents +1
impl TypedInt for P1 {
    #[inline(always)]
    fn to_i32() -> i32 {
        1
    }
}

// N1表示-1 | N1 represents -1
impl TypedInt for N1 {
    #[inline(always)]
    fn to_i32() -> i32 {
        -1
    }
}

// B0<H>表示H * 2 | B0<H> represents H * 2
impl<H: NonZero> TypedInt for B0<H> {
    #[inline(always)]
    fn to_i32() -> i32 {
        H::to_i32() * 2
    }
}

// B1<H>表示H * 2 + 1 | B1<H> represents H * 2 + 1
impl<H: NonZero> TypedInt for B1<H> {
    #[inline(always)]
    fn to_i32() -> i32 {
        H::to_i32() * 2 + 1
    }
}

// =============================================
// ========== NonOne 实现 ======================
// ======== NonOne Implementations =============
// =============================================

impl NonOne for Z0 {}      // 0 ≠ 1 | 0 ≠ 1
impl NonOne for N1 {}      // -1 ≠ 1 | -1 ≠ 1
impl<H: NonZero> NonOne for B0<H> {}  // 当H ≠ 0时，H*2 ≠ 1 | H*2 ≠ 1 when H ≠ 0
impl<H: NonZero> NonOne for B1<H> {}  // 当H ≠ 0时，H*2+1 ≠ 1 | H*2+1 ≠ 1 when H ≠ 0

// =============================================
// ========== NonNegOne 实现 ===================
// ======= NonNegOne Implementations ===========
// =============================================

impl NonNegOne for Z0 {}    // 0 ≠ -1 | 0 ≠ -1
impl NonNegOne for P1 {}    // +1 ≠ -1 | +1 ≠ -1
impl<H: NonZero> NonNegOne for B0<H> {}  // 当H ≠ 0时，H*2 ≠ -1 | H*2 ≠ -1 when H ≠ 0
impl<H: NonZero> NonNegOne for B1<H> {}  // 当H ≠ 0时，H*2+1 ≠ -1 | H*2+1 ≠ -1 when H ≠ 0

// =============================================
// ========== Unsigned 实现 ====================
// ======= Unsigned Implementations ============
// =============================================

impl Unsigned for Z0 {}     // 0是无符号的 | 0 is unsigned
impl Unsigned for P1 {}     // +1是无符号的 | +1 is unsigned
impl<H: NonZero + NonNegOne> Unsigned for B0<H> {}  // 当H为正数时，H*2是无符号的 | H*2 is unsigned if H is positive
impl<H: NonZero + NonNegOne> Unsigned for B1<H> {}  // 当H为正数时，H*2+1是无符号的 | H*2+1 is unsigned if H is positive

// =============================================
// ========== TypedFloat 实现 ===================
// ======= TypedFloat Implementations ===========
// =============================================

// 为特殊浮点值实现TypedFloat
// Implement TypedFloat for special float values
impl TypedFloat for Special {}

// 为Float结构和基本整数实现TypedFloat
// Implement TypedFloat for Float struct and basic integers
impl<Mantissa: NonZero, Exponent: TypedInt> TypedFloat for Float<Mantissa, Exponent> {}
impl TypedFloat for Z0 {}
impl TypedFloat for P1 {}
impl TypedFloat for N1 {}

// =============================================
// ========== PrimitiveInt 实现 ================
// ====== PrimitiveInt Implementations =========
// =============================================

// 为所有标准整数类型实现PrimitiveInt
// Implement PrimitiveInt for all standard integer types
impl PrimitiveInt for i8 {}
impl PrimitiveInt for i16 {}
impl PrimitiveInt for i32 {}
impl PrimitiveInt for i64 {}
impl PrimitiveInt for i128 {}
impl PrimitiveInt for isize {}

impl PrimitiveFloat for f32 {}
impl PrimitiveFloat for f64 {}