use core::ops::{Add, Mul, Neg};
use super::basic::{Z0, P1, N1, B0, B1, Integer, NonZero};
use crate::variable::{Var,Numeric};

// ========== Basic Type Multiplication ==========
// ========== 基本类型乘法 ==========

// ========== 0 * All ==========
// ========== 零乘以任何数 ==========
impl<I: Integer> Mul<I> for Z0 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, _rhs: I) -> Self::Output {
        self  // 0 * any = 0
    }
}

// ========== 1 * All ==========
// ========== 一乘以任何数 ==========
impl<I: Integer> Mul<I> for P1 {
    type Output = I;
    #[inline(always)]
    fn mul(self, rhs: I) -> Self::Output {
        rhs  // 1 * x = x
    }
}

// ========== -1 * All ==========
// ========== 负一乘以任何数 ==========
impl<I: Integer + Neg> Mul<I> for N1 {
    type Output = I::Output;
    #[inline(always)]
    fn mul(self, rhs: I) -> Self::Output {
        -rhs  // -1 * x = -x
    }
}

// ========== B0 * All ==========
// ========== 以0结尾的二进制数乘法 ==========

// B0 * Z0 = 0
// 以0结尾的数乘以零
impl<H: NonZero> Mul<Z0> for B0<H> {
    type Output = Z0;
    #[inline(always)]
    fn mul(self, _rhs: Z0) -> Self::Output {
        Z0  // x * 0 = 0
    }
}

// B0<NonZero> * NonZero = B0<NonZero * I>
// 以0结尾的数乘以非零数
//
// Explanation:
//    B0<H> * I = (2*H)*I = 2*(H*I) = B0<H * I>
//    Therefore, B0<NonZero> * I = B0<NonZero * I>
//
// 说明：
//    B0<H> * I = (2*H)*I = 2*(H*I) = B0<H * I>
//    因此，B0<NonZero> * I = B0<NonZero * I>
impl<H: NonZero + Mul<I>, I: NonZero> Mul<I> for B0<H> {
    type Output = B0<H::Output>;
    #[inline(always)]
    fn mul(self, _rhs: I) -> Self::Output {
        B0::new()  // 构造新的B0类型
    }
}

// ========== B1 * All ==========
// ========== 以1结尾的二进制数乘法 ==========

// B1 * Z0 = 0
// 以1结尾的数乘以零
impl<H: NonZero> Mul<Z0> for B1<H> {
    type Output = Z0;
    #[inline(always)]
    fn mul(self, _rhs: Z0) -> Self::Output {
        Z0  // x * 0 = 0
    }
}

// B1<NonZero> * NonZero = I + B0<NonZero * I>
// 以1结尾的数乘以非零数
//
// Explanation:
//    B1<H> * I = (1 + 2 * H) * I = I + 2 * (H * I)=I + B0<H * I>
//    Therefore, B1<NonZero> * I = I + B0<NonZero * I>
//
// 说明：
//    B1<H> * I = (1 + 2 * H) * I = I + 2 * (H * I)=I + B0<H * I>
//    因此，B1<NonZero> * I = I + B0<NonZero * I>
impl<H: NonZero + Mul<I>, I: NonZero + Add<B0<<H as Mul<I>>::Output>>> Mul<I> for B1<H> {
    type Output = I::Output;
    #[inline(always)]
    fn mul(self, i: I) -> Self::Output {
        i + B0::new()  // I + B0<H*I>
    }
}

/// Type alias for multiplication: `Prod<A, B> = <A as Mul<B>>::Output`
/// 乘法运算的类型别名：`Prod<A, B> = <A as Mul<B>>::Output`
pub type Prod<A, B> = <A as Mul<B>>::Output;

// ========== 与Var<T>乘法(新增加) ==========
// ========== 0 * Var<T> ==========
impl<T:Numeric> Mul<Var<T>> for Z0 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, _rhs: Var<T>) -> Self::Output {
        self  // 0 * any = 0
    }
}

// ========== 1 * Var<T> ==========
impl<T: Numeric> Mul<Var<T>> for P1 {
    type Output = Var<T>;
    #[inline(always)]
    fn mul(self, rhs: Var<T>) -> Self::Output {
        rhs  // 1 * x = x
    }
}

// ========== -1 * Var<T> ==========
impl<T: Numeric> Mul<Var<T>> for N1
where
    Var<T>: Neg,
{
    type Output = <Var<T> as Neg>::Output;
    #[inline(always)]
    fn mul(self, rhs: Var<T>) -> Self::Output {
        -rhs  // -1 * x = -x
    }
}

// ========== B0 * Var<T> ==========
impl<H: NonZero, T:Numeric> Mul<Var<T>> for B0<H>
where 
    B0<H>: Integer,
    Var<T>: Mul<Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn mul(self, rhs: Var<T>) -> Self::Output {
        Var(rhs.0 * T::from(B0::<H>::to_i32()))
    }
}

// ========== B1 * Var<T> ==========

impl<H: NonZero, T:Numeric> Mul<Var<T>> for B1<H>
where 
    B1<H>: Integer,
    Var<T>: Mul<Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn mul(self, rhs: Var<T>) -> Self::Output {
        Var(rhs.0 * T::from(B0::<H>::to_i32()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_multiplication() {
        // Test Z0 (0 * anything = 0)
        // 测试零的乘法
        let _: Z0 = Z0 * Z0;
        let _: Z0 = Z0 * P1;
        let _: Z0 = Z0 * N1;
        
        // Test P1 (1 * anything = anything)
        // 测试正一的乘法
        let _: Z0 = P1 * Z0;
        let _: P1 = P1 * P1;
        let _: N1 = P1 * N1;
        
        // Test N1 (-1 * anything = -anything)
        // 测试负一的乘法
        let _: Z0 = N1 * Z0;
        let _: N1 = N1 * P1;
        let _: P1 = N1 * N1;
    }
    
    #[test]
    fn test_b0_multiplication() {
        // B0<P1> represents binary 10 (decimal 2)
        // B0<P1> 表示二进制10（十进制2）
        let b0_p1: B0<P1> = B0::new();
        
        // 2 * 0 = 0
        let _: Z0 = b0_p1 * Z0;
        
        // 2 * 1 = 2 (B0<P1>)
        let _: B0<P1> = b0_p1 * P1;
        
        // 2 * (-1) = -2 (B0<N1>)
        let _: B0<N1> = b0_p1 * N1;
    }
    
    #[test]
    fn test_b1_multiplication() {
        // B1<P1> represents binary 11 (decimal 3)
        // B1<P1> 表示二进制11（十进制3）
        let b1_p1: B1<P1> = B1::new();
        
        // 3 * 0 = 0
        let _: Z0 = b1_p1 * Z0;
        
        // 3 * 1 = 3 (B1<P1>)
        // Note: This requires addition to be properly implemented
        // 注意：这需要加法正确实现
        // let _: B1<P1> = b1_p1 * P1;
        
        // 3 * (-1) = -3 (B1<N1>)
        // let _: B1<N1> = b1_p1 * N1;
    }
    
    // Helper function to create values
    // 辅助函数创建值
    fn _create_values() {
        let _z0 = Z0;
        let _p1 = P1;
        let _n1 = N1;
        let _b0_p1: B0<P1> = B0::new();
        let _b1_p1: B1<P1> = B1::new();
    }
}