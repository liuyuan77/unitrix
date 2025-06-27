use core::ops::{Sub, Shr, Shl};

use crate::number::{TypedInt, FixedPoint, Float, IfB0, IfB1, NonZero, Primitive, PrimitiveInt, Sub1, Unsigned, Var, B0, B1, N1, P1, Z0};

// ==================== Right Shift Operation (>>) ====================
// ==================== 右移运算（>>） ====================

// ==================== Z0 >> All ====================
// Zero right shifted by any amount is still zero
// 零右移任何位数仍然是零
impl<R: Unsigned> Shr<R> for Z0 {
    type Output = Z0;
    fn shr(self, _: R) -> Self::Output {
        Z0
    }
}

// Zero right shifted by a variable amount is still zero
// 零右移可变位数仍然是零
impl<T: Primitive> Shr<Var<T>> for Z0 {
    type Output = Z0;
    fn shr(self, _: Var<T>) -> Self::Output {
        Z0
    }
}

// ==================== P1 >> U ====================
// Positive one right shifted by zero is itself
// 正一右移零位是其本身
impl Shr<Z0> for P1 {
    type Output = Self;
    fn shr(self, _: Z0) -> Self::Output {
        self
    }
}

// Positive one right shifted by any non-zero unsigned number becomes zero
// 正一右移任何非零无符号数变为零
impl<R: Unsigned + NonZero> Shr<R> for P1 {
    type Output = Z0;
    fn shr(self, _: R) -> Self::Output {
        Z0
    }
}

impl<T: PrimitiveInt + From<P1> + Shr<Output=T>> Shr<Var<T>> for P1 {
    type Output = Var<T>;
    fn shr(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(self) >> rhs.0)
    }
}

// ==================== N1 >> U ====================
// Negative one right shifted by any unsigned number remains negative one
// (due to sign extension in two's complement arithmetic right shift)
// 负一右移任何无符号数仍然是负一（由于二进制补码算术右移中的符号扩展）
impl<R: Unsigned> Shr<R> for N1 {
    type Output = Self;
    fn shr(self, _: R) -> Self::Output {
        self
    }
}

impl<T: PrimitiveInt> Shr<Var<T>> for N1 {
    type Output = Self;
    fn shr(self, _rhs: Var<T>) -> Self::Output {
        self
    }
}

// ==================== B0 >> U ====================
// Binary number ending with 0 right shifted by zero is itself
// 以0结尾的二进制数右移零位是其本身
impl<H: NonZero> Shr<Z0> for B0<H> {
    type Output = Self;
    fn shr(self, _: Z0) -> Self::Output {
        self
    }
}

// Binary number ending with 0 right shifted by more than one
// Recursively shifts right by one until the shift amount is zero
// (equivalent to dropping the least significant bit and shifting the rest)
// 以0结尾的二进制数右移多于一位
// 递归地右移一位直到移位量为零（相当于丢弃最低有效位并右移其余位）
impl<H: NonZero + Default + Shr<<R as Sub1>::Output>, R: Unsigned + NonZero + Sub1> Shr<R> for B0<H>{
    type Output = <  H as Shr< <R as Sub1>::Output >  >::Output;
    fn shr(self, r: R) -> Self::Output {
        H::default() >> r.sub1()
    }
}

impl<H: NonZero, T: PrimitiveInt + From<B0<H>> + Shr<Output=T>> Shr<Var<T>> for B0<H> {
    type Output = Var<T>;
    fn shr(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(self) >> rhs.0)
    }
}

// ==================== B1 >> U ====================
// Binary number ending with 1 right shifted by zero is itself
// 以1结尾的二进制数右移零位是其本身
impl<H: NonZero> Shr<Z0> for B1<H> {
    type Output = Self;
    fn shr(self, _: Z0) -> Self::Output {
        self
    }
}

// Binary number ending with 1 right shifted by more than one
// Recursively shifts right by one until the shift amount is zero,
// and maintains the sign bit
// 以1结尾的二进制数右移多于一位
// 递归地右移一位直到移位量为零，并保持符号位
impl<H: NonZero + Default + Shr<<R as Sub1>::Output>, R: Unsigned + NonZero + Sub1> Shr<R> for B1<H>{
    type Output = <  H as Shr< <R as Sub1>::Output >  >::Output;
    fn shr(self, r: R) -> Self::Output {
         H::default() >> r.sub1()
    }
}

impl<H: NonZero, T: PrimitiveInt + From<B1<H>> + Shr<Output=T>> Shr<Var<T>> for B1<H> {
    type Output = Var<T>;
    fn shr(self, rhs: Var<T>) -> Self::Output {
        Var(T::from(self) >> rhs.0)
    }
}

// ==================== FixedPoint<IntPart, FracPart> >> All ====================
// ==================== 定点数右移实现 ====================

// Fixed-point number right shift by zero - no change
// 定点数右移零位 - 无变化
impl<IntPart, FracPart> Shr<Z0> for FixedPoint<IntPart, FracPart> {
    type Output = Self;
    fn shr(self, _: Z0) -> Self::Output {
        self
    }
}

// Fixed-point with zero integer part right shift
// Shifting affects only the fractional part
// 整数部分为零的定点数右移
// 移位仅影响小数部分
impl<FracPart: TypedInt + Shl<R>, R: Unsigned + NonZero> Shr<R> for FixedPoint<Z0, FracPart> { //FracPart是反向存储的
    type Output = FixedPoint<Z0, <FracPart as Shl<R>>::Output>;
    
    fn shr(self, _r: R) -> Self::Output {
        FixedPoint::new()
    }
}

// Fixed-point with positive one integer part right shift
// Shifting moves bits from integer to fractional part
// 整数部分为正一的定点数右移
// 移位将位从整数部分移动到小数部分
impl<FracPart: TypedInt + IfB1, R: Unsigned + NonZero + Sub1> Shr<R> for FixedPoint<P1, FracPart>
where
    FixedPoint< Z0, <FracPart as IfB1>::Output > : Shr<<R as Sub1>::Output>,
{
    type Output = <  FixedPoint< Z0, <FracPart as IfB1>::Output > as Shr<<R as Sub1>::Output>  >::Output;
    
    fn shr(self, r: R) -> Self::Output {
        FixedPoint::new() >> r.sub1()
    }
}

// Fixed-point with negative one integer part right shift
// Arithmetic right shift preserves the sign bit
// 整数部分为负一的定点数右移
// 算术右移保留符号位
impl<FracPart: IfB1, R> Shr<R> for FixedPoint<N1, FracPart>
where
    R: Unsigned + NonZero + Sub1,
    <FracPart as IfB1>::Output : Shr<<R as Sub1>::Output>,  // 递归条件
{
    type Output = FixedPoint<
        N1, <<FracPart as IfB1>::Output as Shr<<R as Sub1>::Output> >::Output
    >;
    
    fn shr(self, _r: R) -> Self::Output {
        FixedPoint::new()
    }
}

// Fixed-point with binary integer part ending with 0 right shift
// 整数部分以0结尾的二进制定点数右移
impl<H:NonZero, FracPart: IfB0, R: Unsigned + NonZero + Sub1> Shr<R> for FixedPoint<B0<H>, FracPart>
where
    FixedPoint<H,<FracPart as IfB0>::Output>: Shr<<R as Sub1>::Output>,  // 递归条件
{
    type Output = < FixedPoint<H,<FracPart as IfB0>::Output> as Shr<<R as Sub1>::Output> >::Output;
    
    fn shr(self, r: R) -> Self::Output {
        // 递归处理：先移1位，再移n-1位
        FixedPoint::new() >> r.sub1()
    }
}

// Fixed-point with binary integer part ending with 1 right shift
// 整数部分以1结尾的二进制定点数右移
impl<H:NonZero, FracPart: IfB1, R: Unsigned + NonZero + Sub1> Shr<R> for FixedPoint<B1<H>, FracPart>
where
    FixedPoint<H,<FracPart as IfB1>::Output>: Shr<<R as Sub1>::Output>,  // 递归条件
{
    type Output = < FixedPoint<H,<FracPart as IfB1>::Output> as Shr<<R as Sub1>::Output> >::Output;
    
    fn shr(self, r: R) -> Self::Output {
        // 递归处理：先移1位，再移n-1位
        FixedPoint::new() >> r.sub1()
    }
}

// ==================== Float >> U ====================
// Floating-point number right shift
// Effectively decreases the exponent (equivalent to division by 2^r)
// 浮点数右移
// 实际上是减小指数（相当于除以2^r）
impl<Significand, Exponent, R> Shr<R> for Float<Significand, Exponent>
where
    R: Unsigned + NonZero,
    Exponent: Sub<R>, 
{
    type Output = Float<Significand, <Exponent as Sub<R>>::Output>;
    
    fn shr(self, _r: R) -> Self::Output {
        Float::new()
    }
}

impl<T, R> Shr<R> for Var<T>
where
    T: PrimitiveInt + From<R> + Shr<Output=T>,
    R: Unsigned,
{
    type Output = Var<T>;
    
    fn shr(self, rhs: R) -> Self::Output {
        Var(self.0 >> T::from(rhs))
    }
}

// ==================== 辅助类型 ====================
/// 类型别名：右移一位的结果
#[allow(dead_code)]
pub type Shr1<I> = <I as Shr<P1>>::Output;