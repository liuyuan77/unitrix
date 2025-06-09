use super::basic::{Z0, P1, N1, B0, B1, NonZero, NonOne, Unsigned};
use super::sub1::Sub1;
use core::ops::Shr;

// ==================== Right Shift Operation (>>) ====================
// ==================== 右移运算（>>） ====================

// Z0 >> U
// Zero right shifted by any unsigned number is still zero
// 零右移任何无符号数仍然是零
impl<R: Unsigned> Shr<R> for Z0 {
    type Output = Z0;
    fn shr(self, _: R) -> Self::Output {
        Z0  // 0 >> n = 0
    }
}

// P1 >> U
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
        Z0::new()
    }
}

// N1 >> U
// Negative one right shifted by any unsigned number remains negative one
// (due to sign extension in two's complement)
// 负一右移任何无符号数仍然是负一（由于二进制补码中的符号扩展）
impl<R: Unsigned> Shr<R> for N1 {
    type Output = Self;
    fn shr(self, _: R) -> Self::Output {
        self
    }
}

// B0 >> U
// Binary number ending with 0 right shifted by zero is itself
// 以0结尾的二进制数右移零位是其本身
impl<H: NonZero> Shr<Z0> for B0<H> {
    type Output = Self;
    fn shr(self, _: Z0) -> Self::Output {
        self
    }
}

// Binary number ending with 0 right shifted by one becomes its head
// 以0结尾的二进制数右移一位变为其头部
impl<H: NonZero> Shr<P1> for B0<H> {
    type Output = H;
    fn shr(self, _: P1) -> Self::Output {
        H::default()
    }
}

// Binary number ending with 0 right shifted by more than one
// Recursively shifts right by one until the shift amount is zero
// 以0结尾的二进制数右移多于一位
// 递归地右移一位直到移位量为零
impl<H: NonZero, R: Unsigned + NonZero + NonOne + Sub1> Shr<R> for B0<H>
where
    H: Shr<<R as Sub1>::Output>
{
    type Output = <H as Shr<R::Output>>::Output;
    fn shr(self, r: R) -> Self::Output {
        (self>>P1)>>r.sub1()
    }
}

// B1 >> U
// Binary number ending with 1 right shifted by zero is itself
// 以1结尾的二进制数右移零位是其本身
impl<H: NonZero> Shr<Z0> for B1<H> {
    type Output = Self;
    fn shr(self, _: Z0) -> Self::Output {
        self
    }
}

// Binary number ending with 1 right shifted by one becomes its head
// 以1结尾的二进制数右移一位变为其头部
impl<H: NonZero> Shr<P1> for B1<H> {
    type Output = H;
    fn shr(self, _: P1) -> Self::Output {
        H::default()
    }
}

// Binary number ending with 1 right shifted by more than one
// Recursively shifts right by one until the shift amount is zero,
// and maintains the sign bit (B0 prefix)
// 以1结尾的二进制数右移多于一位
// 递归地右移一位直到移位量为零，并保持符号位（B0前缀）
impl<H: NonZero, R: Unsigned + NonZero + NonOne + Sub1> Shr<R> for B1<H>
where
    H: Shr<<R as Sub1>::Output>
{
    type Output = <H as Shr<R::Output>>::Output;
    fn shr(self, r: R) -> Self::Output {
        (self>>P1)>>(r.sub1())
    }
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shr() {
        // Test Z0
        let _: Z0 = Z0 >> Z0;
        let _: Z0 = Z0 >> P1;
        
        // Test P1
        let _: P1 = P1 >> Z0;
        let _: Z0 = P1 >> P1;
        
        // Test N1
        let _: N1 = N1 >> Z0;
        let _: N1 = N1 >> P1;
        
        // Test B0
        let b0: B0<P1> = B0::new();
        let _: B0<P1> = b0 >> Z0;
        let _: P1 = b0 >> P1;
        
        // Test B1
        let b1: B1<P1> = B1::new();
        let _: B1<P1> = b1 >> Z0;
        let _: P1 = b1 >> P1;
    }
}