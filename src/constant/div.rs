/* use core::ops::{Div, Rem, Sub};
use super::basic::{Z0, P1, N1, B0, B1, Integer, NonZero, IsGreaterEqual};
use super::mul::Mul;
use super::sub::Sub;

/// 类型级别除法
pub trait Div<Rhs> {
    type Output;
    type Remainder; // 新增余数类型
}

/// 比较运算（判断是否大于等于）
pub trait IsGreaterEqual<Rhs> {
    type Output;
}

impl<I: Integer> Div<P1> for I {
    type Output = I;
    type Remainder = Z0;
}

impl<I: NonZero> Div<I> for Z0 {
    type Output = Z0;
    type Remainder = Z0;
}

// B1<H> / D 的实现
impl<H, D> Div<D> for B1<H>
where
    H: NonZero,
    D: NonZero + IsGreaterEqual<B1<H>>,
    B0<H>: Div<D>,
    <B0<H> as Div<D>>::Output: Shl1, // 需要实现位移trait
    <B0<H> as Div<D>>::Remainder: Shl1,
    <<B0<H> as Div<D>>::Remainder as Shl1>::Output: Add<P1>,
    <<<B0<H> as Div<D>>::Remainder as Shl1>::Output as Add<P1>>::Output: IsGreaterEqual<D>,
{
    type Output = <<B0<H> as Div<D>>::Output as Shl1>::Output;
    type Remainder = /* 根据比较结果确定余数 */;
} */

// 注意：正数/零和负数/零未实现，因为数学上除以零未定义


pub type Prod<A, B> = <A as Div<B>>::Output;