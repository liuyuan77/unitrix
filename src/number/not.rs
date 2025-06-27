/*数字类型按位取反运算实现
 * 编制人: $ource
 * 修改版次:0版完成版
 * 本版次创建时间: 2025年6月25日
 * 最后修改时间: 无
 * 待完善问题：无
 */
use core::ops::Not;
use crate::number::{Z0, P1, N1, B0, B1, NonNegOne, NonZero, Var, PrimitiveInt};

// ==================== 位取反运算实现 ====================

// 基础类型实现
impl Not for Z0 {  // !0 = -1
    type Output = N1;
    fn not(self) -> Self::Output { N1 }
}

impl Not for P1 {  // !1 = -2 (二进制表示为 B0<N1>)
    type Output = B0<N1>;
    fn not(self) -> Self::Output { B0::new() }
}

impl Not for N1 {  // !(-1) = 0
    type Output = Z0;
    fn not(self) -> Self::Output { Z0 }
}

// 递归类型实现
impl<Other: NonZero + NonNegOne + Not> Not for B0<Other> {  // !B0<T> = B1<!T>
    type Output = B1<Other::Output>;
    fn not(self) -> Self::Output { B1::new() }
}

impl<Other: NonZero + Not> Not for B1<Other> {  // !B1<T> = B0<!T>
    type Output = B0<Other::Output>;
    fn not(self) -> Self::Output { B0::new() }
}

// 特殊处理
impl Not for B0<N1> {  // !(-2) = 1 特例
    type Output = P1;
    fn not(self) -> Self::Output { P1 }
}

/* 注意：
1. 小数类型未实现取反，因为小数部分取反会产生无限尾部1
2. 浮点类型不支持位取反操作（无实际意义）
*/

// 变量类型取反
impl<T: PrimitiveInt + Not> Not for Var<T> {  // !Var<T> = Var<!T>
    type Output = Var<<T as Not>::Output>;
    #[inline(always)]
    fn not(self) -> Self::Output { Var(!self.0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_not() {
        assert_eq!(!Z0, N1);
        assert_eq!(!P1, B0::<N1>::new());
        assert_eq!(!N1, Z0);
    }
    
    #[test]
    fn test_recursive_not() {
        let b0n1 = B0::<N1>::new();
        assert_eq!(!b0n1, P1); // 特殊处理
        
        let b1z0 = B1::<P1>::new();
        assert_eq!(!b1z0, B0::<B0<N1>>::new());
    }
    
    #[test]
    fn test_var_not() {
        let var = Var(42i32);
        let res = !var;
        assert_eq!(res.0, !42i32);
    }
}