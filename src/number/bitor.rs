/* 数字类型按位或运算实现
 * 作者：$ource
 * 版本：0.0
 * 创建时间：2025-06-26
 */
use core::ops::BitOr;
use crate::number::{
    Z0, P1, N1, B0, B1,
    NonZero, TypedInt, IfB0, IfB1, 
    Primitive, Var, FixedPoint
};

// =============== 基础类型实现 ===============

// ----- 零值(Z0)的位或运算 -----
impl<I: TypedInt> BitOr<I> for Z0 {
    type Output = I;
    #[inline(always)]
    fn bitor(self, rhs: I) -> I { rhs }
}

impl<T: Primitive> BitOr<Var<T>> for Z0 {
    type Output = Var<T>;
    #[inline(always)]
    fn bitor(self, rhs: Var<T>) -> Var<T> { rhs }
}

// ----- 正一(P1)的位或运算 -----
impl BitOr<Z0> for P1 {
    type Output = P1;
    #[inline(always)]
    fn bitor(self, _: Z0) -> P1 { P1 }
}

impl BitOr<P1> for P1 {
    type Output = P1;
    #[inline(always)]
    fn bitor(self, _: P1) -> P1 { P1 }
}

impl BitOr<N1> for P1 {
    type Output = N1;
    #[inline(always)]
    fn bitor(self, _: N1) -> N1 { N1 }
}

impl<H: NonZero + IfB1> BitOr<B0<H>> for P1 {
    type Output = H::Output;
    fn bitor(self, _: B0<H>) -> Self::Output {
        <H as IfB1>::b1()
    }
}

impl<H: NonZero> BitOr<B1<H>> for P1 {
    type Output = B1<H>;
    #[inline(always)]
    fn bitor(self, rhs: B1<H>) -> B1<H> { rhs }
}

impl<T: Primitive> BitOr<Var<T>> for P1
where 
    Var<T>: From<P1> + BitOr<Output = Var<T>>
{
    type Output = Var<T>;
    #[inline(always)]
    fn bitor(self, rhs: Var<T>) -> Var<T> {
        Var::<T>::from(self) | rhs
    }
}

// ----- 负一(N1)的位或运算 -----
impl<I: TypedInt> BitOr<I> for N1 {
    type Output = N1;
    #[inline(always)]
    fn bitor(self, _: I) -> N1 { N1 }
}

impl<T: Primitive> BitOr<Var<T>> for N1 {
    type Output = N1;
    #[inline(always)]
    fn bitor(self, _rhs: Var<T>) -> N1 { N1 }
}

// ----- 二进制0位(B0)的位或运算 -----
impl<H: NonZero> BitOr<Z0> for B0<H> {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, _: Z0) -> Self { self }
}

impl<H: NonZero + IfB1> BitOr<P1> for B0<H> {
    type Output = H::Output;
    fn bitor(self, _: P1) -> Self::Output {
        <H as IfB1>::b1()
    }
}

impl<H: NonZero> BitOr<N1> for B0<H> {
    type Output = N1;
    #[inline(always)]
    fn bitor(self, _: N1) -> N1 { N1 }
}

impl<H1: NonZero + BitOr<H2, Output: IfB0>, H2: NonZero> BitOr<B0<H2>> for B0<H1> {
    type Output = <<H1 as BitOr<H2>>::Output as IfB0>::Output;
    fn bitor(self, _: B0<H2>) -> Self::Output {
        <H1 as BitOr<H2>>::Output::b0()
    }
}

impl<H1: NonZero + BitOr<H2,Output: IfB1>, H2: NonZero> BitOr<B1<H2>> for B0<H1> {
    type Output = <<H1 as BitOr<H2>>::Output as IfB1>::Output;
    fn bitor(self, _: B1<H2>) -> Self::Output {
        <H1 as BitOr<H2>>::Output::b1()
    }
}

impl<H: NonZero, T: Primitive> BitOr<Var<T>> for B0<H>
where 
    Var<T>: From<B0<H>> + BitOr<Output = Var<T>>
{
    type Output = Var<T>;
    #[inline(always)]
    fn bitor(self, rhs: Var<T>) -> Var<T> {
        Var::<T>::from(self) | rhs
    }
}

// ----- 二进制1位(B1)的位或运算 -----
impl<H: NonZero> BitOr<Z0> for B1<H> {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, _: Z0) -> Self { self }
}

impl<H: NonZero> BitOr<P1> for B1<H> {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, _: P1) -> Self { self }
}

impl<H: NonZero> BitOr<N1> for B1<H> {
    type Output = N1;
    #[inline(always)]
    fn bitor(self, _: N1) -> N1 { N1 }
}

impl<H1: NonZero + BitOr<H2>, H2: NonZero> BitOr<B0<H2>> for B1<H1> 
where
    H1::Output: IfB1,
{
    type Output = <<H1 as BitOr<H2>>::Output as IfB1>::Output;
    fn bitor(self, _: B0<H2>) -> Self::Output {
        <H1 as BitOr<H2>>::Output::b1()
    }
}

impl<H1: NonZero + BitOr<H2>, H2: NonZero> BitOr<B1<H2>> for B1<H1> 
where
    H1::Output: IfB1,
{
    type Output = <<H1 as BitOr<H2>>::Output as IfB1>::Output;
    fn bitor(self, _: B1<H2>) -> Self::Output {
        <H1 as BitOr<H2>>::Output::b1()
    }
}

impl<H: NonZero, T: Primitive> BitOr<Var<T>> for B1<H>
where 
    Var<T>: From<B1<H>> + BitOr<Output = Var<T>>
{
    type Output = Var<T>;
    #[inline(always)]
    fn bitor(self, rhs: Var<T>) -> Var<T> {
        Var::<T>::from(self) | rhs
    }
}

// ----- 定点数(FixedPoint)的位或运算 -----
impl<I1, I2, F1, F2> BitOr<FixedPoint<I2, F2>> for FixedPoint<I1, F1>
where
    I1: TypedInt + BitOr<I2>,
    I2: TypedInt,
    F1: TypedInt + BitOr<F2>,
    F2: TypedInt,
{
    type Output = FixedPoint<<I1 as BitOr<I2>>::Output, <F1 as BitOr<F2>>::Output>;
    fn bitor(self, _rhs: FixedPoint<I2, F2>) -> Self::Output {
        FixedPoint::new()
    }
}

// =============== 复合类型实现 ===============

// ----- 变量类型(Var<T>)的位或运算 -----
impl<T: Primitive> BitOr<Var<T>> for Var<T> 
where
    T: BitOr<Output = T>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn bitor(self, rhs: Var<T>) -> Var<T> {
        Var(self.0 | rhs.0)
    }
}

impl<T: Primitive, I: TypedInt> BitOr<I> for Var<T> 
where
    I: BitOr<Var<T>>,
{
    type Output = <I as BitOr<Var<T>>>::Output;
    fn bitor(self, rhs: I) -> Self::Output {
        rhs | self
    }
}

// =============== 测试用例 ===============
#[cfg(test)]
mod tests {
    use super::*;
    use crate::number::{B0, B1, P1, N1, Z0};

    #[test]
    fn test_primitive_or() {
        assert_eq!(Z0 | Z0, Z0);
        assert_eq!(P1 | N1, N1);
        assert_eq!(B0::<P1>::new() | B1::<P1>::new(), B1::<P1>::new());
    }

    #[test]
    fn test_var_or() {
        let v1 = Var(0b1010i8);
        let v2 = Var(0b0101i8);
        assert_eq!(v1 | v2, Var(0b1111i8));
        assert_eq!(v1 | P1, Var(0b1011i8));
    }

    #[test]
    fn test_fixed_point_or() {
        let fp1 = FixedPoint::<P1,B0<P1>>::new();
        let fp2 = FixedPoint::<Z0,B1<P1>>::new();
        let res = fp1 | fp2;
        assert_eq!(res, FixedPoint::<P1, B1<P1>>::new());
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(N1 | Z0, N1);
        assert_eq!(B0::<N1>::new() | N1, N1);
        assert_eq!(
            B1::<P1>::new() | B0::<N1>::new(), 
            <<P1 as BitOr<N1>>::Output as IfB1>::Output::new()
        );
    }
}