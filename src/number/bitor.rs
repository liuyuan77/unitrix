use core::ops::BitOr;
use crate::number::{B0, B1, Z0, P1, N1, NonZero, Integer, IfB0, IfB1};

// ==================== 按位或（|运算符） ====================

// ==================== Z0 | All ====================
// Z0 | 整数
impl<I: Integer> BitOr<I> for Z0 {
    type Output = I;
    #[inline(always)]
    fn bitor(self, rhs: I) -> Self::Output {
        rhs
    }
}

// ==================== P1 | All ====================
// P1 | Z0
impl BitOr<Z0> for P1 {
    type Output = P1;
    fn bitor(self, _: Z0) -> Self::Output {
        P1
    }
}

// P1 | P1
impl BitOr<P1> for P1 {
    type Output = P1;
    fn bitor(self, _: P1) -> Self::Output {
        P1
    }
}

// P1 | N1
impl BitOr<N1> for P1 {
    type Output = N1;
    fn bitor(self, _: N1) -> Self::Output {
        N1
    }
}

// P1 | B0
impl<H: NonZero + IfB1> BitOr<B0<H>> for P1 {
    type Output = H::Output;
    fn bitor(self, _rhs: B0<H>) -> Self::Output {
        <H as IfB1>::b1()
    }
}

// P1 | B1
impl<H: NonZero> BitOr<B1<H>> for P1 {
    type Output = B1<H>;
    fn bitor(self, rhs: B1<H>) -> Self::Output {
        rhs
    }
}

// ==================== N1 | All ====================
impl<I: Integer> BitOr<I> for N1 {
    type Output = N1;
    #[inline(always)]
    fn bitor(self, _rhs: I) -> Self::Output {
        N1
    }
}

// ==================== B0 | All ====================
// B0 | Z0
impl<H: NonZero> BitOr<Z0> for B0<H> {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, _rhs: Z0) -> Self::Output {
        self
    }
}

// B0 | P1
impl<H: NonZero + IfB1> BitOr<P1> for B0<H> {
    type Output = H::Output;
    fn bitor(self, _rhs: P1) -> Self::Output {
        <H as IfB1>::b1()
    }
}

// B0 | N1
impl<H: NonZero> BitOr<N1> for B0<H> {
    type Output = N1;
    #[inline(always)]
    fn bitor(self, _rhs: N1) -> Self::Output {
        N1
    }
}

// B0 | B0
impl<H1: NonZero + BitOr<H2, Output: IfB0>, H2: NonZero> BitOr<B0<H2>> for B0<H1> {
    type Output = <H1::Output as IfB0>::Output;
    #[inline(always)]
    fn bitor(self, _rhs: B0<H2>) -> Self::Output {
        <<H1 as BitOr<H2>>::Output as IfB0>::b0()
    }
}

// B0 | B1
impl<H1: NonZero + BitOr<H2, Output: IfB1>, H2: NonZero> BitOr<B1<H2>> for B0<H1> {
    type Output = <H1::Output as IfB1>::Output;
    #[inline(always)]
    fn bitor(self, _rhs: B1<H2>) -> Self::Output {
        <<H1 as BitOr<H2>>::Output as IfB1>::b1()
    }
}

// ==================== B1 | All ====================
// B1 | Z0
impl<H: NonZero> BitOr<Z0> for B1<H> {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, _rhs: Z0) -> Self::Output {
        self
    }
}

// B1 | P1
impl<H: NonZero> BitOr<P1> for B1<H> {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, _rhs: P1) -> Self::Output {
        self
    }
}

// B1 | N1
impl<H: NonZero> BitOr<N1> for B1<H> {
    type Output = N1;
    #[inline(always)]
    fn bitor(self, _rhs: N1) -> Self::Output {
        N1
    }
}

// B1 | B0
impl<H1: NonZero + BitOr<H2, Output: IfB1>, H2: NonZero> BitOr<B0<H2>> for B1<H1> {
    type Output = <H1::Output as IfB1>::Output;
    #[inline(always)]
    fn bitor(self, _rhs: B0<H2>) -> Self::Output {
        <<H1 as BitOr<H2>>::Output as IfB1>::b1()
    }
}

// B1 | B1
impl<H1: NonZero + BitOr<H2, Output: IfB1>, H2: NonZero> BitOr<B1<H2>> for B1<H1> {
    type Output = <H1::Output as IfB1>::Output;
    #[inline(always)]
    fn bitor(self, _rhs: B1<H2>) -> Self::Output {
        <<H1 as BitOr<H2>>::Output as IfB1>::b1()
    }
}