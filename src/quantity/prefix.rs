// prefix.rs
//! SI Prefixes (国际单位制词头)
//! 
//! 提供所有标准SI词头用于单位转换，仅处理10的幂次
//! 包括正词头(da, h, k等)和负词头(d, c, m等)
//! 
//! Provides all standard SI prefixes for unit conversion, handling only powers of 10.
//! Includes both positive prefixes (da, h, k etc.) and negative prefixes (d, c, m etc.)

use core::marker::PhantomData;
use core::ops::{Add, Sub, Mul, Div};

use crate::number::{Const, Integer};

/// Prefix struct representing a power of 10
/// 词头结构体，表示10的幂次
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prefix<Exp: Integer>(PhantomData<Exp>);

impl<Exp: Integer> Prefix<Exp> {
    /// Create a new Prefix instance
    /// 创建一个新的词头实例
    pub fn new() -> Self {
        Prefix(PhantomData)
    }
}

/// Prefixed trait defines operations related to SI prefixes
/// SI词头特质定义了与SI词头相关的操作
pub trait Prefixed {}
impl<I: Integer> Prefixed for Prefix<I>{}

// ========== 基本操作实现 ==========
// ========== Basic Operations Implementation ==========

/// 实现词头乘法 (10^a * 10^b = 10^(a+b))
/// Implements prefix multiplication (10^a * 10^b = 10^(a+b))
impl<Ea: Integer + Add<Eb,Output: Integer>, Eb: Integer> Mul<Prefix<Eb>> for Prefix<Ea>{
    type Output = Prefix< <Ea as Add<Eb>>::Output >;
    
    fn mul(self, _: Prefix<Eb>) -> Self::Output {
        Prefix::new()
    }
}

/// 实现词头除法 (10^a / 10^b = 10^(a-b))
/// Implements prefix division (10^a / 10^b = 10^(a-b))
impl<Ea, Eb> Div<Prefix<Eb>> for Prefix<Ea>
where
    Ea: Integer + Sub<Eb,Output:Integer>,
    Eb: Integer,
{
    type Output = Prefix<<Ea as Sub<Eb>>::Output>;
    
    fn div(self, _: Prefix<Eb>) -> Self::Output {
        Prefix::new()
    }
}

// ========== 实用类型别名 ==========
// ========== Useful Type Aliases ==========

/// 无词头 / No prefix (10^0)
pub type NoPrefix = Prefix<Const<0>>;

/// 十 / Deca (10^1)
pub type Deca = Prefix<Const<1>>;

/// 百 / Hecto (10^2)
pub type Hecto = Prefix<Const<2>>;

/// 千 / Kilo (10^3)
pub type Kilo = Prefix<Const<3>>;

/// 兆 / Mega (10^6)
pub type Mega = Prefix<Const<6>>;

/// 吉 / Giga (10^9) 
pub type Giga = Prefix<Const<9>>;

/// 太 / Tera (10^12)
pub type Tera = Prefix<Const<12>>;

/// 拍 / Peta (10^15)
pub type Peta = Prefix<Const<15>>;

/// 艾 / Exa (10^18)
pub type Exa = Prefix<Const<18>>;

/// 泽 / Zetta (10^21)
pub type Zetta = Prefix<Const<21>>;

/// 尧 / Yotta (10^24)
pub type Yotta = Prefix<Const<24>>;

/// 容 / Ronna (10^27)
pub type Ronna = Prefix<Const<27>>;

/// 昆 / Quetta (10^30)
pub type Quetta = Prefix<Const<30>>;

/// 分 / Deci (10^-1)
pub type Deci = Prefix<Const<-1>>;

/// 厘 / Centi (10^-2)
pub type Centi = Prefix<Const<-2>>;

/// 毫 / Milli (10^-3)
pub type Milli = Prefix<Const<-3>>;

/// 微 / Micro (10^-6)
pub type Micro = Prefix<Const<-6>>;

/// 纳 / Nano (10^-9)
pub type Nano = Prefix<Const<-9>>;

/// 皮 / Pico (10^-12)
pub type Pico = Prefix<Const<-12>>;

/// 飞 / Femto (10^-15)
pub type Femto = Prefix<Const<-15>>;

/// 阿 / Atto (10^-18)
pub type Atto = Prefix<Const<-18>>;

/// 仄 / Zepto (10^-21)
pub type Zepto = Prefix<Const<-21>>;

/// 幺 / Yocto (10^-24)
pub type Yocto = Prefix<Const<-24>>;

/// 柔 / Ronto (10^-27)
pub type Ronto = Prefix<Const<-27>>;

/// 亏 / Quecto (10^-30)
pub type Quecto = Prefix<Const<-30>>;


/// 词头乘法结果类型 / Prefix multiplication result type
/// 例如：PrefixMul<Kilo, Milli> = NoPrefix (因为10^3 * 10^-3 = 10^0)
pub type PrefixMul<A, B> = <A as Mul<B>>::Output;

/// 词头除法结果类型 / Prefix division result type
/// 例如：PrefixDiv<Mega, Kilo> = Kilo (因为10^6 / 10^3 = 10^3)
pub type PrefixDiv<A, B> = <A as Div<B>>::Output;




