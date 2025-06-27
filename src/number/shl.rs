use core::ops::Shl;
use crate::number::{FixedPoint, Float, IfB0, IfB1, NonZero, Primitive, PrimitiveInt, Sub1, TypedInt, Unsigned, Var, B0, B1, N1, P1, Z0};

// ==================== Left Shift Operation (<<) ====================
// ==================== 左移运算（<<） ====================

// ==================== Z0 << All ====================
// Zero left shifted by any amount is still zero
// 零左移任何位数仍然是零
impl<R: Unsigned> Shl<R> for Z0 {
    type Output = Z0;
    fn shl(self, _: R) -> Self::Output {
        Z0
    }
}

// Zero left shifted by a variable amount is still zero
// 零左移可变位数仍然是零
impl<T: Primitive> Shl<Var<T>> for Z0 {
    type Output = Z0;
    fn shl(self, _: Var<T>) -> Self::Output {
        Z0
    }
}

// ==================== P1 << U ====================
// Positive one left shifted by zero is itself
// 正一左移零位是其本身
impl Shl<Z0> for P1 {
    type Output = Self;
    fn shl(self, _: Z0) -> Self::Output {
        self
    }
}

// Positive one left shifted by more than one
// Recursively shifts left by one until the shift amount is zero
// (equivalent to adding zeros at the least significant side)
// 正一左移多于一位
// 递归地左移一位直到移位量为零（相当于在最低有效位侧添加零）
impl<R: Unsigned + NonZero + Sub1> Shl<R> for P1
where
    P1: Shl<<R as Sub1>::Output>,
{
    type Output = B0<<P1 as Shl<<R as Sub1>::Output>>::Output>;
    fn shl(self, _r: R) -> Self::Output {
        B0::new()
    }
}

// Positive one left shifted by a variable amount
// 正一左移可变位数
impl<T: PrimitiveInt + From<P1> + Shl<Output=T>> Shl<Var<T>> for P1 {
    type Output = Var<T>;
    fn shl(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(self) << rhs.0)
    }
}

// ==================== N1 << U ====================
// Negative one left shifted by zero is itself
// 负一左移零位是其本身
impl Shl<Z0> for N1 {
    type Output = Self;
    fn shl(self, _: Z0) -> Self::Output {
        self
    }
}

// Negative one left shifted by more than one
// Recursively shifts left by one until the shift amount is zero
// (maintains sign in two's complement arithmetic)
// 负一左移多于一位
// 递归地左移一位直到移位量为零（在二进制补码算术中保持符号）
impl<R: Unsigned + NonZero + Sub1> Shl<R> for N1
where
    N1: Shl<<R as Sub1>::Output>,
{
    type Output = B0<<N1 as Shl<<R as Sub1>::Output>>::Output>;
    fn shl(self, _r: R) -> Self::Output {
        B0::new()
    }
}

// Negative one left shifted by a variable amount
// 负一左移可变位数
impl<T: PrimitiveInt + From<N1> + Shl<Output=T>> Shl<Var<T>> for N1 {
    type Output = Var<T>;
    fn shl(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(self) << rhs.0)
    }
}

// ==================== B0 << U ====================
// Binary number ending with 0 left shifted by zero is itself
// 以0结尾的二进制数左移零位是其本身
impl<H: NonZero> Shl<Z0> for B0<H> {
    type Output = Self;
    fn shl(self, _: Z0) -> Self::Output {
        self
    }
}

// Binary number ending with 0 left shifted by more than one
// Recursively shifts left by one until the shift amount is zero
// 以0结尾的二进制数左移多于一位
// 递归地左移一位直到移位量为零
impl<H: NonZero, R: Unsigned + NonZero + Sub1> Shl<R> for B0<H>
where 
    B0<H>: Shl<<R as Sub1>::Output>,
{
    type Output = B0<<B0<H> as Shl<<R as Sub1>::Output>>::Output>;
    fn shl(self, _r: R) -> Self::Output {
        B0::new()
    }
}

// Binary number ending with 0 left shifted by a variable amount
// 以0结尾的二进制数左移可变位数
impl<H: NonZero, T: PrimitiveInt + From<B0<H>> + Shl<Output=T>> Shl<Var<T>> for B0<H> {
    type Output = Var<T>;
    fn shl(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(self) << rhs.0)
    }
}

// ==================== B1 << U ====================
// Binary number ending with 1 left shifted by zero is itself
// 以1结尾的二进制数左移零位是其本身
impl<H: NonZero> Shl<Z0> for B1<H> {
    type Output = Self;
    fn shl(self, _: Z0) -> Self::Output {
        self
    }
}

// Binary number ending with 1 left shifted by more than one
// Recursively shifts left by one until the shift amount is zero
// 以1结尾的二进制数左移多于一位
// 递归地左移一位直到移位量为零
impl<H: NonZero, R: Unsigned + NonZero + Sub1> Shl<R> for B1<H>
where 
    B1<H>: Shl<<R as Sub1>::Output>,
{
    type Output = B0<<B1<H> as Shl<<R as Sub1>::Output>>::Output>;
    fn shl(self, _r: R) -> Self::Output {
        B0::new()
    }
}

// Binary number ending with 1 left shifted by a variable amount
// 以1结尾的二进制数左移可变位数
impl<H: NonZero, T: PrimitiveInt + From<B1<H>> + Shl<Output=T>> Shl<Var<T>> for B1<H> {
    type Output = Var<T>;
    fn shl(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(self) << rhs.0)
    }
}

// ==================== FixedPoint<IntPart, FracPart> << All ====================
// ==================== 定点数左移实现 ====================

// Fixed-point number left shifted by zero - no change
// 定点数左移零位 - 无变化
impl<IntPart: TypedInt, FracPart: TypedInt> Shl<Z0> for FixedPoint<IntPart, FracPart> {
    type Output = Self;
    fn shl(self, _: Z0) -> Self::Output {
        self
    }
}

// Fixed-point with zero fractional part left shift
// Shifting affects only the integer part
// 小数部分为零的定点数左移
// 移位仅影响整数部分
impl<IntPart: TypedInt + Shl<R>, R: Unsigned + NonZero> Shl<R> for FixedPoint<IntPart, Z0>
{
    type Output = FixedPoint<
        <IntPart as Shl<R>>::Output,
        Z0
    >;
    
    fn shl(self, _r: R) -> Self::Output {
        FixedPoint::new()
    }
}

// Fixed-point with one fractional part left shift
// Shifting affects only the integer part
// 小数部分为P1的定点数左移
impl<IntPart: IfB1, R: Unsigned + NonZero + Sub1> Shl<R> for FixedPoint<IntPart, P1>
where
    <IntPart as IfB1>::Output: Shl<<R as Sub1>::Output>,  // 递归条件
{
    type Output = FixedPoint<
        <<IntPart as IfB1>::Output as Shl<<R as Sub1>::Output> >::Output,
        Z0
    >;
    
    fn shl(self, _r: R) -> Self::Output {
        FixedPoint::new()
    }
}

// Fixed-point with fractional part ending with 0 left shift
// Shifting moves bit from fractional to integer part
// 小数部分以0结尾的定点数左移
// 移位将位从小数部分移动到整数部分
impl<IntPart: TypedInt + IfB0, L: NonZero, R: Unsigned + NonZero + Sub1> Shl<R> for FixedPoint<IntPart, B0<L>>
where 
    FixedPoint< <IntPart as IfB0>::Output,L >: Shl<<R as Sub1>::Output>,
{
    type Output = <  FixedPoint< <IntPart as IfB0>::Output,L > as Shl<<R as Sub1>::Output>  >::Output;
    
    fn shl(self, r: R) -> Self::Output {
        FixedPoint::new() << r.sub1()
    }
}

// Fixed-point with fractional part ending with 1 left shift
// Shifting moves bit from fractional to integer part
// 小数部分以1结尾的定点数左移
// 移位将位从小数部分移动到整数部分
impl<IntPart: IfB1, L: NonZero, R: Unsigned + NonZero + Sub1> Shl<R> for FixedPoint<IntPart, B1<L>>
where 
    FixedPoint< <IntPart as IfB1>::Output,L >: Shl<<R as Sub1>::Output>,
{
    type Output = <  FixedPoint< <IntPart as IfB1>::Output,L > as Shl<<R as Sub1>::Output>  >::Output;
    
    fn shl(self, r: R) -> Self::Output {
        FixedPoint::new() << r.sub1()
    }
}

/* // Fixed-point left shifted by a variable amount
// 定点数左移可变位数（无意义，取消）
impl<IntPart, FracPart, T> Shl<Var<T>> for FixedPoint<IntPart, FracPart>
where
    T: PrimitiveInt,
    IntPart: Into<T>,
    FracPart: Into<T>,
    Self: Into<T>,
{
    type Output = Var<T>;
    
    fn shl(self, rhs: Var<T>) -> Self::Output {
        Var(self.into() << rhs.0)
    }
} */

// ==================== Float << U ====================
// Floating-point number left shift
// Effectively increases the exponent (equivalent to multiplication by 2^r)
// 浮点数左移
// 实际上是增加指数（相当于乘以2^r）
impl<Significand, Exponent, R> Shl<R> for Float<Significand, Exponent>
where
    R: Unsigned + NonZero,
    Exponent: Shl<R>, 
{
    type Output = Float<Significand, <Exponent as Shl<R>>::Output>;
    
    fn shl(self, _r: R) -> Self::Output {
        Float::new()
    }
}

// ==================== Var << U ====================
impl<T, R> Shl<R> for Var<T>
where
    T: PrimitiveInt + From<R> + Shl<Output=T>,
    R: Unsigned,
{
    type Output = Var<T>;
    
    fn shl(self, rhs: R) -> Self::Output {
        Var(self.0 << T::from(rhs))
    }
}

// ==================== 辅助类型 ====================
/// 类型别名：左移一位的结果
#[allow(dead_code)]
pub type Shl1<I> = <I as Shl<P1>>::Output;