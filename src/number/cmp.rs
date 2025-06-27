use core::cmp::Ordering;
use core::default::Default;
use crate::sealed::Sealed;
use crate::number::{Z0, P1, N1, B0, B1, NonZero};
///类型级比较,仅考虑类型级整数比较

//-------------基本定义-------------------------------------------------
// 比较结果类型别名
pub type Equal = Z0;   // 相等
pub type Less = N1;    // 小于
pub type Greater = P1; // 大于

/// 标记特征(trait)，用于比较结果类型：`Greater`、`Equal` 和 `Less`
pub trait Ord: Sealed {
    /// 将类型转换为运行时的 `Ordering` 值
    fn to_ordering() -> Ordering;
}

impl Ord for Greater {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Greater  // 返回"大于"的比较结果
    }
}

impl Ord for Less {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Less  // 返回"小于"的比较结果
    }
}

impl Ord for Equal {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Equal  // 返回"等于"的比较结果
    }
}

/// 类型比较运算符，用于比较 `Self` 和 `Rhs` 类型。
/// 类似于 [`core::cmp::Ord::cmp`] 但用于类型比较。
///
/// # 示例
/// ```rust
/// use unitrix::number::{Cmp, Ord, N1, Z0, P1};
/// use std::cmp::Ordering;
///
/// assert_eq!(<Z0 as Cmp<N1>>::Output::to_ordering(), Ordering::Greater);
/// assert_eq!(<Z0 as Cmp<Z0>>::Output::to_ordering(), Ordering::Equal);
/// assert_eq!(<Z0 as Cmp<P1>>::Output::to_ordering(), Ordering::Less);
/// ```
pub trait Cmp<Rhs = Self> {
    /// 比较结果的类型（只能是 `Greater`、`Less` 或 `Equal` 之一）
    type Output;

    #[doc(hidden)]  // 隐藏内部实现
    fn compare(self, rhs: Rhs) -> Self::Output;
}

//---------------------bit 相对独立，集中保存bit.rs文件---------------------------------

//---------------------Special 未来改为错误处理，不实现比较---------------------------------

//---------------------Z0比较All---------------------------------
/// 0 == 0
impl Cmp<Z0> for Z0 {
    type Output = Equal;

    #[inline]
    fn compare(self, _: Z0) -> Self::Output {
        Equal::new()
    }
}

/// 0 < 1
impl Cmp<P1> for Z0 {
    type Output = Less;

    #[inline]
    fn compare(self, _: P1) -> Self::Output {
        Less::new()
    }
}

/// 0 > -1
impl Cmp<N1> for Z0 {
    type Output = Greater;

    #[inline]
    fn compare(self, _: N1) -> Self::Output {
        Greater::new()
    }
}

/// 0 <==> B0<H>
impl<H: NonZero + Default> Cmp<B0<H>> for Z0
where
    Z0: Cmp<H>,
{
    type Output = <Z0 as Cmp<H>>::Output;

    #[inline]
    fn compare(self, _: B0<H>) -> Self::Output {
        let _h = H::default();
        <Z0 as Cmp<H>>::compare(self, _h)
    }
}

/// 0 <==> B1<H>
impl<H: NonZero + Default> Cmp<B1<H>> for Z0
where
    Z0: Cmp<H>,
{
    type Output = <Z0 as Cmp<H>>::Output;

    #[inline]
    fn compare(self, _: B1<H>) -> Self::Output {
        let _h = H::default();
        <Z0 as Cmp<H>>::compare(self, _h)
    }
}

//---------------------P1比较All---------------------------------
//---------------------P1比较All---------------------------------
/// 1 > 0
impl Cmp<Z0> for P1 {
    type Output = Greater;

    #[inline]
    fn compare(self, _: Z0) -> Self::Output {
        Greater::new()
    }
}

/// 1 == 1
impl Cmp<P1> for P1 {
    type Output = Equal;

    #[inline]
    fn compare(self, _: P1) -> Self::Output {
        Equal::new()
    }
}

/// 1 > -1
impl Cmp<N1> for P1 {
    type Output = Greater;

    #[inline]
    fn compare(self, _: N1) -> Self::Output {
        Greater::new()
    }
}

/// P1 <==> B0<H>
impl<H: NonZero + Default> Cmp<B0<H>> for P1
where
    Z0: Cmp<H>,
{
    type Output = <Z0 as Cmp<H>>::Output;

    #[inline]
    fn compare(self, _: B0<H>) -> Self::Output {
        let _h = H::default();
        <Z0 as Cmp<H>>::compare(Z0::new(), _h)
    }
}

/// P1 <==> B1<H>
impl<H: NonZero + Default> Cmp<B1<H>> for P1
where
    Z0: Cmp<H>,
{
    type Output = <Z0 as Cmp<H>>::Output;

    #[inline]
    fn compare(self, _: B1<H>) -> Self::Output {
        let _h = H::default();
        <Z0 as Cmp<H>>::compare(Z0::new(), _h)
    }
}


//---------------------N1比较All---------------------------------
/// -1 < 0
impl Cmp<Z0> for N1 {
    type Output = Less;

    #[inline]
    fn compare(self, _: Z0) -> Self::Output {
        Less::new()
    }
}

/// -1 < 1
impl Cmp<P1> for N1 {
    type Output = Less;

    #[inline]
    fn compare(self, _: P1) -> Self::Output {
        Less::new()
    }
}

/// -1 == -1
impl Cmp<N1> for N1 {
    type Output = Equal;

    #[inline]
    fn compare(self, _: N1) -> Self::Output {
        Equal::new()
    }
}

/// N1 <==> B0<H>
impl<H: NonZero + Default> Cmp<B0<H>> for N1
where
    Z0: Cmp<H>,
{
    type Output = <Z0 as Cmp<H>>::Output;

    #[inline]
    fn compare(self, _: B0<H>) -> Self::Output {
        let _h = H::default();
        <Z0 as Cmp<H>>::compare(Z0::new(), _h)
    }
}

/// N1 <==> B1<H>
impl<H: NonZero + Default> Cmp<B1<H>> for N1
where
    Z0: Cmp<H>,
{
    type Output = <Z0 as Cmp<H>>::Output;

    #[inline]
    fn compare(self, _: B1<H>) -> Self::Output {
        let _h = H::default();
        <Z0 as Cmp<H>>::compare(Z0::new(), _h)
    }
}

//---------------------B0<H>比较All---------------------------------

/// B0<H> vs Z0: 比较H与Z0
impl<H: NonZero + Default> Cmp<Z0> for B0<H>
where
    H: Cmp<Z0>,
{
    type Output = <H as Cmp<Z0>>::Output;

    #[inline]
    fn compare(self, _: Z0) -> Self::Output {
        let _h = H::default();
        <H as Cmp<Z0>>::compare(_h, Z0::new())
    }
}

/// B0<H> vs P1: 比较H与Z0
impl<H: NonZero + Default> Cmp<P1> for B0<H>
where
    H: Cmp<Z0>,
{
    type Output = <H as Cmp<Z0>>::Output;

    #[inline]
    fn compare(self, _: P1) -> Self::Output {
        let _h = H::default();
        <H as Cmp<Z0>>::compare(_h, Z0)
    }
}

/// B0<H> vs N1: 比较H与Z0 
impl<H: NonZero + Default> Cmp<N1> for B0<H>
where
    H: Cmp<Z0>,
{
    type Output = <H as Cmp<Z0>>::Output;

    #[inline]
    fn compare(self, _: N1) -> Self::Output {
        let _h = H::default();
        <H as Cmp<Z0>>::compare(_h, Z0)
    }
}

/// B0<H1> vs B0<H2>: 比较H1与H2
impl<H1: NonZero + Default, H2: NonZero + Default> Cmp<B0<H2>> for B0<H1>
where
    H1: Cmp<H2>,
{
    type Output = <H1 as Cmp<H2>>::Output;

    #[inline]
    fn compare(self, _: B0<H2>) -> Self::Output {
        let _h1 = H1::default();
        let _h2 = H2::default();
        <H1 as Cmp<H2>>::compare(_h1, _h2)
    }
}

/// B0<H1> vs B1<H2>: 先比较H1与H2，如果相等则B1>B0
impl<H1: NonZero + Default, H2: NonZero + Default> Cmp<B1<H2>> for B0<H1>
where
    H1: Cmp<H2>,
    <H1 as Cmp<H2>>::Output: IfLess,
    <<H1 as Cmp<H2>>::Output as IfLess>::Output:Default,
{
    type Output = <<H1 as Cmp<H2>>::Output as IfLess>::Output;

    #[inline]
    fn compare(self, _: B1<H2>) -> Self::Output {
        Self::Output::default()
    }
}

// 辅助类型用于处理比较结果
pub trait IfLess {
    type Output;
    fn less() -> Self::Output;
}

impl IfLess for Equal {
    type Output = Less;
    fn less() -> Self::Output{
        Less::new()
    }
}

impl IfLess for Less {
    type Output = Less;
    fn less() -> Self::Output{
        Less::new()
    }
}

impl IfLess for Greater {
    type Output = Greater;
    fn less() -> Self::Output{
        Greater::new()
    }
}

//---------------------B1<H>比较All---------------------------------

/// B1<H> vs Z0: 比较H与Z0
impl<H: NonZero + Default> Cmp<Z0> for B1<H>
where
    H: Cmp<Z0>,
{
    type Output = <H as Cmp<Z0>>::Output;

    #[inline]
    fn compare(self, _: Z0) -> Self::Output {
        let _h = H::default();
        <H as Cmp<Z0>>::compare(_h, Z0)
    }
}

/// B1<H> vs P1: 比较H与Z0
impl<H: NonZero + Default> Cmp<P1> for B1<H>
where
    H: Cmp<Z0>,
{
    type Output = <H as Cmp<Z0>>::Output;

    #[inline]
    fn compare(self, _: P1) -> Self::Output {
        let _h = H::default();
        <H as Cmp<Z0>>::compare(_h, Z0)
    }
}

/// B1<H> vs N1: 比较H与Z0
impl<H: NonZero + Default> Cmp<N1> for B1<H>
where
    H: Cmp<Z0>,
{
    type Output = <H as Cmp<Z0>>::Output;

    #[inline]
    fn compare(self, _: N1) -> Self::Output {
        let _h = H::default();
        <H as Cmp<Z0>>::compare(_h, Z0)
    }
}

/// B1<H1> vs B0<H2>: 先比较H1与H2，如果相等则B1>B0
impl<H1: NonZero + Default, H2: NonZero + Default> Cmp<B0<H2>> for B1<H1>
where
    H1: Cmp<H2>,
    <H1 as Cmp<H2>>::Output: IfGreater,
    <<H1 as Cmp<H2>>::Output as IfGreater>::Output: Default,
{
    type Output = <<H1 as Cmp<H2>>::Output as IfGreater>::Output;

    #[inline]
    fn compare(self, _: B0<H2>) -> Self::Output {
        Self::Output::default()
    }
}

/// B1<H1> vs B1<H2>: 比较H1与H2
impl<H1: NonZero + Default, H2: NonZero + Default> Cmp<B1<H2>> for B1<H1>
where
    H1: Cmp<H2>,
{
    type Output = <H1 as Cmp<H2>>::Output;

    #[inline]
    fn compare(self, _: B1<H2>) -> Self::Output {
        let _h1 = H1::default();
        let _h2 = H2::default();
        <H1 as Cmp<H2>>::compare(_h1, _h2)
    }
}

// 新增辅助trait用于处理B1>B0的情况
pub trait IfGreater {
    type Output;
    fn greater() -> Self::Output;
}

impl IfGreater for Equal {
    type Output = Greater;
    fn greater() -> Self::Output {
        Greater::new()
    }
}

impl IfGreater for Less {
    type Output = Less;
    fn greater() -> Self::Output {
        Less::new()
    }
}

impl IfGreater for Greater {
    type Output = Greater;
    fn greater() -> Self::Output {
        Greater::new()
    }
}