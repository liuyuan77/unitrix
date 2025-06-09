use super::basic::{Z0, P1, N1, B0, B1, NonZero, NonOne, Unsigned};
use super::sub1::Sub1;
use core::ops::Shl;

// ==================== 左移运算（<<） ====================
// Z0 << U
impl<R: Unsigned> Shl<R> for Z0 {
    type Output = Z0;
    fn shl(self, _: R) -> Self::Output {
        Z0  // 0 << n = 0
    }
}

// P1 << U
impl Shl<Z0> for P1 {// P1 << Z0
    type Output = Self;
    fn shl(self, _: Z0) -> Self::Output {
        self
    }
}

impl Shl<P1> for P1 {// P1 << P1
    type Output = B0<P1>;
    fn shl(self, _: P1) -> Self::Output {
        B0::new()
    }
}

impl<R: Unsigned + NonZero + NonOne + Sub1> Shl<R> for P1
where
    P1: Shl<<R as Sub1>::Output>
{// P1 << 超过1的数
    type Output = B0<<P1 as Shl<R::Output>>::Output>;
    fn shl(self, _: R) -> Self::Output {
        B0::new()
    }
}

// N1 << U
impl Shl<Z0> for N1 {// N1 << Z0
    type Output = Self;
    fn shl(self, _: Z0) -> Self::Output {
        self
    }
}

impl Shl<P1> for N1 {// N1 << P1
    type Output = B0<N1>;
    fn shl(self, _: P1) -> Self::Output {
        B0::new()
    }
}

impl<R: Unsigned + NonZero + NonOne + Sub1> Shl<R> for N1
where
    N1: Shl<<R as Sub1>::Output>
{// P1 << 超过1的数
    type Output = B0<<N1 as Shl<R::Output>>::Output>;
    fn shl(self, _: R) -> Self::Output {
        B0::new()
    }
}

// B0 << U
impl<H: NonZero> Shl<Z0> for B0<H> {// B0 << Z0
    type Output = Self;
    fn shl(self, _: Z0) -> Self::Output {
        self
    }
}

impl<H: NonZero> Shl<P1> for B0<H> {// B0 << P1
    type Output = B0<B0<H>>;
    fn shl(self, _: P1) -> Self::Output {
        B0::new()
    }
}

impl<H: NonZero, R: Unsigned + NonZero + NonOne + Sub1> Shl<R> for B0<H>
where
    B0<H>: Shl<<R as Sub1>::Output>
{// B0 << 超过1的数
    type Output = B0<<B0<H> as Shl<R::Output>>::Output>;
    fn shl(self, _: R) -> Self::Output {
        B0::new()
    }
}

// B1 << U
impl<H: NonZero> Shl<Z0> for B1<H> {// B1 << Z0
    type Output = Self;
    fn shl(self, _: Z0) -> Self::Output {
        self
    }
}

impl<H: NonZero> Shl<P1> for B1<H> {// B1 << P1
    type Output = B0<B1<H>>;
    fn shl(self, _: P1) -> Self::Output {
        B0::new()
    }
}

impl<H: NonZero, R: Unsigned + NonZero + NonOne + Sub1> Shl<R> for B1<H>
where
    B1<H>: Shl<<R as Sub1>::Output>
{// B1 << 超过1的数
    type Output = B0<<B1<H> as Shl<R::Output>>::Output>;
    fn shl(self, _: R) -> Self::Output {
        B0::new()
    }
}

#[allow(dead_code)]
pub type Shl1<I>=<I as Shl<P1>>::Output;