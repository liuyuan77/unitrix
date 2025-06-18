/*
 * 变量结构体 Var
 * 该结构体泛型参数 T 需满足 Primitive 约束
 */

use core::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign};
use crate::number::{N1, P1, B0, B1, TypedInt, NonZero, Var};




/// 定义 Primitive trait，约束 Var泛型参数 T 必须实现基本数值运算
/// 包括：
/// - 一元负号运算 (Neg)
/// - 加减乘除运算 (Add, Sub, Mul, Div)
/// - 复合赋值运算 (AddAssign, SubAssign)
/// - 复制语义 (Copy, Clone)
/// - 默认值 (Default)
/// - 静态生命周期 ('static)
pub trait Primitive:
    Neg<Output = Self> +
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    AddAssign +
    SubAssign +
    Copy +
    Clone +
    Default +
    From<i32> + // 后期取消
    Sized +
    'static
{}

// 为基本类型实现 Primitive trait
impl Primitive for i32 {} // i64 类型实现 Primitive
impl Primitive for i64 {} // i64 类型实现 Primitive
// impl Primitive for f32 {} // f64 类型实现 Primitive
impl Primitive for f64 {} // f64 类型实现 Primitive








// 运算符重载实现
// =============================================

/// 实现 Var 的取反运算
/// 用法: -V
impl<T: Primitive> Neg for Var<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Var(-self.0)   // 对内部值取反后包装为新 Var
    }
}

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
impl<H: NonZero + Default, T:Primitive> Div<B0<H>> for Var<T>
where 
    B0<H>: TypedInt,
    Var<T>: Div<Var<T>,Output = Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn div(self, _rhs: B0<H>) -> Self::Output {
        Var(self.0 / T::from(B0::<H>::to_i32()))
    }
}

/// V / B1
impl<H: NonZero + Default, T:Primitive> Div<B1<H>> for Var<T>
where 
    B1<H>: TypedInt,
    Var<T>: Div<Var<T>,Output = Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn div(self, _rhs: B1<H>) -> Self::Output {
        Var(self.0 / T::from(B1::<H>::to_i32()))
    }
}

// 类型转换实现
// =============================================

/// 实现从 i32 到 Var<i64> 的转换
impl From<i32> for Var<i64> {
    fn from(value: i32) -> Self {
        Var(value as i64)  // 将 i32 转换为 i64 后包装
    }
}

/// 实现从 i32 到 Var<f64> 的转换
impl From<i32> for Var<f64> {
    fn from(value: i32) -> Self {
        Var(value as f64)  // 将 i32 转换为 f64 后包装
    }
}

/// 实现从 i64 到 Var<i64> 的转换
impl From<i64> for Var<i64> {
    fn from(value: i64) -> Self {
        Var(value)   // 直接包装 i64 值
    }
}

/// 实现从 f64 到 Var<f64> 的转换
impl From<f64> for Var<f64> {
    fn from(value: f64) -> Self {
        Var(value) // 直接包装 f64 值
    }
}

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