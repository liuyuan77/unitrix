use core::ops::{BitXor,Not};
use crate::number::{B0, B1, Z0, P1, N1, NonZero, TypedInt, IfB0, IfB1};

// ==================== 按位异或（^运算符） ====================


// ==================== P1 ^ All ====================
// P1 ^ Z0
impl BitXor<Z0> for P1 {
    type Output = P1;
    fn bitxor(self, _: Z0) -> Self::Output {
        P1
    }
}

// P1 ^ P1
impl BitXor<P1> for P1 {
    type Output = Z0;
    fn bitxor(self, _: P1) -> Self::Output {
        Z0
    }
}

// P1 ^ N1
impl BitXor<N1> for P1 {
    type Output = B0<N1>;
    fn bitxor(self, _: N1) -> Self::Output {
        B0::new()
    }
}

// P1 ^ B0
impl<H: NonZero + IfB1> BitXor<B0<H>> for P1 {
    type Output = H::Output;
    fn bitxor(self, _rhs: B0<H>) -> Self::Output {
        <H as IfB1>::b1()
    }
}

// P1 ^ B1
impl<H: NonZero + IfB0> BitXor<B1<H>> for P1 {
    type Output = <H as IfB0>::Output;
    fn bitxor(self, _rhs: B1<H>) -> Self::Output {
        <H as IfB0>::b0()
    }
}

// ==================== N1 ^ All ====================
// N1 ^ All
impl<I: TypedInt + Not> BitXor<I> for N1 {
    type Output = <I as Not>::Output;
    fn bitxor(self, rhs: I) -> Self::Output {
        !rhs
    }
}

// ==================== B0 ^ All ====================
// B0 ^ Z0
impl<H: NonZero> BitXor<Z0> for B0<H> {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, _rhs: Z0) -> Self::Output {
        self
    }
}

// B0 ^ P1
impl<H: NonZero + IfB1> BitXor<P1> for B0<H> {
    type Output = H::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: P1) -> Self::Output {
        <H as IfB1>::b1()
    }
}

// B0 ^ N1
impl<H: NonZero> BitXor<N1> for B0<H>
where 
    B0<H>: Not,
{
    type Output = <B0<H> as Not>::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: N1) -> Self::Output {
        !self
    }
}

// B0 ^ B0
impl<H1: NonZero + BitXor<H2, Output: IfB0>, H2: NonZero> BitXor<B0<H2>> for B0<H1> {
    type Output = <H1::Output as IfB0>::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: B0<H2>) -> Self::Output {
        <<H1 as BitXor<H2>>::Output as IfB0>::b0()
    }
}

// B0 ^ B1
impl<H1: NonZero + BitXor<H2, Output: IfB1>, H2: NonZero> BitXor<B1<H2>> for B0<H1> {
    type Output = <H1::Output as IfB1>::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: B1<H2>) -> Self::Output {
        <<H1 as BitXor<H2>>::Output as IfB1>::b1()
    }
}

// ==================== B1 ^ All ====================
// B1 ^ Z0
impl<H: NonZero> BitXor<Z0> for B1<H> {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, _rhs: Z0) -> Self::Output {
        self
    }
}

// B1 ^ P1
impl<H: NonZero + IfB0> BitXor<P1> for B1<H> {
    type Output = H::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: P1) -> Self::Output {
        <H as IfB0>::b0()
    }
}

// B1 ^ N1
impl<H: NonZero> BitXor<N1> for B1<H>
where 
    B1<H>:Not,
{
    type Output = <B1<H> as Not>::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: N1) -> Self::Output {
        !self
    }
}

// B1 ^ B0
impl<H1: NonZero + BitXor<H2, Output: IfB1>, H2: NonZero> BitXor<B0<H2>> for B1<H1> {
    type Output = <H1::Output as IfB1>::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: B0<H2>) -> Self::Output {
        <<H1 as BitXor<H2>>::Output as IfB1>::b1()
    }
}

// B1 ^ B1
impl<H1: NonZero + BitXor<H2, Output: IfB0>, H2: NonZero> BitXor<B1<H2>> for B1<H1> {
    type Output = <H1::Output as IfB0>::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: B1<H2>) -> Self::Output {
        <<H1 as BitXor<H2>>::Output as IfB0>::b0()
    }
}