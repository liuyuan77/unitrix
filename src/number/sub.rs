use core::ops::{Neg, Not, Sub};
use crate::number::{Z0, N1, P1, B0, B1, TypedInt, NonZero};
use super::add1::Add1;
use super::sub1::Sub1;
use super::standardization::{IfB0, IfB1};
use crate::number::{Var,Primitive};

// ==================== 带借位减法 Trait ====================
/// 带借位减法运算
/// Subtraction with borrow operation
///
/// 表示 a - b - 1 的运算 (相当于 a - (b + 1))
/// Represents the operation of a - b - 1 (equivalent to a - (b + 1))
/// 说明：有借位表示有低位数，本位 NonZero
pub trait SubWithBorrow<Rhs> {
    type Output;
}

// ==================== 带借位减法实现 ====================

// ========== 带借位P1 - NonZero ==========
 // P1 - I (带借位) = P1-I-1=-I
impl<I:NonZero + Neg> SubWithBorrow<I> for P1 {
    type Output = I::Output;
}

// ========== 带借位N1 - NonZero ==========
// N1 - I (带借位,非0) = !I-1 (即N1+（!I+1）-1)
impl<I: NonZero + Not> SubWithBorrow<I> for N1
where
    <I as Not>::Output:Sub1,
{
    type Output = <I::Output as Sub1>::Output;
}

// ========== 带借位B0 - NonZero ==========
// B0 - P1 (带借位)
impl<H: NonZero + Sub1> SubWithBorrow<P1> for B0<H>
where
    H::Output: IfB0,
{
    type Output = <H::Output as IfB0>::Output;
}

// B0 - N1 (带借位)
impl<H: NonZero> SubWithBorrow<N1> for B0<H>
{
    type Output = Self;
}

// B0 - B0 (带借位)
impl<H1: NonZero + SubWithBorrow<H2>, H2: NonZero> SubWithBorrow<B0<H2>> for B0<H1>
where
    H1::Output: IfB1,
{
    type Output = <H1::Output as IfB1>::Output;
}

// B0 - B1 (带借位)
impl<H1: NonZero + SubWithBorrow<H2>, H2: NonZero> SubWithBorrow<B1<H2>> for B0<H1>
where
    <H1 as SubWithBorrow<H2>>::Output: IfB0,
{
    type Output = <H1::Output as IfB0>::Output;
}

// ========== 带借位B1 - NonZero ==========
// B1 - P1 (带借位)
impl<H: NonZero + Sub1> SubWithBorrow<P1> for B1<H>
where
    H::Output: IfB1,
{
    type Output = <H::Output as IfB1>::Output;
}

// B1 - N1 (带借位)
impl<H: NonZero> SubWithBorrow<N1> for B1<H>
{
    type Output = Self;
}

// B1 - B0 (带借位)
impl<H1: NonZero + Sub<H2>, H2: NonZero> SubWithBorrow<B0<H2>> for B1<H1>
where
    <H1 as Sub<H2>>::Output: IfB0,
{
    type Output = <H1::Output as IfB0>::Output;
}

// B1 - B1 (带借位)
impl<H1: NonZero + SubWithBorrow<H2>, H2: NonZero> SubWithBorrow<B1<H2>> for B1<H1>
where
    <H1 as SubWithBorrow<H2>>::Output: IfB1,
{
    type Output = <H1::Output as IfB1>::Output;
}

// ==================== 标准减法实现 (Sub trait) ====================


// ========== P1 - All ==========
// P1 - I = -(I-P1)
impl<I: TypedInt + Sub1> Sub<I> for P1
where
    <I as Sub1>::Output: Neg,
{
    type Output = <I::Output as Neg>::Output;
    #[inline(always)]
    fn sub(self, i: I) -> Self::Output {
       -i.sub1()
    }
}

// ========== N1 - All ==========
// N1 - I = -(I+P1)
impl<I: TypedInt + Add1> Sub<I> for N1
where
    <I as Add1>::Output: Neg,
{
    type Output = <I::Output as Neg>::Output;
    #[inline(always)]
    fn sub(self, i: I) -> Self::Output {
       -i.add1()
    }
}

// ========== B0 - All ==========
// B0 - Z0
impl<H: NonZero> Sub<Z0> for B0<H> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, _: Z0) -> Self::Output {
        self
    }
}

// B0 - P1
impl<H: NonZero> Sub<P1> for B0<H>
where
    B0<H>:Sub1,
{
    type Output = <B0<H> as Sub1>::Output;
    #[inline(always)]
    fn sub(self, _: P1) -> Self::Output {
        self.sub1()
    }
}

// B0 - N1
impl<H: NonZero> Sub<N1> for B0<H>
where
    B0<H>:Add1,
{
    type Output = <B0<H> as Add1>::Output;
    #[inline(always)]
    fn sub(self, _: N1) -> Self::Output {
        self.add1()
    }
}

// B0 - B0
impl<H1: NonZero + Sub<H2>, H2: NonZero> Sub<B0<H2>> for B0<H1>
where
    <H1 as Sub<H2>>::Output: IfB0,
{
    type Output = <H1::Output as IfB0>::Output;
    #[inline(always)]
    fn sub(self, _: B0<H2>) -> Self::Output {
        <<H1 as Sub<H2>>::Output as IfB0>::b0()
    }
}

// B0 - B1 (需要借位)
impl<H1: NonZero + SubWithBorrow<H2>, H2: NonZero> Sub<B1<H2>> for B0<H1>
where
    <H1 as SubWithBorrow<H2>>::Output: IfB1,
{
    type Output = <H1::Output as IfB1>::Output;
    #[inline(always)]
    fn sub(self, _: B1<H2>) -> Self::Output {
        <<H1 as SubWithBorrow<H2>>::Output as IfB1>::b1()
    }
}

// ========== B1 - All ==========
// B1-Z0
impl<H: NonZero> Sub<Z0> for B1<H> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, _: Z0) -> Self::Output {
        self
    }
}

// B1 - P1
impl<H: NonZero> Sub<P1> for B1<H>
where
    B1<H>:Sub1,
{
    type Output = <B1<H> as Sub1>::Output;
    #[inline(always)]
    fn sub(self, _: P1) -> Self::Output {
        self.sub1()
    }
}

// B1 - N1
impl<H: NonZero + Add1> Sub<N1> for B1<H>
where 
    B1<H>: Add1,
{
    type Output = <B1<H> as Add1>::Output;
    #[inline(always)]
    fn sub(self, _: N1) -> Self::Output {
        self.add1()
    }
}

// B1 - B0
impl<H1: NonZero + Sub<H2,Output: IfB1>, H2: NonZero> Sub<B0<H2>> for B1<H1>{
    type Output = <H1::Output as IfB1>::Output;
    #[inline(always)]
    fn sub(self, _: B0<H2>) -> Self::Output {
        < <H1 as Sub<H2>>::Output as IfB1 >::b1()
    }
}

// B1 - B1
impl<H1: NonZero + Sub<H2,Output: IfB0>, H2: NonZero> Sub<B1<H2>> for B1<H1>{
    type Output = <H1::Output as IfB0>::Output;
    #[inline(always)]
    fn sub(self, _: B1<H2>) -> Self::Output {
        // 获取 H1 - H2 的结果类型
        < <H1 as Sub<H2> >::Output as IfB0>::b0()
    }
}

// ==================== 与Var<T>运算符重载 ====================



// ==================== P1 - Var<T> ====================
impl<T: Primitive> Sub<Var<T>> for P1{
    type Output = Var<T>;
    #[inline(always)]
    fn sub(self, rhs: Var<T>) -> Self::Output {
        Var(1.into()) - rhs
    }
}

// ==================== N1 - Var<T> ====================
impl<T: Primitive> Sub<Var<T>> for N1
where
    Var<T>: Neg,
    <Var<T> as Neg>::Output: Sub1,
{
    type Output = Var<T>;
    #[inline(always)]
    fn sub(self, rhs: Var<T>) -> Self::Output {
        Var((-1).into()) - rhs
    }
}

// ==================== B0 - Var<T> ====================
// B0 - Var<T>
impl<T: Primitive, H: NonZero> Sub<Var<T>> for B0<H>
where
    B0<H>:TypedInt
{
    type Output = Var<T>;
    #[inline(always)]
    fn sub(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(B0::<H>::to_i32()) - rhs.0)
    }
}

// ==================== B1 - Var<T> ====================
// B1 - Var<T>
impl<T: Primitive, H: NonZero> Sub<Var<T>> for B1<H>
where
    B1<H>:TypedInt
{
    type Output = Var<T>;
    #[inline(always)]
    fn sub(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(B1::<H>::to_i32()) - rhs.0)
    }
}