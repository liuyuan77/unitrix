use core::ops::{Neg, Div};

use crate::number::{B0, B1, P1, N1};
use crate::number::{TypedInt, NonZero};
use crate::number::{Var, Primitive};

// ========== Basic Type Division Operations ==========
// ========== 基本类型除法运算 ==========



// ========== 1 / All ===========
// Division of one by various types
// 1 除以各种类型

// 1 / 0 is illegal and not implemented
// 1 / 0 非法，未实现

// 1 / 1 = 1
impl Div<P1> for P1 {
    type Output = Self;
    #[inline(always)]
    fn div(self, _rhs: Self) -> Self::Output {
        P1
    }
}

// 1/-1=-1
impl Div<N1> for P1 {
    type Output = N1;
    #[inline(always)]
    fn div(self, _rhs: N1) -> Self::Output {
        N1
    }
}

// 1 / B0 has precision loss, returns f64
// 1 / B0 有精度损失，返回f64
impl<H:NonZero> Div<B0<H>> for P1 {
    type Output = f64;
    fn div(self, _: B0<H>) -> f64 {
        f64::from(1) / f64::from(B0::<H>::to_i32())
    }
}

// 1 / B1 has precision loss, returns f64
// 1 / B1 有精度损失，返回f64
impl<H:NonZero> Div<B1<H>> for P1 {
    type Output = f64;
    fn div(self, _: B1<H>) -> f64 {
        f64::from(1) / f64::from(B1::<H>::to_i32())
    }
}

// ========== -1 / All ===========
// Division of negative one by various types
// -1 除以各种类型

// -1 / 0 is illegal and not implemented
// -1 / 0 非法，未实现

// -1 / 1 = -1
impl Div<P1> for N1 {
    type Output = Self;
    #[inline(always)]
    fn div(self, _rhs: P1) -> Self::Output {
        N1
    }
}

// -1/-1=1
impl Div<N1> for N1 {
    type Output = P1;
    #[inline(always)]
    fn div(self, _rhs: N1) -> Self::Output {
        P1
    }
}

// -1 / B0 has precision loss, returns f64
// -1 / B0 有精度损失，返回f64
impl<H:NonZero> Div<B0<H>> for N1 {
    type Output = f64;
    fn div(self, _: B0<H>) -> f64 {
        f64::from(-1) / f64::from(B0::<H>::to_i32())
    }
}

// -1 / B1 has precision loss, returns f64
// -1 / B1 有精度损失，返回f64
impl<H:NonZero> Div<B1<H>> for N1 {
    type Output = f64;
    fn div(self, _: B1<H>) -> f64 {
        f64::from(-1) / f64::from(B1::<H>::to_i32())
    }
}

// ========== B0 / All ===========
// Division of binary type ending with 0 by various types
// 以0结尾的二进制类型除以各种类型

// B0 / 0 is illegal and not implemented
// B0 / 0 非法，未实现

// B0 / 1 = B0
impl<H: NonZero> Div<P1> for B0<H> {
    type Output = Self;
    #[inline(always)]
    fn div(self, _rhs: P1) -> Self::Output {
        self
    }
}

// B0/-1=-B0
impl<H: NonZero> Div<N1> for B0<H>
where 
    B0<H>: Neg,
{
    type Output = <B0<H> as Neg>::Output;
    #[inline(always)]
    fn div(self, _rhs: N1) -> Self::Output {
        -self
    }
}

// B0 / B0 has precision loss, returns f64
// B0 / B0 有精度损失，返回f64
impl<H1:NonZero, H2:NonZero> Div<B0<H2>> for B0<H1> {
    type Output = f64;
    fn div(self, _: B0<H2>) -> f64 {
        f64::from(B0::<H1>::to_i32()) / f64::from(B0::<H2>::to_i32())
    }
}

// B0 / B1 has precision loss, returns f64
// B0 / B1 有精度损失，返回f64
impl<H1:NonZero, H2:NonZero> Div<B1<H2>> for B0<H1> {
    type Output = f64;
    fn div(self, _: B1<H2>) -> f64 {
        f64::from(B0::<H1>::to_i32()) / f64::from(B1::<H2>::to_i32())
    }
}

// ========== B1 / All ===========
// Division of binary type ending with 1 by various types
// 以1结尾的二进制类型除以各种类型

// B1 / 0 is illegal and not implemented
// B1 / 0 非法，未实现

// B1 / 1 = B1
impl<H: NonZero> Div<P1> for B1<H> {
    type Output = Self;
    #[inline(always)]
    fn div(self, _rhs: P1) -> Self::Output {
        self
    }
}

// B1 / -1=-B1
impl<H: NonZero> Div<N1> for B1<H>
where 
    B1<H>: Neg,
{
    type Output = <B1<H> as Neg>::Output;
    #[inline(always)]
    fn div(self, _rhs: N1) -> Self::Output {
        -self
    }
}

// B1 / B0 has precision loss, returns f64
// B1 / B0 有精度损失，返回f64
impl<H1:NonZero, H2:NonZero> Div<B0<H2>> for B1<H1> {
    type Output = f64;
    fn div(self, _: B0<H2>) -> f64 {
        f64::from(B1::<H1>::to_i32()) / f64::from(B0::<H2>::to_i32())
    }
}

// B1 / B1 has precision loss, returns f64
// B1 / B1 有精度损失，返回f64
impl<H1:NonZero, H2:NonZero> Div<B1<H2>> for B1<H1> {
    type Output = f64;
    fn div(self, _: B1<H2>) -> f64 {
        f64::from(B1::<H1>::to_i32()) / f64::from(B1::<H2>::to_i32())
    }
}

// ========== Division with Var<T> ==========
// ========== 与Var<T>的除法运算 ==========


// ========== 1 / Var<T> ==========
impl<T: Primitive + From<P1>> Div<Var<T>> for P1 {
    type Output = Var<T>;
    #[inline(always)]
    fn div(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(P1) / rhs.0)
    }
}

// ========== -1 / Var<T> ==========
impl<T: Primitive + From<N1>> Div<Var<T>> for N1{
    type Output = <Var<T> as Neg>::Output;
    #[inline(always)]
    fn div(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(N1) / rhs.0)
    }
}

// ========== B0 / Var<T> ==========
impl<H: NonZero, T:Primitive + From<B0<H>>> Div<Var<T>> for B0<H>
where 
    B0<H>: TypedInt,
    Var<T>: Div<Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn div(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(self) / rhs.0)
    }
}

// ========== B1 / Var<T> ==========

impl<H: NonZero, T: Primitive + From<B1<H>>> Div<Var<T>> for B1<H>
where 
    B1<H>: TypedInt,
    Var<T>: Div<Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn div(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(self) / rhs.0)
    }
}