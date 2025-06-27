use core::marker::PhantomData;
use crate::sealed::Sealed;

//===============================================
// 特殊浮点值枚举
//===============================================

/// 特殊浮点值（NaN/±∞）
#[derive(Debug, PartialEq, Default)]
pub enum Special {
    #[default]
    Nan,            // Not a Number
    Infinity,       // Positive infinity
    NegInfinity,    // Negative infinity
}

//===============================================
// 基础数值类型表示
//===============================================

/// 二进制0的终结表示（类型系统中的原子常量）
/// - 不能作为小数 `B0`/`B1` 的泛型参数
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct Z0;

/// 正号或小数终结符/数值1表示：
/// - 独立使用：值 = 1
/// - 作为泛型参数时：当前位=1，高位=0
///   - 示例：`B1<P1>` 表示二进制 `011`（十进制 +3）
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct P1;

/// 负号终结符/数值-1表示：
/// - 独立使用：值 = -1
/// - 作为泛型参数时：当前位=1，高位=1（二进制补码）
///   - 示例：`B0<N1>` 表示二进制 `...1110`（十进制 -2）
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct N1;

/// 二进制补码的0位：
/// - `Other`: 整数的高位类型或小数的低位类型
/// - 示例：`B0<P1>` 表示二进制 `010`（十进制 +2）
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct B0<Other>(PhantomData<Other>);

impl<Other> Default for B0<Other> {
    fn default() -> Self {
        B0(PhantomData)
    }
}

/// 二进制补码的1位：
/// - `Other`: 整数的高位类型或小数的低位类型
/// - 示例：`B1<P1>` 表示二进制 `011`（十进制 +3）
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct B1<Other>(PhantomData<Other>);

impl<Other> Default for B1<Other> {
    fn default() -> Self {
        B1(PhantomData)
    }
}

//===============================================
// 复合数值类型表示
//===============================================

/// **定点数表示（整数部分 + 小数部分）**
/// - `IntPart`:  整数部分（二进制补码表示，如 `B1<P1>` 表示整数 3）
/// - `FracPart`: 小数部分（二进制小数，单独的Z0、P1，或使用 `B0`/`B1` 嵌套链表示并用 `P1` 结束）
/// 
/// # 表示规则
/// - **整数部分**：标准二进制补码（同整数规则）
/// - **小数部分**：从高位到低位（2^{-1}, 2^{-2}, ...）的链式结构：
///   - `B0<Next>` = 当前小数位为 0
///   - `B1<Next>` = 当前小数位为 1
///   - `P1` = 为1，也是B0、B1结束符
///   - `Z0` = 单独使用，表示0
/// 
/// # 示例
/// 3.5 的定点表示：
/// - 整数部分: `B1<P1>`（二进制 `11` = 3）
/// - 小数部分: `P1`（二进制 `0.1` = 0.5）
/// - 完整类型: `FixedPoint<B1<P1>, B1<Z0>>`
/// 
/// 
///   - 始终为无符号，二进制位是整数部分的延续
///
/// # 二进制布局规则
/// ```text
/// [符号位][整数位][小数位]
///   MSB -----------------> LSB
/// ```
///
/// # 示例
/// ```
/// type Q1_3 = FixedPoint<B1<P1>, B0<B1<P1>>>;  // -1.011 (二进制)
/// type Q3_5 = FixedPoint<B0<B1<P1>>, B1<B0<P1>>>; // 3.101
/// ```
///
/// # 编译时强制约束
/// 1. 整数部分必须为合法二进制补码形式
/// 2. 小数部分禁止包含 `Z0`
/// 3. 两部分都必须以 `P1` 结尾
/// 
/// 
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct FixedPoint<IntPart, FracPart>(PhantomData<(IntPart, FracPart)>);

impl<IntPart, FracPart> Default for FixedPoint<IntPart, FracPart> {
    fn default() -> Self {
        FixedPoint::new()
    }
}


/// **类型级浮点数（科学计数法 M × 2^E）**
/// - `Significand`: 尾数（定点数，用 `FixedPoint<IntPart, FracPart>` 表示）
/// - `Exponent`: 指数（二进制补码表示）
/// - 支持特殊值：NaN, ±∞
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Float<Significand, Exponent>(PhantomData<(Significand, Exponent)>);

impl<Significand, Exponent> Default for Float<Significand, Exponent> {
    fn default() -> Self {
        Float(PhantomData)
    }
}

/// **原生数值的包装类型**
/// - 在自定义类型和原生类型间搭建桥梁
/// - 支持类型安全的运算符重载
/// - 示例：`Var(3) + P1` → `i32 + 类型级1`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Var<T>(pub T);

//===============================================
// 构造函数实现
//===============================================

impl Z0 {
    #[inline]
    pub fn new() -> Self {
        Z0
    }
}

impl P1 {
    #[inline]
    pub fn new() -> Self {
        P1
    }
}

impl N1 {
    #[inline]
    pub fn new() -> Self {
        N1
    }
}

impl<Other> B0<Other> {
    #[inline]
    pub fn new() -> Self {
        B0(PhantomData)
    }
}

impl<Other> B1<Other> {
    #[inline]
    pub fn new() -> Self {
        B1(PhantomData)
    }
}

impl<IntPart, FracPart> FixedPoint<IntPart, FracPart> {
    #[inline]
    pub fn new() -> Self {
        FixedPoint(PhantomData)
    }
}

impl<Significand, Exponent> Float<Significand, Exponent> {
    #[inline]
    pub fn new() -> Self {
        Float(PhantomData)
    }
}

//===============================================
// Sealed trait 实现（模块私有约束）
//===============================================

impl Sealed for Special {}
impl Sealed for Z0 {}
impl Sealed for P1 {}
impl Sealed for N1 {}
impl<Other> Sealed for B0<Other> {}
impl<Other> Sealed for B1<Other> {}
impl<IntPart, FracPart> Sealed for FixedPoint<IntPart, FracPart> {}
impl<Significand, Exponent> Sealed for Float<Significand, Exponent> {}
impl Sealed for Var<i8> {}
impl Sealed for Var<i16> {}
impl Sealed for Var<i32> {}
impl Sealed for Var<i64> {}
impl Sealed for Var<i128> {}
impl Sealed for Var<isize> {}
impl Sealed for Var<f32> {}
impl Sealed for Var<f64> {}