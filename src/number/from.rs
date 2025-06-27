use core::convert::From;
use crate::number::{Z0, P1, N1, B0, B1, NonZero, Var, Primitive};


// 基本类型到变量类型的转换实现
// =============================================
impl<T: Primitive> From<T> for Var<T> { fn from(value: T) -> Var<T> { Var(value) } }

impl From<i8> for Var<i16> { fn from(value: i8) -> Var<i16> { Var(value as i16) } }
impl From<i8> for Var<i32> { fn from(value: i8) -> Var<i32> { Var(value as i32) } }
impl From<i8> for Var<i64> { fn from(value: i8) -> Var<i64> { Var(value as i64) } }
impl From<i8> for Var<i128> { fn from(value: i8) -> Var<i128> { Var(value as i128) } }

impl From<i16> for Var<i32> { fn from(value: i16) -> Var<i32> { Var(value as i32) } }
impl From<i16> for Var<i64> { fn from(value: i16) -> Var<i64> { Var(value as i64) } }
impl From<i16> for Var<i128> { fn from(value: i16) -> Var<i128> { Var(value as i128) } }

impl From<i32> for Var<i64> { fn from(value: i32) -> Var<i64> { Var(value as i64) } }
impl From<i32> for Var<i128> { fn from(value: i32) -> Var<i128> { Var(value as i128) } }

impl From<i64> for Var<i128> { fn from(value: i64) -> Var<i128> { Var(value as i128) } }

impl From<f32> for Var<f64> { fn from(value: f32) -> Var<f64> { Var(value as f64) } }


// 数字类型到变量类型的转换实现
// =============================================

// 实现Z0(零)到各种整数类型的转换
impl From<Z0> for Var<i8> { fn from(_: Z0) -> Var<i8> { Var(0) } }
impl From<Z0> for Var<i16> { fn from(_: Z0) -> Var<i16> { Var(0) } }
impl From<Z0> for Var<i32> { fn from(_: Z0) -> Var<i32> { Var(0) } }
impl From<Z0> for Var<i64> { fn from(_: Z0) -> Var<i64> { Var(0) } }
impl From<Z0> for Var<i128> { fn from(_: Z0) -> Var<i128> { Var(0) } }

// 实现P1(正一)到各种整数类型的转换
impl From<P1> for Var<i8> { fn from(_: P1) -> Var<i8> { Var(1) } }
impl From<P1> for Var<i16> { fn from(_: P1) -> Var<i16> { Var(1) } }
impl From<P1> for Var<i32> { fn from(_: P1) -> Var<i32> { Var(1) } }
impl From<P1> for Var<i64> { fn from(_: P1) -> Var<i64> { Var(1) } }
impl From<P1> for Var<i128> { fn from(_: P1) -> Var<i128> { Var(1) } }

// 实现N1(负一)到各种整数类型的转换
impl From<N1> for Var<i8> { fn from(_: N1) -> Var<i8> { Var(-1) } }
impl From<N1> for Var<i16> { fn from(_: N1) -> Var<i16> { Var(-1) } }
impl From<N1> for Var<i32> { fn from(_: N1) -> Var<i32> { Var(-1) } }
impl From<N1> for Var<i64> { fn from(_: N1) -> Var<i64> { Var(-1) } }
impl From<N1> for Var<i128> { fn from(_: N1) -> Var<i128> { Var(-1) } }

// 实现Z0(零)到浮点类型的转换
impl From<Z0> for Var<f32> { fn from(_: Z0) -> Var<f32> { Var(0.0) } }
impl From<Z0> for Var<f64> { fn from(_: Z0) -> Var<f64> { Var(0.0) } }

// 实现P1(正一)到浮点类型的转换
impl From<P1> for Var<f32> { fn from(_: P1) -> Var<f32> { Var(1.0) } }
impl From<P1> for Var<f64> { fn from(_: P1) -> Var<f64> { Var(1.0) } }

// 实现N1(负一)到浮点类型的转换
impl From<N1> for Var<f32> { fn from(_: N1) -> Var<f32> { Var(-1.0) } }
impl From<N1> for Var<f64> { fn from(_: N1) -> Var<f64> { Var(-1.0) } }

// 二进制数字类型的转换实现
// =============================================

/// 实现B0(最低位0)到Var<T>的转换
/// 转换逻辑：将高位部分值乘以2
impl<T: Primitive, H: NonZero> From<B0<H>> for Var<T>
where 
    Var<T>: From<H> + From<i8>,
{
    #[inline(always)]
    fn from(_: B0<H>) -> Var<T> {
        let val = Var::<T>::from(H::default());
        val * Var::<T>::from(2_i8)
    }
}

/// 实现B1(最低位1)到Var<T>的转换
/// 转换逻辑：将高位部分值乘以2再加1
impl<T: Primitive, H: NonZero> From<B1<H>> for Var<T>
where 
    Var<T>: From<H> + From<i8>,
{
    #[inline(always)]
    fn from(_: B1<H>) -> Var<T> {
        let val = Var::<T>::from(H::default());
        val * Var::<T>::from(2_i8) + Var::<T>::from(1_i8)
    }
}