use core::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
use crate::number::{Z0, N1, TypedInt, Unsigned, Var, Primitive};


// ==================== Z0 & All ====================
// Z0 & I
impl<I: TypedInt> BitAnd<I> for Z0 {
    type Output = Z0;
    #[inline(always)]
    fn bitand(self, _rhs: I) -> Self::Output {
        Z0
    }
}

// Z0 & Var<T> = Z0
impl<T: Primitive> BitAnd<Var<T>> for Z0 {
    type Output = Z0;
    #[inline(always)]
    fn bitand(self, _rhs: Var<T>) -> Self::Output {
        Z0
    }
}

// ==================== Z0 | All ====================
// Z0 | I
impl<I: TypedInt> BitOr<I> for Z0 {
    type Output = I;
    #[inline(always)]
    fn bitor(self, rhs: I) -> Self::Output {
        rhs
    }
}

// Z0 | Var<T> = Var<T>
impl<T: Primitive> BitOr<Var<T>> for Z0 {
    type Output = Var<T>;
    #[inline(always)]
    fn bitor(self, rhs: Var<T>) -> Self::Output {
        rhs
    }
}

// ==================== Z0 ^ All ====================
// Z0 ^ I = I
impl<I: TypedInt> BitXor<I> for Z0 {
    type Output = I;
    #[inline(always)]
    fn bitxor(self, rhs: I) -> Self::Output {
        rhs
    }
}

// Z0 ^ Var<T> = Var<T>
impl<T: Primitive> BitXor<Var<T>> for Z0 {
    type Output = Var<T>;
    #[inline(always)]
    fn bitxor(self, rhs: Var<T>) -> Self::Output {
        rhs
    }
}

// ==================== Z0 << All ====================
// Z0 << U = Z0
impl<R: Unsigned> Shl<R> for Z0 {
    type Output = Z0;
    fn shl(self, _: R) -> Self::Output {
        Z0  // 0 << n = 0
    }
}

// Z0 << Val<T> = Z0
impl<T: Primitive> Shl<Var<T>> for Z0 {
    type Output = Z0;
    fn shl(self, _: Var<T>) -> Self::Output {
        Z0
    }
}

// ==================== Z0 >> All ====================
// Z0 >> U = Z0
impl<R: Unsigned> Shr<R> for Z0 {
    type Output = Z0;
    fn shr(self, _: R) -> Self::Output {
        Z0
    }
}

// Z0 << Val<T> = Z0
impl<T: Primitive> Shr<Var<T>> for Z0 {
    type Output = Z0;
    fn shr(self, _: Var<T>) -> Self::Output {
        Z0
    }
}

// ==================== !Z0 ====================
// !Z0 = N1
impl Not for Z0 {
    type Output = N1;
    fn not(self) -> Self::Output {
        N1
    }
}