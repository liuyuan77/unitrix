use core::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};

use crate::number::{Z0, P1, N1, B0, B1, NonZero, Var, Non};

// ========== 相等比较 ==========
impl PartialEq<Z0> for Z0 {
    /// `0 == 0` 恒为真
    #[inline(always)]
    fn eq(&self, _: &Z0) -> bool {
        true
    }
}

impl<I:NonZero> PartialEq<I> for Z0 {
    /// `0 == 非0` 恒为假
    #[inline(always)]
    fn eq(&self, _: &I) -> bool {
        false
    }
}

impl<T> PartialEq<Var<T>> for Z0 
where
    T: PartialEq + Default,
{
    /// `0 == Var(x)` 转换为 `x == T::default()`
    #[inline(always)]
    fn eq(&self, rhs: &Var<T>) -> bool {
        rhs.0 == T::default()
    }
}

// 实现Eq trait（需要所有字段实现Eq）
impl Eq for Z0 {}

// ========== 大小比较 ==========
impl PartialOrd<Z0> for Z0 {
    /// `0.cmp(&0)` 返回 Equal
    fn partial_cmp(&self, _: &Z0) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl PartialOrd<P1> for Z0 {
    /// `0.cmp(&1)` 返回 Less
    fn partial_cmp(&self, _: &P1) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

impl PartialOrd<N1> for Z0 {
    /// `0.cmp(&-1)` 返回 Greater
    fn partial_cmp(&self, _: &N1) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}

impl<H> PartialOrd<B0<H>> for Z0 
where
    Z0: PartialOrd<H>, // 递归比较约束
{
    #[inline(always)]
    fn partial_cmp(&self, rhs: &B0<H>) -> Option<Ordering> {
        self.partial_cmp(&rhs.0) // 递归比较高位
    }
}

impl<H> PartialOrd<B1<H>> for Z0 
where
    Z0: PartialOrd<H>,
{
    #[inline(always)]
    fn partial_cmp(&self, rhs: &B1<H>) -> Option<Ordering> {
        self.partial_cmp(&rhs.0) // 递归比较高位
    }
}

impl<T> PartialOrd<Var<T>> for Z0 
where
    T: PartialOrd + Default,
{
    /// `0.cmp(&Var(x))` 转换为 `T::default().cmp(&x)`
    fn partial_cmp(&self, rhs: &Var<T>) -> Option<Ordering> {
        T::default().partial_cmp(&rhs.0)
    }
}

// 实现Ord trait（需要全序关系）
impl Ord for Z0 {
    fn cmp(&self, other: &Z0) -> Ordering {
        Ordering::Equal
    }
}