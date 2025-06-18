/// 类型系统编译时运行，后期可能将PositiveInfinity修改为错误类型
/// The type system runs at compile time, PositiveInfinity may be changed to an error type later

use core::ops::{Add, Sub, Mul, Div, Rem, Neg};
use crate::number::{Negative, NegativeInfinity, NonSpecial, NotANumber, Positive, PositiveInfinity, Primitive, SubWithBorrow, TypedNum, Var, Z0};

// ==============================================
// PositiveInfinity 算术运算实现
// PositiveInfinity Arithmetic Implementations
// ==============================================

// ----------------------------
// 一元负号运算实现
// Unary negation operation implementation
// ----------------------------
impl Neg for PositiveInfinity {
    type Output = NegativeInfinity;
    
    /// 对PositiveInfinity取负，结果为NegativeInfinity
    fn neg(self) -> Self::Output {
        NegativeInfinity
    }
}

// ==============================================
// PositiveInfinity 加法运算实现
// PositiveInfinity Addition Implementations
// ==============================================

// PositiveInfinity + NotANumber
impl Add<NotANumber> for PositiveInfinity {
    type Output = NotANumber;
    
    #[inline(always)]
    fn add(self, _rhs: NotANumber) -> Self::Output {
        NotANumber
    }
}

// PositiveInfinity + PositiveInfinity
impl Add<PositiveInfinity> for PositiveInfinity {
    type Output = PositiveInfinity;
    
    #[inline(always)]
    fn add(self, _rhs: PositiveInfinity) -> Self::Output {
        PositiveInfinity
    }
}

// PositiveInfinity + NegativeInfinity
impl Add<NegativeInfinity> for PositiveInfinity {
    type Output = NotANumber;
    
    #[inline(always)]
    fn add(self, _rhs: NegativeInfinity) -> Self::Output {
        NotANumber
    }
}

// PositiveInfinity + NonSpecial
impl<N: NonSpecial> Add<N> for PositiveInfinity {
    type Output = PositiveInfinity;
    
    #[inline(always)]
    fn add(self, _rhs: N) -> Self::Output {
        PositiveInfinity
    }
}

// PositiveInfinity + Var<T>
impl<T: Primitive> Add<Var<T>> for PositiveInfinity {
    type Output = PositiveInfinity;
    
    #[inline(always)]
    fn add(self, _rhs: Var<T>) -> Self::Output {
        PositiveInfinity
    }
}

// ==============================================
// PositiveInfinity 减法运算实现
// PositiveInfinity Subtraction Implementations
// ==============================================

// PositiveInfinity - NotANumber
impl Sub<NotANumber> for PositiveInfinity {
    type Output = NotANumber;
    
    #[inline(always)]
    fn sub(self, _rhs: NotANumber) -> Self::Output {
        NotANumber
    }
}

// PositiveInfinity - PositiveInfinity
impl Sub<PositiveInfinity> for PositiveInfinity {
    type Output = NotANumber;
    
    #[inline(always)]
    fn sub(self, _rhs: PositiveInfinity) -> Self::Output {
        NotANumber
    }
}

// PositiveInfinity - NegativeInfinity
impl Sub<NegativeInfinity> for PositiveInfinity {
    type Output = PositiveInfinity;
    
    #[inline(always)]
    fn sub(self, _rhs: NegativeInfinity) -> Self::Output {
        PositiveInfinity
    }
}

// PositiveInfinity - NonSpecial
impl<N: NonSpecial> Sub<N> for PositiveInfinity {
    type Output = PositiveInfinity;
    
    #[inline(always)]
    fn sub(self, _rhs: N) -> Self::Output {
        PositiveInfinity
    }
}

// PositiveInfinity - Var<T>
impl<T: Primitive> Sub<Var<T>> for PositiveInfinity {
    type Output = PositiveInfinity;
    
    #[inline(always)]
    fn sub(self, _rhs: Var<T>) -> Self::Output {
        PositiveInfinity
    }
}

// ==============================================
// PositiveInfinity 乘法运算实现
// PositiveInfinity Multiplication Implementations
// ==============================================

impl Mul<T: TypedNum> for PositiveInfinity {
     type Output =  match T::sign() {
            Sign::Positive => PositiveInfinity,
            Sign::Negative => NegativeInfinity,
            Sign::Zero => NotANumber,
            Sign::PInfinity => PositiveInfinity,
            Sign::NInfinity => NegativeInfinity,
            Sign::Nan => NotANumber,
        };
    
    #[inline(always)]
    fn mul(self, rhs: T) -> Self::Output {
        match T::sign() {
            Sign::Positive => PositiveInfinity,
            Sign::Negative => NegativeInfinity,
            Sign::Zero => NotANumber,
            Sign::PInfinity => PositiveInfinity,
            Sign::NInfinity => NegativeInfinity,
            Sign::Nan => NotANumber,
        }
    }
}

// ==============================================
// PositiveInfinity 除法运算实现
// PositiveInfinity Division Implementations
// ==============================================

// PositiveInfinity / TypedNum
impl<F: TypedNum> Div<F> for PositiveInfinity {
    type Output = PositiveInfinity;
    
    /// PositiveInfinity 除以任何类型数字，结果为 PositiveInfinity
    /// Dividing PositiveInfinity by any typed number results in PositiveInfinity
    #[inline(always)]
    fn div(self, _rhs: F) -> Self::Output {
        PositiveInfinity
    }
}

// PositiveInfinity / Var<T>
impl<T: Primitive> Div<Var<T>> for PositiveInfinity {
    type Output = PositiveInfinity;
    
    /// PositiveInfinity 除以变量，结果为 PositiveInfinity
    /// Dividing PositiveInfinity by a variable results in PositiveInfinity
    #[inline(always)]
    fn div(self, _rhs: Var<T>) -> Self::Output {
        PositiveInfinity
    }
}

// ==============================================
// PositiveInfinity 取余运算实现
// PositiveInfinity Remainder Implementations
// ==============================================

// PositiveInfinity % TypedNum
impl<F: TypedNum> Rem<F> for PositiveInfinity {
    type Output = PositiveInfinity;
    
    /// PositiveInfinity 对任何类型数字取余，结果为 PositiveInfinity
    /// Remainder of PositiveInfinity divided by any typed number is PositiveInfinity
    #[inline(always)]
    fn rem(self, _rhs: F) -> Self::Output {
        PositiveInfinity
    }
}

// PositiveInfinity % Var<T>
impl<T: Primitive> Rem<Var<T>> for PositiveInfinity {
    type Output = PositiveInfinity;
    
    /// PositiveInfinity 对变量取余，结果为 PositiveInfinity
    /// Remainder of PositiveInfinity divided by a variable is PositiveInfinity
    #[inline(always)]
    fn rem(self, _rhs: Var<T>) -> Self::Output {
        PositiveInfinity
    }
}