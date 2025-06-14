use core::ops::BitAnd;
use crate::number::{B0, B1, Z0, P1, N1, NonZero, Integer, IfB0, IfB1};

// ==================== 按位与（&运算符） ====================

// ==================== Z0 & All ====================
// Z0 & 整数
impl<I: Integer> BitAnd<I> for Z0 {
    type Output = Z0;
    #[inline(always)]
    fn bitand(self, _rhs: I) -> Self::Output {
        Z0
    }
}

// ==================== P1 & All ====================
// P1 & Z0
impl BitAnd<Z0> for P1 {
    type Output = Z0;
    fn bitand(self, _: Z0) -> Self::Output {
        Z0
    }
}

// P1 & P1
impl BitAnd<P1> for P1 {
    type Output = P1;
    fn bitand(self, _: P1) -> Self::Output {
        P1
    }
}

// P1 & N1
impl BitAnd<N1> for P1 {
    type Output = P1;
    fn bitand(self, _: N1) -> Self::Output {
        P1
    }
}

// P1 & B0
impl<H: NonZero> BitAnd<B0<H>> for P1 { //说明高位可能有Z0出现，需要规格化处理
    type Output = Z0;
    fn bitand(self, _rhs: B0<H>) -> Self::Output {
        Z0
    }
}

// P1 & B1
impl<H: NonZero> BitAnd<B1<H>> for P1 {
    type Output = P1;
    fn bitand(self, _rhs: B1<H>) -> Self::Output {
        P1
    }
}

// ==================== N1 & All ====================
impl<I: Integer> BitAnd<I> for N1 {
    type Output = I;
    #[inline(always)]
    fn bitand(self, rhs: I) -> Self::Output {
        rhs
    }
}

// ==================== B0 & All ====================
// B0 & Z0
impl<H: NonZero> BitAnd<Z0> for B0<H> {
    type Output = Z0;
    #[inline(always)]
    fn bitand(self, _rhs: Z0) -> Self::Output {
        Z0
    }
}

// B0 & P1
impl<H: NonZero> BitAnd<P1> for B0<H>{ //说明高位可能有Z0出现，需要规格化处理
    type Output = Z0;
    #[inline(always)]
    fn bitand(self, _rhs: P1) -> Self::Output {
        Z0
    }
}

// B0 & N1
impl<H: NonZero> BitAnd<N1> for B0<H>{
    type Output = Self;
    #[inline(always)]
    fn bitand(self, _rhs: N1) -> Self::Output {
        self
    }
}

// B0 & B0
impl<H1: NonZero + BitAnd<H2, Output: IfB0>, H2: NonZero> BitAnd<B0<H2>> for B0<H1>{
    type Output = <H1::Output as IfB0>::Output;
    #[inline(always)]
    fn bitand(self, _rhs: B0<H2>) -> Self::Output {
        <<H1 as BitAnd<H2>>::Output as IfB0>::b0()
    }
}

// B0 & B1
impl<H1: NonZero + BitAnd<H2, Output: IfB0>, H2: NonZero> BitAnd<B1<H2>> for B0<H1>{
    type Output = <H1::Output as IfB0>::Output;
    #[inline(always)]
    fn bitand(self, _rhs: B1<H2>) -> Self::Output {
        <<H1 as BitAnd<H2>>::Output as IfB0>::b0()
    }
}

// ==================== B1 & All ====================
// B1 & Z0
impl<H: NonZero> BitAnd<Z0> for B1<H> {
    type Output = Z0;
    #[inline(always)]
    fn bitand(self, _rhs: Z0) -> Self::Output {
        Z0
    }
}

// B1 & P1
impl<H: NonZero> BitAnd<P1> for B1<H>{
    type Output = P1;
    #[inline(always)]
    fn bitand(self, _rhs: P1) -> Self::Output {
        P1
    }
}

// B1 & N1
impl<H: NonZero> BitAnd<N1> for B1<H>{
    type Output = Self;
    #[inline(always)]
    fn bitand(self, _rhs: N1) -> Self::Output {
        self
    }
}

// B1 & B0
impl<H1: NonZero + BitAnd<H2,Output: IfB0>, H2: NonZero> BitAnd<B0<H2>> for B1<H1>{
    type Output = <H1::Output as IfB0>::Output;
    #[inline(always)]
    fn bitand(self, _rhs: B0<H2>) -> Self::Output {
        <<H1 as BitAnd<H2>>::Output as IfB0>::b0()
    }
}

// B1 & B1
impl<H1: NonZero + BitAnd<H2, Output: IfB1>, H2: NonZero> BitAnd<B1<H2>> for B1<H1>{
    type Output = <H1::Output as IfB1>::Output;
    #[inline(always)]
    fn bitand(self, _rhs: B1<H2>) -> Self::Output {
        <<H1 as BitAnd<H2>>::Output as IfB1>::b1()
    }
}