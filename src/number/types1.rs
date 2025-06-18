use core::marker::PhantomData;
use crate::sealed::Sealed;

/// 特殊浮点数值
#[derive(Debug, PartialEq, Default)]
pub enum Spec {
    #[default]
    Nan,        // 非数字
    Infinity,   // 正无穷
    NegInfinity,// 负无穷
}

// ========== 基础数值类型表示 ==========

/// 二进制0表示
/// - 可以用于低位或独立使用，不能用于高位
/// - 独立使用时表示值0
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct Z;

/// 正数终结符/数值1表示
/// - 独立使用：表示值1
/// - 作为泛型参数：
///   - 当前位为1
///   - 高位使用时表示更高位为0
///   - 示例：`B<P, P>` 表示 `011`（+3）
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct P;

/// 负数终结符/数值-1表示
/// - 独立使用：表示值-1
/// - 作为泛型高位参数：
///   - 当前位为1
///   - 更高位为1（补码表示）
///   - 示例：`B<N, Z>` 表示 `...1110`（-2）
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct N;

/// 补码表示的二进制链表
/// - `H`: 高位类型，可以是 `N`、`P` 或 `B<H, L>`
///   - `N` 表示当前位为1且更高位为1
///   - `P` 表示当前位为1且更高位为0
///   - `B<H, L>` 递归表示更高位
/// - `L`: 低位类型，可以是 `Z` 或 `P`
///   - `Z` 表示0
///   - `P` 表示1
/// - 示例：`B<P, Z>` 表示 `010`（+2）
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct B<H, L>(PhantomData<(H, L)>);

impl<H, L> Default for B<H, L> {
    fn default() -> Self { B(PhantomData) }
}

/// 类型级科学计数法表示的浮点数（M × 2^E）
/// - `Mantissa`: 尾数部分，表示有效数字（包含符号）
/// - `Exponent`: 指数部分，使用补码表示
/// - 支持特殊值：NaN、+∞、-∞
#[derive(Clone, Copy, Debug)]
pub struct F<Mantissa, Exponent>(PhantomData<(Mantissa, Exponent)>);

impl<Mantissa, Exponent> Default for F<Mantissa, Exponent> {
    fn default() -> Self { F(PhantomData) }
}

/// 数值桥接类型
/// - 用于连接库类型与原生数值类型
/// - 支持自定义类型与原生类型的混合运算
/// - 提供类型安全的运算符重载
/// - 示例：`V(3) + P` → `i32 + 类型级1`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct V<T>(pub T);

// ========== 构造函数实现 ==========

impl Z {
    #[inline] 
    pub fn new() -> Self { Z } 
}

impl P {
    #[inline] 
    pub fn new() -> Self { P } 
}

impl N {
    #[inline] 
    pub fn new() -> Self { N } 
}

impl<H, L> B<H, L> { 
    #[inline] 
    pub fn new() -> Self { B(PhantomData) } 
}

impl<Mantissa, Exponent> F<Mantissa, Exponent> {
    #[inline] 
    pub fn new() -> Self { F(PhantomData) }
}

// ========== Sealed trait实现 ==========

impl Sealed for Spec {}
impl Sealed for Z {}
impl Sealed for P {}
impl Sealed for N {}
impl<H, L> Sealed for B<H, L> {}
impl<Mantissa, Exponent> Sealed for F<Mantissa, Exponent> {}
impl Sealed for V<i8> {}
impl Sealed for V<i16> {}
impl Sealed for V<i32> {}
impl Sealed for V<i64> {}
impl Sealed for V<i128> {}
impl Sealed for V<isize> {}
impl Sealed for V<f32> {}
impl Sealed for V<f64> {}