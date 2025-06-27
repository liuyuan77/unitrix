/* 数字类型按位与运算实现
 * 作者：$ource
 * 版本：0.0
 * 创建时间：2025-06-26
 */
use core::ops::BitAnd;
use crate::number::{Z0, P1, N1, B0, B1, NonZero, TypedInt, IfB0, IfB1, Var, Primitive, FixedPoint};

// =============== 位与运算实现 ===============

// ----- 零值(Z0)的位与运算 -----
// Z0与任何类型相与结果都是Z0
impl<I: TypedInt> BitAnd<I> for Z0 {
    type Output = Z0;
    #[inline(always)]
    fn bitand(self, _: I) -> Z0 { Z0 }
}

// Z0与变量类型相与
impl<T: Primitive> BitAnd<Var<T>> for Z0 {
    type Output = Z0;
    #[inline(always)]
    fn bitand(self, _: Var<T>) -> Z0 { Z0 }
}

// ----- 正一(P1)的位与运算 -----
impl BitAnd<Z0> for P1 { type Output = Z0; fn bitand(self, _: Z0) -> Z0 { Z0 } }
impl BitAnd<P1> for P1 { type Output = P1; fn bitand(self, _: P1) -> P1 { P1 } }
impl BitAnd<N1> for P1 { type Output = P1; fn bitand(self, _: N1) -> P1 { P1 } }

// P1与二进制0位相与
impl<H: NonZero> BitAnd<B0<H>> for P1 {
    type Output = Z0;
    fn bitand(self, _: B0<H>) -> Z0 { Z0 }
}

// P1与二进制1位相与
impl<H: NonZero> BitAnd<B1<H>> for P1 {
    type Output = P1;
    fn bitand(self, _: B1<H>) -> P1 { P1 }
}

// P1与变量类型相与
impl<T: Primitive> BitAnd<Var<T>> for P1
where 
    Var<T>: From<P1> + BitAnd<Output = Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn bitand(self, rhs: Var<T>) -> Var<T> { Var::<T>::from(P1) & rhs }
}

// ----- 负一(N1)的位与运算 -----
// N1与任何类型相与都返回右操作数
impl<I: TypedInt> BitAnd<I> for N1 {
    type Output = I;
    #[inline(always)]
    fn bitand(self, rhs: I) -> I { rhs }
}

// N1与变量类型相与
impl<T: Primitive> BitAnd<Var<T>> for N1 {
    type Output = N1;
    #[inline(always)]
    fn bitand(self, _: Var<T>) -> N1 { N1 }
}

// ----- 二进制0位(B0)的位与运算 -----
impl<H: NonZero> BitAnd<Z0> for B0<H> { type Output = Z0; fn bitand(self, _: Z0) -> Z0 { Z0 } }
impl<H: NonZero> BitAnd<P1> for B0<H> { type Output = Z0; fn bitand(self, _: P1) -> Z0 { Z0 } }
impl<H: NonZero> BitAnd<N1> for B0<H> { type Output = Self; fn bitand(self, _: N1) -> Self { self } }

// B0与B0相与
impl<H1: NonZero + BitAnd<H2, Output: IfB0>, H2: NonZero> BitAnd<B0<H2>> for B0<H1> {
    type Output = <H1::Output as IfB0>::Output;
    fn bitand(self, _: B0<H2>) -> Self::Output { <<H1 as BitAnd<H2>>::Output as IfB0>::b0() }
}

// B0与B1相与
impl<H1: NonZero + BitAnd<H2, Output: IfB0>, H2: NonZero> BitAnd<B1<H2>> for B0<H1> {
    type Output = <H1::Output as IfB0>::Output;
    fn bitand(self, _: B1<H2>) -> Self::Output { <<H1 as BitAnd<H2>>::Output as IfB0>::b0() }
}

// B0与变量类型相与
impl<H: NonZero, T: Primitive> BitAnd<Var<T>> for B0<H>
where
    Var<T>: From<B0<H>> + BitAnd<Output=Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn bitand(self, rhs: Var<T>) -> Var<T> { Var::<T>::from(self) & rhs }
}

// ----- 二进制1位(B1)的位与运算 -----
impl<H: NonZero> BitAnd<Z0> for B1<H> { type Output = Z0; fn bitand(self, _: Z0) -> Z0 { Z0 } }
impl<H: NonZero> BitAnd<P1> for B1<H> { type Output = P1; fn bitand(self, _: P1) -> P1 { P1 } }
impl<H: NonZero> BitAnd<N1> for B1<H> { type Output = Self; fn bitand(self, _: N1) -> Self { self } }

// B1与B0相与
impl<H1: NonZero + BitAnd<H2, Output: IfB0>, H2: NonZero> BitAnd<B0<H2>> for B1<H1> {
    type Output = <H1::Output as IfB0>::Output;
    fn bitand(self, _: B0<H2>) -> Self::Output { <<H1 as BitAnd<H2>>::Output as IfB0>::b0() }
}

// B1与B1相与
impl<H1: NonZero + BitAnd<H2, Output: IfB1>, H2: NonZero> BitAnd<B1<H2>> for B1<H1> {
    type Output = <H1::Output as IfB1>::Output;
    fn bitand(self, _: B1<H2>) -> Self::Output { <<H1 as BitAnd<H2>>::Output as IfB1>::b1() }
}

// B1与变量类型相与
impl<H: NonZero, T: Primitive> BitAnd<Var<T>> for B1<H>
where 
    Var<T>: From<B1<H>> + BitAnd<Output = Var<T>>
{
    type Output = Var<T>;
    #[inline(always)]
    fn bitand(self, rhs: Var<T>) -> Var<T> { Var::<T>::from(self) & rhs }
}

// ----- 定点数运算 -----
// 定点数的位与运算（整数部分和小数部分分别进行位与）
impl<I1: TypedInt + BitAnd<I2>, I2: TypedInt, F1: TypedInt + BitAnd<F2>, F2: TypedInt> 
    BitAnd<FixedPoint<I2, F2>> for FixedPoint<I1, F1> 
{
    type Output = FixedPoint<I1::Output, F1::Output>;
    #[inline(always)]
    fn bitand(self, _: FixedPoint<I2, F2>) -> Self::Output { FixedPoint::new() }
}

// ----- 变量类型运算 -----
// 变量类型与其他类型的位与运算
impl<T: Primitive, I: TypedInt + BitAnd<Var<T>>> BitAnd<I> for Var<T> {
    type Output = <I as BitAnd<Var<T>>>::Output;
    #[inline(always)]
    fn bitand(self, rhs: I) -> Self::Output { rhs & self }
}

impl<T: Primitive + BitAnd<Output = T>> BitAnd<Var<T>> for Var<T> {
    type Output = Var<T>;
    #[inline(always)]
    fn bitand(self, rhs: Var<T>) -> Self::Output { Var(self.0 & rhs.0) }
}

#[cfg(test)]
mod tests {
    use crate::number::{B0, B1, P1, N1, Z0, Var, FixedPoint};

    #[test]
    fn test_z0_bitand() {
        // Z0 与任何类型相与都是 Z0
        assert_eq!(Z0 & Z0, Z0);
        assert_eq!(Z0 & P1, Z0);
        assert_eq!(Z0 & N1, Z0);
        
        let b0 = B0::<P1>::new();
        let b1 = B1::<P1>::new();
        assert_eq!(Z0 & b0, Z0);
        assert_eq!(Z0 & b1, Z0);
        
        let var = Var(5i32);
        assert_eq!(Z0 & var, Z0);
    }

    #[test]
    fn test_p1_bitand() {
        // P1 基本位与运算
        assert_eq!(P1 & Z0, Z0);
        assert_eq!(P1 & P1, P1);
        assert_eq!(P1 & N1, P1);
        
        // P1 与二进制位运算
        let b0 = B0::<P1>::new();
        let b1 = B1::<P1>::new();
        assert_eq!(P1 & b0, Z0);
        assert_eq!(P1 & b1, P1);
        
        // P1 与变量运算
        let var = Var(3i32);
        assert_eq!(P1 & var, Var(3i32 & 1_i32));
    }

    #[test]
    fn test_n1_bitand() {
        // N1 与任何类型相与返回右操作数
        assert_eq!(N1 & Z0, Z0);
        assert_eq!(N1 & P1, P1);
        assert_eq!(N1 & N1, N1);
        
        let b0 = B0::<P1>::new();
        let b1 = B1::<P1>::new();
        assert_eq!(N1 & b0, b0);
        assert_eq!(N1 & b1, b1);
        
        // N1 与变量运算
        let var = Var(3i32);
        assert_eq!(N1 & var, N1);
    }

    #[test]
    fn test_b0_bitand() {
        let b0_p1 = B0::<P1>::new();
        let b1_p1 = B1::<P1>::new();
        let b0_n1 = B0::<N1>::new();
        
        // B0 与基本类型
        assert_eq!(b0_p1 & Z0, Z0);
        assert_eq!(b0_p1 & P1, Z0);
        assert_eq!(b0_p1 & N1, b0_p1);
        
        // B0 与 B0/B1
        assert_eq!(b0_p1 & b0_p1, B0::<P1>::new());  // P1 & P1 = P1, 但 B0 的 IfB0 返回 Z0
        assert_eq!(b0_p1 & b1_p1, B0::<P1>::new());
        assert_eq!(b0_n1 & b0_n1, B0::<N1>::new());  // N1 & N1 = N1
        
        // B0 与变量
        let var = Var(5i32);
        assert_eq!(b0_p1 & var, Var(0i32));  // B0 被视为 0
    }

    #[test]
    fn test_b1_bitand() {
        let b0_p1 = B0::<P1>::new();
        let b1_p1 = B1::<P1>::new();
        let b0_n1 = B0::<N1>::new();
        
        // B1 与基本类型
        assert_eq!(b1_p1 & Z0, Z0);
        assert_eq!(b1_p1 & P1, P1);
        assert_eq!(b1_p1 & N1, b1_p1);
        
        // B1 与 B0/B1
        assert_eq!(b1_p1 & b0_p1, B0::<P1>::new());
        assert_eq!(b1_p1 & b1_p1, B1::<P1>::new());  // P1 & P1 = P1, 但 B1 的 IfB1 返回 Z0
        assert_eq!(b0_n1 & b0_n1, B0::<N1>::new());  // N1 & N1 = N1
        
        // B1 与变量
        let var = Var(5i32);
        assert_eq!(b1_p1 & var, Var(1i32));  // B1 被视为 1
    }

    #[test]
    fn test_var_bitand() {
        let var1 = Var(5i32);
        let var2 = Var(3i32);
        
        // 变量与变量
        assert_eq!(var1 & var2, Var(5i32 & 3i32));
        
        // 变量与其他类型
        assert_eq!(var1 & Z0, Z0);
        assert_eq!(var1 & P1, Var(5i32 & 1i32));
        assert_eq!(var1 & N1, N1);
        
        let b0 = B0::<P1>::new();
        let b1 = B1::<P1>::new();
        assert_eq!(var1 & b0, Var(5i32 & 0));
        assert_eq!(var1 & b1, Var(5i32 & 1));
    }

    #[test]
    fn test_fixed_point_bitand() {
        // 定点数位与运算
        let fp1 = FixedPoint::<P1, B0<P1>>::new();
        let fp2 = FixedPoint::<N1, B1<P1>>::new();
        assert_eq!(fp1 & fp2, FixedPoint::<P1, B0<P1>>::new());
        let _result = fp1 & fp2;
        // 这里只是验证编译通过，实际测试需要根据具体实现补充
    }
}