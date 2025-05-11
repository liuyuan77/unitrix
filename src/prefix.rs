// prefix.rs
//! SI Prefixes (国际单位制词头)
//! 
//! 提供所有标准SI词头用于单位转换，仅处理10的幂次
//! 
//! Provides all standard SI prefixes for unit conversion, handling only powers of 10.

use typenum::{
    Z0, P1, P2, P3, P6, P9, P12, P15, P18, P21, P24, P27, P30,
    N1, N2, N3, N6, N9, N12, N15, N18, N21, N24, N27, N30,
    Integer, IsEqual, Sum, Diff
};
use core::marker::PhantomData;
use core::ops::{Add, Sub, Mul, Div};

/// Prefixed trait defines operations related to SI prefixes
/// SI词头特质定义了与SI词头相关的操作
pub trait Prefixed {
    /// The symbol of the prefix (e.g. "k" for kilo)
    /// 词头符号(例如"k"表示千)
    const SYMBOL: &'static str;
    
    /// The exponent of the prefix (e.g. 3 for kilo which means 10^3)
    /// 词头的幂次(例如3表示千，即10^3)
    const EXPONENT: i32;
    
    /// Whether the prefix is positive (exponent > 0)
    /// 是否是正词头(幂次>0)
    const IS_POSITIVE: bool = { Self::EXPONENT > 0 };
    
    /// Whether the prefix is negative (exponent < 0)
    /// 是否是负词头(幂次<0)
    const IS_NEGATIVE: bool = { Self::EXPONENT < 0 };
}

/// Prefix struct representing a power of 10
/// 词头结构体，表示10的幂次
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prefix<Exp: Integer>(PhantomData<Exp>);

// 实现构造函数
impl<Exp: Integer> Prefix<Exp> {
    /// Create a new Prefix instance
    /// 创建一个新的词头实例
    pub fn new() -> Self {
        Prefix(PhantomData)
    }
}

// 为各类型实现PrefixLike
// Implement Prefixed for various types

// 无词头 / No prefix
impl Prefixed for Prefix<Z0> {
    const SYMBOL: &'static str = "";
    const EXPONENT: i32 = 0;
}

// 正词头 / Positive prefixes
impl Prefixed for Prefix<P1> {
    const SYMBOL: &'static str = "da";
    const EXPONENT: i32 = 1;
}

impl Prefixed for Prefix<P2> {
    const SYMBOL: &'static str = "h";
    const EXPONENT: i32 = 2;
}

impl Prefixed for Prefix<P3> {
    const SYMBOL: &'static str = "k";
    const EXPONENT: i32 = 3;
}

impl Prefixed for Prefix<P6> {
    const SYMBOL: &'static str = "M";
    const EXPONENT: i32 = 6;
}

impl Prefixed for Prefix<P9> {
    const SYMBOL: &'static str = "G";
    const EXPONENT: i32 = 9;
}

impl Prefixed for Prefix<P12> {
    const SYMBOL: &'static str = "T";
    const EXPONENT: i32 = 12;
}

impl Prefixed for Prefix<P15> {
    const SYMBOL: &'static str = "P";
    const EXPONENT: i32 = 15;
}

impl Prefixed for Prefix<P18> {
    const SYMBOL: &'static str = "E";
    const EXPONENT: i32 = 18;
}

impl Prefixed for Prefix<P21> {
    const SYMBOL: &'static str = "Z";
    const EXPONENT: i32 = 21;
}

impl Prefixed for Prefix<P24> {
    const SYMBOL: &'static str = "Y";
    const EXPONENT: i32 = 24;
}

impl Prefixed for Prefix<P27> {
    const SYMBOL: &'static str = "R";
    const EXPONENT: i32 = 27;
}

impl Prefixed for Prefix<P30> {
    const SYMBOL: &'static str = "Q";
    const EXPONENT: i32 = 30;
}

// 负词头 / Negative prefixes
impl Prefixed for Prefix<N1> {
    const SYMBOL: &'static str = "d";
    const EXPONENT: i32 = -1;
}

impl Prefixed for Prefix<N2> {
    const SYMBOL: &'static str = "c";
    const EXPONENT: i32 = -2;
}

impl Prefixed for Prefix<N3> {
    const SYMBOL: &'static str = "m";
    const EXPONENT: i32 = -3;
}

impl Prefixed for Prefix<N6> {
    const SYMBOL: &'static str = "μ";
    const EXPONENT: i32 = -6;
}

impl Prefixed for Prefix<N9> {
    const SYMBOL: &'static str = "n";
    const EXPONENT: i32 = -9;
}

impl Prefixed for Prefix<N12> {
    const SYMBOL: &'static str = "p";
    const EXPONENT: i32 = -12;
}

impl Prefixed for Prefix<N15> {
    const SYMBOL: &'static str = "f";
    const EXPONENT: i32 = -15;
}

impl Prefixed for Prefix<N18> {
    const SYMBOL: &'static str = "a";
    const EXPONENT: i32 = -18;
}

impl Prefixed for Prefix<N21> {
    const SYMBOL: &'static str = "z";
    const EXPONENT: i32 = -21;
}

impl Prefixed for Prefix<N24> {
    const SYMBOL: &'static str = "y";
    const EXPONENT: i32 = -24;
}

impl Prefixed for Prefix<N27> {
    const SYMBOL: &'static str = "r";
    const EXPONENT: i32 = -27;
}

impl Prefixed for Prefix<N30> {
    const SYMBOL: &'static str = "q";
    const EXPONENT: i32 = -30;
}

// ========== 基本操作实现 ==========
// ========== Basic Operations Implementation ==========

/// 实现词头乘法 (10^a * 10^b = 10^(a+b))
/// Implements prefix multiplication (10^a * 10^b = 10^(a+b))
impl<Ea, Eb> Mul<Prefix<Eb>> for Prefix<Ea>
where
    Ea: Integer + Add<Eb>,
    Eb: Integer,
    Sum<Ea, Eb>: Integer,
{
    type Output = Prefix<Sum<Ea, Eb>>;
    
    fn mul(self, _: Prefix<Eb>) -> Self::Output {
        Prefix::new()
    }
}

/// 实现词头除法 (10^a / 10^b = 10^(a-b))
/// Implements prefix division (10^a / 10^b = 10^(a-b))
impl<Ea, Eb> Div<Prefix<Eb>> for Prefix<Ea>
where
    Ea: Integer + Sub<Eb>,
    Eb: Integer,
    Diff<Ea, Eb>: Integer,
{
    type Output = Prefix<Diff<Ea, Eb>>;
    
    fn div(self, _: Prefix<Eb>) -> Self::Output {
        Prefix::new()
    }
}

// ========== 实用类型别名 ==========
// ========== Useful Type Aliases ==========

pub type NoPrefix = Prefix<Z0>;
pub type Deca = Prefix<P1>;
pub type Hecto = Prefix<P2>;
pub type Kilo = Prefix<P3>;
pub type Mega = Prefix<P6>;
pub type Giga = Prefix<P9>;
pub type Tera = Prefix<P12>;
pub type Peta = Prefix<P15>;
pub type Exa = Prefix<P18>;
pub type Zetta = Prefix<P21>;
pub type Yotta = Prefix<P24>;
pub type Ronna = Prefix<P27>;
pub type Quetta = Prefix<P30>;

pub type Deci = Prefix<N1>;
pub type Centi = Prefix<N2>;
pub type Milli = Prefix<N3>;
pub type Micro = Prefix<N6>;
pub type Nano = Prefix<N9>;
pub type Pico = Prefix<N12>;
pub type Femto = Prefix<N15>;
pub type Atto = Prefix<N18>;
pub type Zepto = Prefix<N21>;
pub type Yocto = Prefix<N24>;
pub type Ronto = Prefix<N27>;
pub type Quecto = Prefix<N30>;

/// 词头乘法结果类型 / Prefix multiplication result type
pub type PrefixMul<A, B> = <A as Mul<B>>::Output;

/// 词头除法结果类型 / Prefix division result type
pub type PrefixDiv<A, B> = <A as Div<B>>::Output;

/// 词头相等判断类型 / Prefix equality comparison type
pub type PrefixEq<A, B> = <A as IsEqual<B>>::Output;

// ========== 调试和测试代码 ==========
// ========== Debug and Test Code ==========

#[cfg(test)]
mod tests {
    use super::*;
    
    /// 测试词头属性和基本操作
    /// Test prefix properties and basic operations
    #[test]
    fn test_prefix_properties() {
        assert_eq!(Kilo::SYMBOL, "k");
        assert_eq!(Kilo::EXPONENT, 3);
        assert!(Kilo::IS_POSITIVE);
        assert!(!Kilo::IS_NEGATIVE);
        
        assert_eq!(Milli::SYMBOL, "m");
        assert_eq!(Milli::EXPONENT, -3);
        assert!(!Milli::IS_POSITIVE);
        assert!(Milli::IS_NEGATIVE);
    }
    
    #[test]
    fn test_prefix_multiplication() {
        let kilo = Prefix::<P3>::new();
        let milli = Prefix::<N3>::new();
        
        // k * m = 10^3 * 10^-3 = 10^0
        let result = kilo * milli;
        assert_eq!(result, Prefix::<Z0>::new());
        
        // M * k = 10^6 * 10^3 = 10^9 (Giga)
        let mega = Prefix::<P6>::new();
        let kilo = Prefix::<P3>::new();
        let result = mega * kilo;
        assert_eq!(result, Prefix::<P9>::new());
    }
    
    #[test]
    fn test_prefix_division() {
        let kilo = Prefix::<P3>::new();
        let milli = Prefix::<N3>::new();
        
        // k / m = 10^3 / 10^-3 = 10^6 (Mega)
        let result = kilo / milli;
        assert_eq!(result, Prefix::<P6>::new());
        
        // m / k = 10^-3 / 10^3 = 10^-6 (Micro)
        let result = milli / kilo;
        assert_eq!(result, Prefix::<N6>::new());
    }
    
    #[test]
    fn test_type_aliases() {
        assert_eq!(<PrefixMul<Kilo, Milli> as Prefixed>::EXPONENT, 0);
        assert_eq!(<PrefixDiv<Mega, Kilo> as Prefixed>::EXPONENT, 3);
    }
}

/// 调试输出实现
/// Debug output implementation
impl<Exp: Integer> core::fmt::Display for Prefix<Exp>
where
    Prefix<Exp>: Prefixed,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Prefix({}, 10^{})", Self::SYMBOL, Self::EXPONENT)
    }
}