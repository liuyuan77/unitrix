use core::ops::{Add, Sub, Mul, Div, Rem, Neg};
use crate::number::{Z0, TypedInt, NonZero, Var, Primitive};

// ========== Z0 算术运算实现 / Z0 Arithmetic Implementations ==========

// ==================== Z0 + All ====================
// Z0 + I
impl<I: TypedInt> Add<I> for Z0 {
    type Output = I;
    #[inline(always)]
    fn add(self, rhs: I) -> Self::Output {
        rhs
    }
}

// Z0 + Var<T>
impl<T: Primitive> Add<Var<T>> for Z0 {
    type Output = Var<T>;
    #[inline(always)]
    fn add(self, rhs: Var<T>) -> Self::Output {
        rhs
    }
}

// ==================== Z0 - All ====================
// Z0 - I = -I
impl<I: TypedInt + Neg> Sub<I> for Z0 {
    type Output = I::Output;
    #[inline(always)]
    fn sub(self, i: I) -> Self::Output {
       -i
    }
}

// Z0 - Var<T>
impl<T: Primitive + Neg> Sub<Var<T>> for Z0 {
    type Output = Var<T>;
    #[inline(always)]
    fn sub(self, rhs: Var<T>) -> Self::Output {
        Var(-rhs.0)
    }
}

// ==================== Z0 * All ====================
// Z0 * I = Z0
impl<I: TypedInt> Mul<I> for Z0 {
    type Output = Z0;
    #[inline(always)]
    fn mul(self, _rhs: I) -> Self::Output {
        Z0
    }
}

// Z0 * Var<T> = Z0
impl<T: Primitive> Mul<Var<T>> for Z0 {
    type Output = Z0;
    #[inline(always)]
    fn mul(self, _rhs: Var<T>) -> Self::Output {
        Z0
    }
}

// ==================== Z0 / All ====================
// Division of zero by any non-zero type
// 0 除以任何非零类型

// 0 / 0 is illegal and not implemented
// 0 / 0 非法，未实现

// Z0 / NonZero = Z0
impl<I: NonZero> Div<I> for Z0 {
    type Output = Z0;
    #[inline(always)]
    fn div(self, _rhs: I) -> Self::Output {
        Z0
    }
}

// Z0 / Var<T>
impl<T: Primitive + PartialEq +From< Z0>> Div<Var<T>> for Z0 {
    type Output = Z0;
    fn div(self, rhs: Var<T>) -> Self::Output {
        assert!(rhs.0 != T::from(Z0), "division by zero");
        Z0
    }
}

// ==================== Z0 % All ====================
// Remainder of zero by any non-zero type
// 0 取余任何非零类型

// 0 % 0 is illegal and not implemented
// 0 % 0 非法，未实现

// Z0 % NonZero = Z0
impl<I: NonZero> Rem<I> for Z0 {
    type Output = Z0;
    #[inline(always)]
    fn rem(self, _rhs: I) -> Self::Output {
        Z0
    }
}

// Z0 % Var<T>
impl<T: Primitive + PartialEq + From<Z0>> Rem<Var<T>> for Z0 {
    type Output = Z0;
    fn rem(self, rhs: Var<T>) -> Self::Output {
        assert!(rhs.0 != T::from(Z0), "division by zero");
        Z0
    }
}