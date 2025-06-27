/* 数字类型按位异或运算实现
 * 作者：$ource
 * 版本：0.0
 * 创建时间：2025-06-27
 */
use core::ops::{BitXor,Not};
use crate::number::{IfB0, IfB1, NonZero, Primitive, TypedInt, Unsigned, Var, B0, B1, N1, P1, Z0, FixedPoint};

// ==================== 按位异或（^运算符） ====================
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

// ==================== P1 ^ All ====================
// P1 ^ Z0
impl BitXor<Z0> for P1 {
    type Output = P1;
    fn bitxor(self, _: Z0) -> Self::Output {
        P1
    }
}

// P1 ^ P1
impl BitXor<P1> for P1 {
    type Output = Z0;
    fn bitxor(self, _: P1) -> Self::Output {
        Z0
    }
}

// P1 ^ N1
impl BitXor<N1> for P1 {
    type Output = B0<N1>;
    fn bitxor(self, _: N1) -> Self::Output {
        B0::new()
    }
}

// P1 ^ B0
impl<H: NonZero + IfB1> BitXor<B0<H>> for P1 {
    type Output = H::Output;
    fn bitxor(self, _rhs: B0<H>) -> Self::Output {
        <H as IfB1>::b1()
    }
}

// P1 ^ B1
impl<H: NonZero + IfB0> BitXor<B1<H>> for P1 {
    type Output = <H as IfB0>::Output;
    fn bitxor(self, _rhs: B1<H>) -> Self::Output {
        <H as IfB0>::b0()
    }
}

// P1 ^ Var<T> = Var<T>
impl<T: Primitive> BitXor<Var<T>> for P1
where 
    Var<T>: From<P1> + BitXor<Output = Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn bitxor(self, rhs: Var<T>) -> Self::Output {
        Var::<T>::from(self) ^ rhs
    }
}

// ==================== N1 ^ All ====================
// N1 ^ All
impl<I: TypedInt + Not> BitXor<I> for N1 {
    type Output = <I as Not>::Output;
    fn bitxor(self, rhs: I) -> Self::Output {
        !rhs
    }
}

// N1 ^ Var<T> = Var<T>
impl<T: Primitive> BitXor<Var<T>> for N1
where 
    Var<T>: Not<Output = Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn bitxor(self, rhs: Var<T>) -> Self::Output {
        !rhs
    }
}

// ==================== B0 ^ All ====================
// B0 ^ Z0
impl<H: NonZero> BitXor<Z0> for B0<H> {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, _rhs: Z0) -> Self::Output {
        self
    }
}

// B0 ^ P1
impl<H: NonZero + IfB1> BitXor<P1> for B0<H> {
    type Output = H::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: P1) -> Self::Output {
        <H as IfB1>::b1()
    }
}

// B0 ^ N1
impl<H: NonZero> BitXor<N1> for B0<H>
where 
    B0<H>: Not,
{
    type Output = <B0<H> as Not>::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: N1) -> Self::Output {
        !self
    }
}

// B0 ^ B0
impl<H1: NonZero + BitXor<H2, Output: IfB0>, H2: NonZero> BitXor<B0<H2>> for B0<H1> {
    type Output = <H1::Output as IfB0>::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: B0<H2>) -> Self::Output {
        <<H1 as BitXor<H2>>::Output as IfB0>::b0()
    }
}

// B0 ^ B1
impl<H1: NonZero + BitXor<H2, Output: IfB1>, H2: NonZero> BitXor<B1<H2>> for B0<H1> {
    type Output = <H1::Output as IfB1>::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: B1<H2>) -> Self::Output {
        <<H1 as BitXor<H2>>::Output as IfB1>::b1()
    }
}

// B0<H> ^ Var<T> = Var<T>
impl<H: NonZero, T: Primitive> BitXor<Var<T>> for B0<H>
where 
    Var<T>: From<B0<H>> + BitXor<Output = Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn bitxor(self, rhs: Var<T>) -> Self::Output {
        Var::<T>::from(self) ^ rhs
    }
}

// ==================== B1 ^ All ====================
// B1 ^ Z0
impl<H: NonZero> BitXor<Z0> for B1<H> {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, _rhs: Z0) -> Self::Output {
        self
    }
}

// B1 ^ P1
impl<H: NonZero + IfB0> BitXor<P1> for B1<H> {
    type Output = H::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: P1) -> Self::Output {
        <H as IfB0>::b0()
    }
}

// B1 ^ N1
impl<H: NonZero> BitXor<N1> for B1<H>
where 
    B1<H>:Not,
{
    type Output = <B1<H> as Not>::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: N1) -> Self::Output {
        !self
    }
}

// B1 ^ B0
impl<H1: NonZero + BitXor<H2, Output: IfB1>, H2: NonZero> BitXor<B0<H2>> for B1<H1> {
    type Output = <H1::Output as IfB1>::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: B0<H2>) -> Self::Output {
        <<H1 as BitXor<H2>>::Output as IfB1>::b1()
    }
}

// B1 ^ B1
impl<H1: NonZero + BitXor<H2, Output: IfB0>, H2: NonZero> BitXor<B1<H2>> for B1<H1> {
    type Output = <H1::Output as IfB0>::Output;
    #[inline(always)]
    fn bitxor(self, _rhs: B1<H2>) -> Self::Output {
        <<H1 as BitXor<H2>>::Output as IfB0>::b0()
    }
}

// B1 ^ Var<T> = Var<T>
impl<H: NonZero, T: Primitive> BitXor<Var<T>> for B1<H>
where 
    Var<T>: From<B1<H>> + BitXor<Output = Var<T>>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn bitxor(self, rhs: Var<T>) -> Self::Output {
        Var::<T>::from(self) ^ rhs
    }
}

// ----- 定点数(FixedPoint) -----
impl<I1, I2, F1, F2> BitXor<FixedPoint<I2, F2>> for FixedPoint<I1, F1>
where
    I1: TypedInt + BitXor<I2>,
    I2: TypedInt,
    F1: Unsigned + BitXor<F2>,
    F2: Unsigned,
{
    type Output = FixedPoint<<I1 as BitXor<I2>>::Output, <F1 as BitXor<F2>>::Output>;
    fn bitxor(self, _rhs: FixedPoint<I2, F2>) -> Self::Output {
        FixedPoint::new()
    }
}

// 浮点数实现无意义

// =============== 复合类型实现 ===============

// ----- 变量类型(Var<T>)的位或运算 -----
impl<T: Primitive> BitXor<Var<T>> for Var<T> 
where
    T: BitXor<Output = T>,
{
    type Output = Var<T>;
    #[inline(always)]
    fn bitxor(self, rhs: Var<T>) -> Var<T> {
        Var(self.0 ^ rhs.0)
    }
}

impl<T: Primitive, I: TypedInt> BitXor<I> for Var<T> 
where
    I: BitXor<Var<T>>,
{
    type Output = <I as BitXor<Var<T>>>::Output;
    fn bitxor(self, rhs: I) -> Self::Output {
        rhs ^ self
    }
}

// ==================== 测试代码 ====================
#[cfg(test)]
mod tests {
    use crate::number::*;
    
    #[test]
    fn test_z0_operations() {
        // Z0 ^ I = I
        assert_eq!(Z0 ^ Z0, Z0);
        assert_eq!(Z0 ^ P1, P1);
        assert_eq!(Z0 ^ N1, N1);
        
        let b0 = B0::<P1>::new();
        let b1 = B1::<P1>::new();
        assert_eq!(Z0 ^ b0, b0);
        assert_eq!(Z0 ^ b1, b1);
        
        let var = Var(42);
        assert_eq!(Z0 ^ var, var);
    }
    
    #[test]
    fn test_p1_operations() {
        // P1 ^ Z0 = P1
        assert_eq!(P1 ^ Z0, P1);
        // P1 ^ P1 = Z0
        assert_eq!(P1 ^ P1, Z0);
        // P1 ^ N1 = B0<N1>
        assert_eq!(P1 ^ N1, B0::<N1>::new());
        
        let b0 = B0::<P1>::new();
        let b1 = B1::<P1>::new();
        // P1 ^ B0<P1> = P1::Output (B1)
        assert_eq!(P1 ^ b0, B1::<P1>::new());
        // P1 ^ B1<P1> = P1::Output (B0)
        assert_eq!(P1 ^ b1, B0::<P1>::new());
        
        let var = Var(42);
        assert_eq!(P1 ^ var, Var(42 ^ 1)); // 1 is P1's value
    }
    
    #[test]
    fn test_n1_operations() {
        // N1 ^ Z0 = !Z0 = N1 (假设!Z0 = N1)
        assert_eq!(N1 ^ Z0, N1);
        // N1 ^ P1 = !P1 = N1 (假设!P1 = N1)
        assert_eq!(N1 ^ P1, B0::<N1>::new());
        // N1 ^ N1 = !N1 = P1 (假设!N1 = P1)
        assert_eq!(N1 ^ N1, Z0);
        
        let b0 = B0::<P1>::new();
        let b1 = B1::<P1>::new();
        // N1 ^ B0 = !B0 = B1 (假设!B0 = B1)
        assert_eq!(N1 ^ b0, B1::<B0<N1>>::new());
        // N1 ^ B1 = !B1 = B0 (假设!B1 = B0)
        assert_eq!(N1 ^ b1, B0::<B0<N1>>::new());
        
        let var = Var(42);
        assert_eq!(N1 ^ var, Var(!42)); // ! is bitwise NOT
    }
    
    #[test]
    fn test_b0_operations() {
        let b0_p1 = B0::<P1>::new();
        
        // B0 ^ Z0 = B0
        assert_eq!(b0_p1 ^ Z0, b0_p1);
        
        // B0 ^ P1 = H::Output
        assert_eq!(b0_p1 ^ P1, B1::<P1>::new());
        
        // B0 ^ N1 = !B0
        assert_eq!(b0_p1 ^ N1, B1::<B0<N1>>::new());
        
        // B0 ^ B0
        assert_eq!(b0_p1 ^ B0::<P1>::new(), Z0);
        assert_eq!(b0_p1 ^ B0::<N1>::new(), B0::<B0<N1>>::new());
        
        // B0 ^ B1
        assert_eq!(b0_p1 ^ B1::<P1>::new(), P1);
        
        let var = Var(42);
        assert_eq!(b0_p1 ^ var, Var(2 ^ 42)); // B0 is 0
    }
    
    #[test]
    fn test_b1_operations() {
        let b1_p1 = B1::<P1>::new();
        
        // B1 ^ Z0 = B1
        assert_eq!(b1_p1 ^ Z0, b1_p1);
        
        // B1 ^ P1 = H::Output
        assert_eq!(b1_p1 ^ P1, B0::<P1>::new());
        
        // B1 ^ N1 = !B1
        assert_eq!(b1_p1 ^ N1, B0::<B0<N1>>::new());
        
        // B1 ^ B0
        assert_eq!(b1_p1 ^ B0::<P1>::new(), P1);
        
        // B1 ^ B1
        assert_eq!(b1_p1 ^ B1::<P1>::new(), Z0);
        assert_eq!(b1_p1 ^ N1, B0::<B0<N1>>::new());
        
        let var = Var(42);
        assert_eq!(b1_p1 ^ var, Var(3 ^ 42)); // B1 is 1
    }
    
    #[test]
    fn test_var_operations() {
        let var1 = Var(42);
        let var2 = Var(13);
        
        // Var ^ Var
        assert_eq!(var1 ^ var2, Var(42 ^ 13));
        
        // Var ^ Z0
        assert_eq!(var1 ^ Z0, var1);
        
        // Var ^ P1
        assert_eq!(var1 ^ P1, Var(42 ^ 1));
        
        // Var ^ N1
        assert_eq!(var1 ^ N1, Var(!42));
        
        let b0 = B0::<P1>::new();
        let b1 = B1::<P1>::new();
        // Var ^ B0
        assert_eq!(var1 ^ b0, Var(42 ^ 2));//  101010   101000
        // Var ^ B1
        assert_eq!(var1 ^ b1, Var(42 ^ 3));
    }
}