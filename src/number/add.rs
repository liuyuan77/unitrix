// 类型级别二进制数加法实现
/// Type-level binary number addition implementation
///
/// 本模块提供类型级别的二进制数加法运算，包括：
/// This module provides type-level binary number addition operations, including:
/// - 基本加法 (Basic addition)
/// - 带进位加法 (Addition with carry)
/// - 结果标准化处理 (Result standardization)
use core::ops::Add;
use crate::number::{Z0, P1, N1, B0, B1, Add1, Sub1, Integer, NonZero, IfB0,IfB1, Var,Numeric};

// ==================== 带进位加法 Trait ====================
// ==================== Addition With Carry Trait ====================

/// 带进位加法运算
/// Addition with carry operation
///
/// 表示 a + b + 1 的运算
/// Represents the operation of a + b + 1
/// 说明：有进位，说明有低位数，目前B1<Z0>已经被P1替换，本位NonZero

pub trait AddWithCarry<Rhs> {// NonZero
    type Output;
}

// ========== 带进位P1 + NonZero ==========
//  带进位 P1 + All
impl<I:NonZero+ Add<B0<P1>>> AddWithCarry<I> for P1{
    type Output = I::Output;
}

// ========== 带进位N1 + NonZero ==========
impl<I:NonZero> AddWithCarry<I> for N1 {
    type Output = I;
}

// ========== 带进位B0 + NonZero ==========
// B0 + P1
impl<H: NonZero + Add1<Output: IfB0>> AddWithCarry<P1> for B0<H>{//避免B1<N1>,需要特化
    type Output = <H::Output as IfB0>::Output;
}

// B0 + N1
impl<H: NonZero> AddWithCarry<N1> for B0<H>{
    type Output = Self;
}

// B0 + B0
impl<H1: NonZero + IfB1,H2: NonZero> AddWithCarry<B0<H2>> for B0<H1>{
    type Output = H1::Output;
}

// B0 + B1
impl<H1: NonZero + AddWithCarry<H2, Output: IfB0>,H2: NonZero> AddWithCarry<B1<H2>> for B0<H1>{
    type Output = <H1::Output as IfB0>::Output;
}

// ========== 带进位B1 + NonZero ==========

// B1 + P1
impl<H: NonZero + Add1<Output: IfB1>> AddWithCarry<P1> for B1<H>{
    type Output = < <H as Add1>::Output as IfB1 >::Output;
}

// B1 + N1
impl<H: NonZero + Add1> AddWithCarry<N1> for B1<H>{// 不变
    type Output = Self;
}

// B1 + B0
impl<H1: NonZero + AddWithCarry<H2, Output: IfB0>,H2: NonZero> AddWithCarry<B0<H2>> for B1<H1>{
    type Output = <H1::Output as IfB0>::Output;
}

// B1 + B1
impl<H1: NonZero + AddWithCarry<H2,Output: IfB1>,H2: NonZero> AddWithCarry<B1<H2>> for B1<H1>{
    type Output = <H1::Output as IfB1>::Output;
}

// ==================== 运算符重载 ====================

// ==================== Z0 + All ====================
// Z0 + 整数
impl<I: Integer> Add<I> for Z0 {
    type Output = I;
    #[inline(always)]
    fn add(self, rhs: I) -> Self::Output {
        rhs
    }
}

// ==================== P1 + All ====================
impl<I: Integer + Add1> Add<I> for P1 {
    type Output = I::Output;
    #[inline(always)]
    fn add(self, rhs: I) -> Self::Output {
        rhs.add1()
    }
}

// ==================== N1 + All ====================
impl<I: Integer + Sub1> Add<I> for N1 {
    type Output = I::Output;
    #[inline(always)]
    fn add(self, rhs: I) -> Self::Output {
        rhs.sub1()
    }
}

// ==================== B0 + All ====================
// B0 + Z0
impl<H: NonZero> Add<Z0> for B0<H> {
    type Output = Self;
    #[inline(always)]
    fn add(self, _rhs: Z0) -> Self::Output {
        self
    }
}

// B0 + P1
impl<H:  NonZero> Add<P1> for B0<H>
where 
    B0<H>: Add1,
{
    type Output = <B0<H> as Add1>::Output;
    #[inline(always)]
    fn add(self, _rhs: P1) -> Self::Output {
        self.add1()
    }
}

// B0 + N1
impl<H: NonZero> Add<N1> for B0<H>
where
    B0<H>: Sub1,
{
    type Output = <B0<H> as Sub1>::Output;
    #[inline(always)]
    fn add(self, _rhs: N1) -> Self::Output {
        self.sub1()
    }
}

// B0 + B0
impl<H1: NonZero + Add<H2, Output: IfB0> + , H2: NonZero> Add<B0<H2>> for B0<H1>{
    type Output = <H1::Output as IfB0>::Output;
    #[inline(always)]
    fn add(self, _rhs: B0<H2>) -> Self::Output {
        <<H1 as Add<H2>>::Output as IfB0>::b0()
    }
}

// B0 + B1
impl<H1: NonZero + Add<H2,Output: IfB1>, H2: NonZero> Add<B1<H2>> for B0<H1>{
    type Output = <H1::Output as IfB1>::Output;
    #[inline(always)]
    fn add(self, _rhs: B1<H2>) -> Self::Output {
        <<H1 as Add<H2>>::Output as IfB1>::b1()
    }
}

// ==================== B1 + All ====================
// B1 + Z0
impl<H: NonZero> Add<Z0> for B1<H> {
    type Output = Self;
    #[inline(always)]
    fn add(self, _rhs: Z0) -> Self::Output {
        self
    }
}

// B1 + P1
impl<H:  NonZero> Add<P1> for B1<H>
where
    B1<H>: Add1,
{
    type Output = <B1<H> as Add1>::Output;
    #[inline(always)]
    fn add(self, _rhs: P1) -> Self::Output {
        self.add1()
    }
}

// B1 + N1
impl<H: NonZero> Add<N1> for B1<H>
where 
    B1<H>: Sub1

{
    type Output = <B1<H> as Sub1>::Output;
    #[inline(always)]
    fn add(self, _rhs: N1) -> Self::Output {
        self.sub1()
    }
}

// B1 + B0
impl<H1: NonZero + Add<H2> + , H2: NonZero> Add<B0<H2>> for B1<H1>
where
    <H1 as Add<H2>>::Output: IfB1,
{
    type Output = <H1::Output as IfB1>::Output;
    #[inline(always)]
    fn add(self, _rhs: B0<H2>) -> Self::Output {
        <<H1 as Add<H2>>::Output as IfB1>::b1()
    }
}

// B1 + B1
impl<H1: NonZero + AddWithCarry<H2, Output: IfB0>, H2: NonZero> Add<B1<H2>> for B1<H1>{
    type Output = <H1::Output as IfB0>::Output;
    #[inline(always)]
    fn add(self, _rhs: B1<H2>) -> Self::Output {
        <<H1 as AddWithCarry<H2>>::Output as IfB0>::b0()
    }
}

// ==================== 与Var<T>运算符重载 ====================

// ==================== Z0 + Var<T> ====================
// Z0 + Var<T>
impl<T: Numeric> Add<Var<T>> for Z0 {
    type Output = Var<T>;
    #[inline(always)]
    fn add(self, rhs: Var<T>) -> Self::Output {
        rhs
    }
}

// ==================== P1 + Var<T> ====================
impl<T: Numeric + Add1> Add<Var<T>> for P1 {
    type Output = Var<T>;
    #[inline(always)]
    fn add(self, rhs: Var<T>) -> Self::Output {
        rhs.add1()
    }
}

// ==================== N1 + Var<T> ====================
impl<T: Numeric + Sub1> Add<Var<T>> for N1 {
    type Output = Var<T>;
    #[inline(always)]
    fn add(self, rhs: Var<T>) -> Self::Output {
        rhs.sub1()
    }
}

// ==================== B0 + Var<T> ====================
// B0 + Var<T>
impl<T: Numeric, H: NonZero> Add<Var<T>> for B0<H>
where
    B0<H>:Integer
{
    type Output = Var<T>;
    #[inline(always)]
    fn add(self, rhs: Var<T>) -> Self::Output {
        Var(rhs.0 + T::from(B0::<H>::to_i32()))
    }
}

// ==================== B1 + Var<T> ====================
// B1 + Var<T>
impl<T: Numeric, H: NonZero> Add<Var<T>> for B1<H>
where
    B1<H>:Integer
{
    type Output = Var<T>;
    #[inline(always)]
    fn add(self, rhs: Var<T>) -> Self::Output {
        Var(rhs.0 + T::from(B1::<H>::to_i32()))
    }
}