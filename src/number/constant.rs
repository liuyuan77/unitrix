use core::marker::PhantomData;

use crate::sealed::Sealed;

// ========== 基础类型定义 ==========
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct B0<H>(pub PhantomData<H>);

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct B1<H>(pub PhantomData<H>);

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct Z0;

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct P1;//新增

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct N1;

impl<H> Default for B0<H>{
    fn default() -> Self{
        B0(PhantomData)
    }
}

impl<H> Default for B1<H>{
    fn default() -> Self{
        B1(PhantomData)
    }
}

// ========== Sealed 实现 ==========
impl<H> Sealed for B0<H> {}
impl<H> Sealed for B1<H> {}
impl Sealed for Z0 {}
impl Sealed for P1 {}
impl Sealed for N1 {}

// ========== 标记特质定义 ==========
pub trait Integer: Default+Sealed + Copy + 'static {
    fn to_i32() -> i32;
}
pub trait NonZero: Integer {}
pub trait NonOne: Integer {}
pub trait NonNegOne: Integer {}
pub trait Unsigned: Integer {}

// ========== NonZero 实现 ==========
impl NonZero for P1 {}
impl NonZero for N1 {}
impl<H: NonZero> NonZero for B0<H> {}
impl<H: NonZero> NonZero for B1<H> {}

// ========== Integer 实现 ==========
// Z0 表示 0
impl Integer for Z0 {
    #[inline(always)]
    fn to_i32() -> i32 {
        0
    }
}

// P1 表示 +1
impl Integer for P1 {
    #[inline(always)]
    fn to_i32() -> i32 {
        1
    }
}

// N1 表示 -1
impl Integer for N1 {
    #[inline(always)]
    fn to_i32() -> i32 {
        -1
    }
}

// B0<H> 表示 H * 2
impl<H: NonZero> Integer for B0<H> {
    #[inline(always)]
    fn to_i32() -> i32 {
        H::to_i32() * 2
    }
}

// B1<H> 表示 H * 2 + 1
impl<H: NonZero> Integer for B1<H> {
    #[inline(always)]
    fn to_i32() -> i32 {
        H::to_i32() * 2 + 1
    }
}

// ========== NonOne 实现 ==========
impl NonOne for Z0 {}
impl NonOne for N1 {}
impl<H: NonZero> NonOne for B0<H> {}
impl<H: NonZero> NonOne for B1<H> {}

// ========== NonNegOne 实现 ==========
impl NonNegOne for Z0 {}
impl NonNegOne for P1 {}
impl<H: NonZero> NonNegOne for B0<H> {}
impl<H: NonZero> NonNegOne for B1<H> {}

// ========== Unsigned 实现 ==========
impl Unsigned for Z0 {}
impl Unsigned for P1 {}
impl<H: NonZero + NonNegOne> Unsigned for B0<H> {}
impl<H: NonZero + NonNegOne> Unsigned for B1<H> {}

// ========== 构造函数 ==========
impl<H> B0<H> {
    #[inline]
    pub fn new() -> Self {
        B0(PhantomData)
    }
}

impl<H> B1<H> {
    #[inline]
    pub fn new() -> Self {
        B1(PhantomData)
    }
}

impl P1 {
    #[inline]
    pub fn new() -> Self {
        P1
    }
}

impl N1 {
    #[inline]
    pub fn new() -> Self {
        N1
    }
}

impl Z0 {
    #[inline]
    pub fn new() -> Self {
        Z0
    }
}