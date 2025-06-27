/*
 * 变量结构体(Var)内部方法及重载
 * 该结构体泛型参数 T 需满足 Primitive 约束
 */

use core::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign};
use crate::number::{N1, P1, B0, B1, TypedInt, NonZero, Var, Primitive};


/// 实现 Var 与 Var 的加法运算
/// 用法: V + V
impl<T: Primitive> Add for Var<T> {
    type Output = Self;

    fn add(self, b: Self) -> Self::Output {
        Var(self.0 + b.0) // 内部值相加后包装为新 Var
    }
}

/// 实现 Var 与 Var 的减法运算
/// 用法: V - V
impl<T: Primitive> Sub for Var<T> {
    type Output = Self;

    fn sub(self, b: Self) -> Self::Output {
        Var(self.0 - b.0) // 内部值相减后包装为新 Var
    }
}

/// 实现 Var 与 Var 的乘法运算
/// 用法: V * V
impl<T: Primitive> Mul<Var<T>> for Var<T> {
    type Output = Self;

    fn mul(self, b: Self) -> Self::Output {
        Var(self.0 * b.0) // 内部值相乘后包装为新 Var
    }
}

/// 实现 Var 与 Var 的除法运算
/// 用法：V / V
impl<T: Primitive> Div<Var<T>> for Var<T> {
    type Output = Self;

    fn div(self, b: Self) -> Self::Output {
        Var(self.0 / b.0) // 内部值相除后包装为新 Var
    }
}

/// 实现 Var 的加法赋值运算
/// 用法: V += V
impl<T: Primitive> AddAssign for Var<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;  // 直接转发到底层类型的 += 运算
    }
}

/// 实现 Var 的减法赋值运算
/// 用法: V -= V
impl<T: Primitive> SubAssign for Var<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;  // 直接转发到底层类型的 -= 运算
    }
}

// 与常量运算实现
// =============================================

/// 实现 Var 与常量的加法运算
/// 用法: V + C
impl<T: Primitive, C: TypedInt + Add<Var<T>>> Add<C> for Var<T> {
    type Output = <C as Add<Var<T>>>::Output;

    fn add(self, c:C) -> Self::Output {
        c + self // 交换加法顺序，调用常量的 + 运算符
    }
}

/// 实现 Var 与常量的减法运算
/// 用法: V - C
impl<T: Primitive, C: TypedInt + Neg> Sub<C> for Var<T>
where 
    <C as Neg>::Output: Add<Var<T>>,
{
    type Output = < C::Output as Add<Var<T>> >::Output;

    fn sub(self, c: C) -> Self::Output {
        -c+self // 转换为 -c + V 的形式
    }
}

/// 实现 Var 与常量的乘法运算
/// 用法: V * C
impl<T: Primitive, C:TypedInt + Mul<Var<T>>> Mul<C> for Var<T> {
    type Output = <C as Mul<Var<T>>>::Output;

    fn mul(self, c: C) -> Self::Output {
        c * self // 交换乘法顺序，调用常量的 * 运算符
    }
}

/// 实现 Var 与常量的除法运算（部分实现）

/// V / 0 未实现

/// V / 1 = V
impl<T: Primitive> Div<P1> for Var<T> {
    type Output = Self;

    fn div(self, _rhs: P1) -> Self::Output {
        self
    }
}

/// V / -1 = -V
impl<T: Primitive> Div<N1> for Var<T> {
    type Output = Self;

    fn div(self, _rhs: N1) -> Self::Output {
        -self
    }
}

/// V / B0
impl<H: NonZero + Default, T:Primitive+ From<B0<H>>> Div<B0<H>> for Var<T>
where 
    B0<H>: TypedInt,
    Var<T>: Div<Var<T>,Output = Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn div(self, rhs: B0<H>) -> Self::Output {
        Var(self.0 / T::from(rhs))
    }
}

/// V / B1
impl<H: NonZero + Default, T:Primitive + From<B1<H>>> Div<B1<H>> for Var<T>
where 
    B1<H>: TypedInt,
    Var<T>: Div<Var<T>,Output = Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn div(self, rhs: B1<H>) -> Self::Output {
        Var(self.0 / T::from(rhs))
    }
}

// 类型转换实现
// =============================================

/// 实现 Var 的乘法赋值运算
/// 用法: V *= V
impl<T: Primitive + MulAssign> MulAssign for Var<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0; // 直接转发到底层类型的 *= 运算
    }
}

/// 实现 Var 与底层类型的乘法赋值运算
/// 用法: V *= T
impl<T: Primitive + MulAssign> MulAssign<T> for Var<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs; // 直接使用底层类型的 *= 运算
    }
}