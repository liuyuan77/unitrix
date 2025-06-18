use core::ops::{Neg, Add, Sub, Mul, Div, Rem};
use core::convert::From;

use crate::number::{Special, Z0, N1, P1, B0, B1, Float, Var, TypedInt, NonZero, PrimitiveInt};

// ==============================================
// Special转基本类型
// ==============================================
// 实现 Special -> f32 的转换
impl From<Special> for f32 {
    fn from(value: Special) -> f32 {
        match value {
            Special::Nan => f32::NAN,
            Special::Infinity => f32::INFINITY,
            Special::NegInfinity => f32::NEG_INFINITY,
        }
    }
}

// 实现 Special -> f64 的转换
impl From<Special> for f64 {
    fn from(value: Special) -> f64 {
        match value {
            Special::Nan => f64::NAN,
            Special::Infinity => f64::INFINITY,
            Special::NegInfinity => f64::NEG_INFINITY,
        }
    }
}


// ==============================================
// Special 算术运算实现
// Special Arithmetic Implementations
// ==============================================

// ----------------------------
// 一元负号运算实现
// Unary negation operation implementation
// ----------------------------

impl Neg for Special {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Special::Nan => Special::Nan,          // -NaN = NaN
            Special::Infinity => Special::NegInfinity,    // -∞ = -∞
            Special::NegInfinity => Special::Infinity,    // -(-∞) = ∞
        }
    }
}

// ==============================================
// Special 加法运算实现
// Special Addition Implementations
// ==============================================
// Special + Special
impl Add for Special {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // +∞ + +∞ = +∞
            (Special::Infinity, Special::Infinity) => Special::Infinity,
            // -∞ + -∞ = -∞
            (Special::NegInfinity, Special::NegInfinity) => Special::NegInfinity,
            // +∞ + -∞ = +∞
            (Special::Infinity, Special::NegInfinity) => Special::Nan,
            // -∞ + +∞ = -∞
            (Special::NegInfinity,Special::Infinity) => Special::Nan,
            // 其他情况返回 NaN
            // Other cases return NaN
            _ => Special::Nan,
        }
    }
}

// Special + 0
impl Add<Z0> for Special {
    type Output = Self;
    fn add(self, _rhs: Z0) -> Self::Output {
        self
    }
}

// Special + 1
impl Add<P1> for Special {
    type Output = Self;
    fn add(self, _rhs: P1) -> Self::Output {
        self
    }
}

// Special + -1
impl Add<N1> for Special {
    type Output = Self;
    fn add(self, _rhs: N1) -> Self::Output {
        self
    }
}

// Special + B0<H>
impl<H: NonZero> Add<B0<H>> for Special {
    type Output = Self;
    fn add(self, _rhs: B0<H>) -> Self::Output {
        self
    }
}

// Special + B1<H>
impl<H: NonZero> Add<B1<H>> for Special {
    type Output = Self;
    fn add(self, _rhs: B1<H>) -> Self::Output {
        self
    }
}

// Special + Float
impl<Mantissa: NonZero, Exponent: TypedInt> Add<Float<Mantissa, Exponent>> for Special {
    type Output = Self;
    fn add(self, _rhs: Float<Mantissa, Exponent>) -> Self::Output {
        self
    }
}

// Special + Var<T>
impl<T: PrimitiveInt> Add<Var<T>> for Special {
    type Output = Self;
    fn add(self, _rhs: Var<T>) -> Self::Output {
        self
    }
}

impl Add<Var<f32>> for Special {
    type Output = Var<f32>;
    fn add(self, rhs: Var<f32>) -> Self::Output {
        Var(f32::from(self) + rhs.0)
    }
}

impl Add<Var<f64>> for Special {
    type Output = Var<f64>;
    fn add(self, rhs: Var<f64>) -> Self::Output {
        Var(f64::from(self) + rhs.0)
    }
}
//........


// 实现减法运算（x - y）
// Implement subtraction operation (x - y)
impl Sub for Special {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // +∞ - +∞ = NaN
            (Special::Infinity, Special::Infinity) => Special::Nan,
            // -∞ - (-∞) = NaN
            (Special::NegInfinity, Special::NegInfinity) => Special::Nan,
            // +∞ - -∞ = +∞
            (Special::Infinity, Special::NegInfinity) => Special::Infinity,
            // -∞ - +∞ = -∞
            (Special::NegInfinity,Special::Infinity) => Special::NegInfinity,
            // 其他情况返回 NaN
            // Other cases return NaN
            _ => Special::Nan,
        }
    }
}

// 实现乘法运算（x * y）
// Implement multiplication operation (x * y)
impl Mul for Special {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // +∞ * +∞ = +∞
            (Special::Infinity, Special::Infinity) => Special::Infinity,
            // -∞ * -∞ = +∞
            (Special::NegInfinity, Special::NegInfinity) => Special::Infinity,
            // +∞ * -∞ = -∞
            (Special::Infinity, Special::NegInfinity) => Special::NegInfinity,
            // -∞ * +∞ = -∞
            (Special::NegInfinity, Special::Infinity) => Special::NegInfinity,
            // 其他情况返回 NaN
            // Other cases return NaN
            _ => Special::Nan,
        }
    }
}

// 实现除法运算（x / y）
// Implement division operation (x / y)
impl Div for Special {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // +∞ / +∞ = NaN
            (Special::Infinity, Special::Infinity) => Special::Nan,
            // -∞ / -∞ = NaN
            (Special::NegInfinity, Special::NegInfinity) => Special::Nan,
            // +∞ / -∞ = NaN
            (Special::Infinity, Special::NegInfinity) => Special::Nan,
            // -∞ / ∞ = NaN
            (Special::NegInfinity, Special::Infinity) => Special::Nan,
            // x / ∞ = 0 (这里用 NaN 表示，实际可能需要扩展枚举支持普通数字)
            // x / ∞ = 0 (represented by NaN here, may need to extend enum for normal numbers)
            _ => Special::Nan,
        }
    }
}

impl Rem for Special {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            _ => Special::Nan,
        }
    }
}

// 测试代码
// Test code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(Special::default(), Special::Nan);
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Special::Infinity, Special::NegInfinity);
        assert_eq!(-Special::NegInfinity, Special::Infinity);
        assert_eq!(-Special::Nan, Special::Nan);
    }

    #[test]
    fn test_add() {
        assert_eq!(Special::Infinity + Special::NegInfinity, Special::Nan);
        assert_eq!(Special::Infinity + Special::Nan, Special::Infinity);
    }

    #[test]
    fn test_sub() {
        assert_eq!(Special::Infinity - Special::Infinity, Special::Nan);
        assert_eq!(Special::NegInfinity - Special::Nan, Special::NegInfinity);
    }

    #[test]
    fn test_mul() {
        assert_eq!(Special::Infinity * Special::Infinity, Special::Infinity);
        assert_eq!(Special::NegInfinity * Special::Infinity, Special::NegInfinity);
    }

    #[test]
    fn test_div() {
        assert_eq!(Special::Infinity / Special::Infinity, Special::Nan);
        assert_eq!(Special::NegInfinity / Special::Infinity, Special::Nan);
    }
}