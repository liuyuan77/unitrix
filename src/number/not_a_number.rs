/// 类型系统编译时运行，后期可能将NotANumber修改为错误类型
/// The type system runs at compile time, NotANumber may be changed to an error type later

use core::ops::{Add, Sub, Mul, Div, Rem, Neg};
use crate::number::{NotANumber, TypedNum, Var, Primitive};

// ==============================================
// NotANumber 算术运算实现
// NotANumber Arithmetic Implementations
// ==============================================

// ----------------------------
// 一元负号运算实现
// Unary negation operation implementation
// ----------------------------
impl Neg for NotANumber {
    type Output = NotANumber;
    
    /// 对NotANumber取负，结果仍为NotANumber
    /// Negating NotANumber still yields NotANumber
    fn neg(self) -> Self::Output {
        NotANumber
    }
}

// ==============================================
// NotANumber 加法运算实现
// NotANumber Addition Implementations
// ==============================================

// NotANumber + TypedNum
impl<F: TypedNum> Add<F> for NotANumber {
    type Output = NotANumber;
    
    /// NotANumber 与任何类型数字相加，结果为 NotANumber
    /// Adding NotANumber with any typed number results in NotANumber
    #[inline(always)]
    fn add(self, _rhs: F) -> Self::Output {
        NotANumber
    }
}

// NotANumber + Var<T>
impl<T: Primitive> Add<Var<T>> for NotANumber {
    type Output = NotANumber;
    
    /// NotANumber 与变量相加，结果为 NotANumber
    /// Adding NotANumber with a variable results in NotANumber
    #[inline(always)]
    fn add(self, _rhs: Var<T>) -> Self::Output {
        NotANumber
    }
}

// ==============================================
// NotANumber 减法运算实现
// NotANumber Subtraction Implementations
// ==============================================

// NotANumber - TypedNum
impl<F: TypedNum> Sub<F> for NotANumber {
    type Output = NotANumber;
    
    /// NotANumber 减去任何类型数字，结果为 NotANumber
    /// Subtracting any typed number from NotANumber results in NotANumber
    #[inline(always)]
    fn sub(self, _rhs: F) -> Self::Output {
        NotANumber
    }
}

// NotANumber - Var<T>
impl<T: Primitive> Sub<Var<T>> for NotANumber {
    type Output = NotANumber;
    
    /// NotANumber 减去变量，结果为 NotANumber
    /// Subtracting a variable from NotANumber results in NotANumber
    #[inline(always)]
    fn sub(self, _rhs: Var<T>) -> Self::Output {
        NotANumber
    }
}

// ==============================================
// NotANumber 乘法运算实现
// NotANumber Multiplication Implementations
// ==============================================

// NotANumber * TypedNum
impl<F: TypedNum> Mul<F> for NotANumber {
    type Output = NotANumber;
    
    /// NotANumber 乘以任何类型数字，结果为 NotANumber
    /// Multiplying NotANumber by any typed number results in NotANumber
    #[inline(always)]
    fn mul(self, _rhs: F) -> Self::Output {
        NotANumber
    }
}

// NotANumber * Var<T>
impl<T: Primitive> Mul<Var<T>> for NotANumber {
    type Output = NotANumber;
    
    /// NotANumber 乘以变量，结果为 NotANumber
    /// Multiplying NotANumber by a variable results in NotANumber
    #[inline(always)]
    fn mul(self, _rhs: Var<T>) -> Self::Output {
        NotANumber
    }
}

// ==============================================
// NotANumber 除法运算实现
// NotANumber Division Implementations
// ==============================================

// NotANumber / TypedNum
impl<F: TypedNum> Div<F> for NotANumber {
    type Output = NotANumber;
    
    /// NotANumber 除以任何类型数字，结果为 NotANumber
    /// Dividing NotANumber by any typed number results in NotANumber
    #[inline(always)]
    fn div(self, _rhs: F) -> Self::Output {
        NotANumber
    }
}

// NotANumber / Var<T>
impl<T: Primitive> Div<Var<T>> for NotANumber {
    type Output = NotANumber;
    
    /// NotANumber 除以变量，结果为 NotANumber
    /// Dividing NotANumber by a variable results in NotANumber
    #[inline(always)]
    fn div(self, _rhs: Var<T>) -> Self::Output {
        NotANumber
    }
}

// ==============================================
// NotANumber 取余运算实现
// NotANumber Remainder Implementations
// ==============================================

// NotANumber % TypedNum
impl<F: TypedNum> Rem<F> for NotANumber {
    type Output = NotANumber;
    
    /// NotANumber 对任何类型数字取余，结果为 NotANumber
    /// Remainder of NotANumber divided by any typed number is NotANumber
    #[inline(always)]
    fn rem(self, _rhs: F) -> Self::Output {
        NotANumber
    }
}

// NotANumber % Var<T>
impl<T: Primitive> Rem<Var<T>> for NotANumber {
    type Output = NotANumber;
    
    /// NotANumber 对变量取余，结果为 NotANumber
    /// Remainder of NotANumber divided by a variable is NotANumber
    #[inline(always)]
    fn rem(self, _rhs: Var<T>) -> Self::Output {
        NotANumber
    }
}