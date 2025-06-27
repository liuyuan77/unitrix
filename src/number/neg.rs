/*库数字取负
 * 编制人: $ource
 * 修改版次:0版完成版
 * 本版次创建时间: 2025年6月25日
 * 最后修改时间: 无
 * 待完善问题：无
 */
use core::ops::{Not, Neg};
use crate::number::{FixedPoint, Float, NonOne, NonZero, Primitive, Unsigned, Var, B0, B1, N1, P1, Z0};

// ==================== 算术取负（-运算符）实现 ====================

/// 零的取负实现
/// -0 = 0
impl Neg for Z0 {
    type Output = Z0;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Z0
    }
}

/// 正1的取负实现
/// -1 = -1
impl Neg for P1 {
    type Output = N1;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        N1
    }
}

/// 负1的取负实现
/// -(-1) = 1
impl Neg for N1 {
    type Output = P1;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        P1
    }
}

/// 二进制数B0的取负实现（最低位为0的情况）
/// 
/// # 类型参数
/// - H: 更高位部分，需要满足NonZero且可Neg
/// 
/// # 示例
/// - B0<P1> => B0<N1>
/// - B0<N1> => B0<P1>
/// 不影响当前位(B0保持不变)
impl<H: NonZero + Neg> Neg for B0<H> {
    type Output = B0<H::Output>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        B0::new()  // 递归地对高位部分取负，保持B0不变
    }
}

/// 二进制数B1的取负实现（最低位为1的情况）
/// 
/// # 类型参数
/// - H: 更高位部分，需要满足NonZero且可Not
/// 
/// # 示例
/// - B1<P1> => B1<B0<N1>>
/// - B1<N1> => B1<P1>
/// 不影响当前位(B1保持不变)
impl<H: NonZero + Not> Neg for B1<H> {
    type Output = B1<H::Output>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        B1::new()  // 递归地对高位部分取反，保持B1不变
    }
}

// ==================== 定点数取负实现 ====================

/// 定点数取负实现（小数部分为0时）
/// 行为与整数相同
impl<IntPart: Neg> Neg for FixedPoint<IntPart, Z0> {
    type Output = FixedPoint<IntPart::Output, Z0>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        FixedPoint::new()
    }
}

/// 定点数取负实现（小数部分为P1时）
/// 小数部分保持不变
impl<IntPart: Not> Neg for FixedPoint<IntPart, P1> {
    type Output = FixedPoint<IntPart::Output, P1>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        FixedPoint::new()
    }
}

/// 定点数取负实现（小数部分为B0时）
/// 小数部分取反：B0<L> => B1<L::Output>
impl<IntPart: Not, L: Unsigned + NonZero + NonOne + Not> Neg for FixedPoint<IntPart, B0<L>> {
    type Output = FixedPoint<IntPart::Output, B1<L::Output>>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        FixedPoint::new()
    }
}

/// 定点数取负实现（小数部分为B1时）
/// 小数部分取反：B1<L> => B0<L::Output>
impl<IntPart: Not, L: Unsigned + NonZero + NonOne + Not> Neg for FixedPoint<IntPart, B1<L>> {
    type Output = FixedPoint<IntPart::Output, B0<L::Output>>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        FixedPoint::new()
    }
}

/// 小数部分取反特化：B0<P1> => B1<P1>
impl<IntPart: Not> Neg for FixedPoint<IntPart, B0<P1>> {
    type Output = FixedPoint<IntPart::Output, B1<P1>>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        FixedPoint::new()
    }
}

/// 小数部分取反特化：B1<P1> => B0<P1>
impl<IntPart: Not> Neg for FixedPoint<IntPart, B1<P1>> {
    type Output = FixedPoint<IntPart::Output, B0<P1>>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        FixedPoint::new()
    }
}

// ==================== 浮点数取负实现 ====================

/// 浮点数取负实现
/// 对尾数部分取负，指数部分保持不变
impl<Mantissa: Neg, Exponent> Neg for Float<Mantissa, Exponent> {
    type Output = Float<Mantissa::Output, Exponent>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Float::new()  // 构造新的浮点数，尾数部分已取负
    }
}

// ==================== 变量取负实现 ====================

/// 变量取负实现
/// 对变量内部的值取负，返回新的变量
impl<T: Primitive + Neg> Neg for Var<T> {
    type Output = Var<<T as Neg>::Output>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Var(-self.0)  // 对内部值取负并包装为新变量
    }
}

// ==================== 测试代码 ====================

#[cfg(test)]
mod tests {
    use crate::number::{B0, B1, P1, N1, Z0, FixedPoint, Float, Var};

    #[test]
    fn test_neg_z0() {
        let zero = Z0;
        let neg_zero = -zero;
        assert_eq!(neg_zero, Z0);
    }

    #[test]
    fn test_neg_p1() {
        let one = P1;
        let neg_one = -one;
        assert_eq!(neg_one, N1);
    }

    #[test]
    fn test_neg_n1() {
        let neg_one = N1;
        let one = -neg_one;
        assert_eq!(one, P1);
    }

    #[test]
    fn test_neg_b0() {
        type B0P1 = B0<P1>;
        type B0N1 = B0<N1>;
        
        let b0p1 = B0P1::new();
        let b0n1 = -b0p1;
        assert_eq!(b0n1, B0N1::new());
        
        let b0n1_orig = B0N1::new();
        let b0p1_again = -b0n1_orig;
        assert_eq!(b0p1_again, B0P1::new());
    }

    #[test]
    fn test_neg_b1() {
        type B1P1 = B1<P1>;
        type B1B0N1 = B1<B0<N1>>;
        
        let b1p1 = B1P1::new();
        let b1b0n1 = -b1p1;
        assert_eq!(b1b0n1, B1B0N1::new());
        
        let b1b0n1_orig = B1B0N1::new();
        let b1p1_again = -b1b0n1_orig;
        assert_eq!(b1p1_again, B1P1::new());
    }

    #[test]
    fn test_neg_fixed_point_z0() {
        type FP = FixedPoint<P1, Z0>;
        let fp = FP::new();
        let neg_fp = -fp;
        assert_eq!(neg_fp, FixedPoint::<N1, Z0>::new());
    }

    #[test]
    fn test_neg_fixed_point_p1() {
        type FP = FixedPoint<P1, P1>;
        let fp = FP::new();
        let neg_fp = -fp;
        assert_eq!(neg_fp, FixedPoint::<B0<N1>, P1>::new());
    }

    #[test]
    fn test_neg_fixed_point_b0() {
        type FP = FixedPoint<P1, B0<P1>>; 
        type NegFP = FixedPoint<B0<N1>, B1<P1>>;
        let fp = FP::new();
        let neg_fp = -fp;
        assert_eq!(neg_fp, NegFP::new());
    }

    #[test]
    fn test_neg_fixed_point_b1() {
        type FP = FixedPoint<P1, B1<P1>>;
        type NegFP = FixedPoint<B0<N1>, B0<P1>>;
        let fp = FP::new();
        let neg_fp = -fp;
        assert_eq!(neg_fp, NegFP::new());
    }

    #[test]
    fn test_neg_float() {
        type F = Float<B0<P1>, Z0>;
        let f = F::new();
        let neg_f = -f;
        assert_eq!(neg_f, Float::<B0<N1>, Z0>::new());
    }

    #[test]
    fn test_neg_var() {
        let var = Var(100_i32);
        let neg_var = -var;
        assert_eq!(neg_var, Var(-100));
    }
}