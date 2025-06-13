use super::constant::{Z0, P1, N1, B0, B1};
/// Type-level integer alias
/// 类型级整数别名
/// 
/// Converts compile-time constant N into corresponding type representation
/// 将编译时常量 N 转换为对应的类型表示
/// 
/// # Example
/// # 示例
/// ```rust
/// use unitrix::number::Const;
/// 
/// // `Const<42>` expands to `B0<B1<B0<B1<B0<Z0>>>>>`
/// // `Const<42>` 会展开为 `B0<B1<B0<B1<B0<Z0>>>>>`
/// type Answer = Const<42>;
/// ```
/// 
/// # Range
/// # 范围
/// Currently only implements integers in [-64, 64] range
/// 目前仅实现 [-64，64] 范围的整数
pub type Const<const N: i32> = <ConstInt<N> as ToInt>::Output;

// Wrapper type for representing integer constants in generic contexts
/// 用于在泛型上下文中表示整数的包装类型
/// 
/// This is a zero-sized marker type used to associate integer values with types
/// 这是一个零大小的标记类型，用于将整数值与类型关联
pub struct ConstInt<const N: i32>;

/// Trait for converting integer constants to type representations
/// 将整数常量转换为类型表示的 trait
/// 
/// This trait provides the core mechanism for type-level integer encoding
/// 该 trait 提供了类型级整数编码的核心机制
pub trait ToInt {
    // The resulting type representation
    /// 对应的类型表示
    type Output;
}

// =============================================
//  Zero Representation / 零表示
// =============================================
impl ToInt for ConstInt<0> {
    type Output = Z0;
}

// =============================================
//  Positive Numbers Representation / 正数表示
// =============================================
/* 
 * Representation Rules:
 * 1. MSB innermost, LSB outermost
 * 2. Innermost is P1 (positive sign)
 * 3. B0/B1 represent binary bits
 *
 * 表示规则：
 * 1. 最高有效位在内，最低有效位在外
 * 2. 最内层为 P1 (正号)
 * 3. B0/B1 表示二进制位
 */

// Group 1: 1-bit numbers (1-1)
impl ToInt for ConstInt<1> {
    type Output = P1;
}

// Group 2: 2-bit numbers (2-3)
impl ToInt for ConstInt<2> {
    type Output = B0<P1>;
}

impl ToInt for ConstInt<3> {
    type Output = B1<P1>;
}

// Group 3: 3-bit numbers (4-7)
impl ToInt for ConstInt<4> {
    type Output = B0<B0<P1>>;
}

impl ToInt for ConstInt<5> {
    type Output = B1<B0<P1>>;
}

impl ToInt for ConstInt<6> {
    type Output = B0<B1<P1>>;
}

impl ToInt for ConstInt<7> {
    type Output = B1<B1<P1>>;
}

// Group 4: 4-bit numbers (8-15)
impl ToInt for ConstInt<8> {
    type Output = B0<B0<B0<P1>>>;
}

impl ToInt for ConstInt<9> {
    type Output = B1<B0<B0<P1>>>;
}

impl ToInt for ConstInt<10> {
    type Output = B0<B1<B0<P1>>>;
}

impl ToInt for ConstInt<11> {
    type Output = B1<B1<B0<P1>>>;
}

impl ToInt for ConstInt<12> {
    type Output = B0<B0<B1<P1>>>;
}

impl ToInt for ConstInt<13> {
    type Output = B1<B0<B1<P1>>>;
}

impl ToInt for ConstInt<14> {
    type Output = B0<B1<B1<P1>>>;
}

impl ToInt for ConstInt<15> {
    type Output = B1<B1<B1<P1>>>;
}

// Group 5: 5-bit numbers (16-31)
impl ToInt for ConstInt<16> {
    type Output = B0<B0<B0<B0<P1>>>>;
}

impl ToInt for ConstInt<17> {
    type Output = B1<B0<B0<B0<P1>>>>;
}

impl ToInt for ConstInt<18> {
    type Output = B0<B1<B0<B0<P1>>>>;
}

impl ToInt for ConstInt<19> {
    type Output = B1<B1<B0<B0<P1>>>>;
}

impl ToInt for ConstInt<20> {
    type Output = B0<B0<B1<B0<P1>>>>;
}

impl ToInt for ConstInt<21> {
    type Output = B1<B0<B1<B0<P1>>>>;
}

impl ToInt for ConstInt<22> {
    type Output = B0<B1<B1<B0<P1>>>>;
}

impl ToInt for ConstInt<23> {
    type Output = B1<B1<B1<B0<P1>>>>;
}

impl ToInt for ConstInt<24> {
    type Output = B0<B0<B0<B1<P1>>>>;
}

impl ToInt for ConstInt<25> {
    type Output = B1<B0<B0<B1<P1>>>>;
}

impl ToInt for ConstInt<26> {
    type Output = B0<B1<B0<B1<P1>>>>;
}

impl ToInt for ConstInt<27> {
    type Output = B1<B1<B0<B1<P1>>>>;
}

impl ToInt for ConstInt<28> {
    type Output = B0<B0<B1<B1<P1>>>>;
}

impl ToInt for ConstInt<29> {
    type Output = B1<B0<B1<B1<P1>>>>;
}

impl ToInt for ConstInt<30> {
    type Output = B0<B1<B1<B1<P1>>>>;
}

impl ToInt for ConstInt<31> {
    type Output = B1<B1<B1<B1<P1>>>>;
}

// Group 6: 6-bit numbers (32-63)
impl ToInt for ConstInt<32> {
    type Output = B0<B0<B0<B0<B0<P1>>>>>;
}

impl ToInt for ConstInt<33> {
    type Output = B1<B0<B0<B0<B0<P1>>>>>;
}

impl ToInt for ConstInt<34> {
    type Output = B0<B1<B0<B0<B0<P1>>>>>;
}

impl ToInt for ConstInt<35> {
    type Output = B1<B1<B0<B0<B0<P1>>>>>;
}

impl ToInt for ConstInt<36> {
    type Output = B0<B0<B1<B0<B0<P1>>>>>;
}

impl ToInt for ConstInt<37> {
    type Output = B1<B0<B1<B0<B0<P1>>>>>;
}

impl ToInt for ConstInt<38> {
    type Output = B0<B1<B1<B0<B0<P1>>>>>;
}

impl ToInt for ConstInt<39> {
    type Output = B1<B1<B1<B0<B0<P1>>>>>;
}

impl ToInt for ConstInt<40> {
    type Output = B0<B0<B0<B1<B0<P1>>>>>;
}

impl ToInt for ConstInt<41> {
    type Output = B1<B0<B0<B1<B0<P1>>>>>;
}

impl ToInt for ConstInt<42> {
    type Output = B0<B1<B0<B1<B0<P1>>>>>;
}

impl ToInt for ConstInt<43> {
    type Output = B1<B1<B0<B1<B0<P1>>>>>;
}

impl ToInt for ConstInt<44> {
    type Output = B0<B0<B1<B1<B0<P1>>>>>;
}

impl ToInt for ConstInt<45> {
    type Output = B1<B0<B1<B1<B0<P1>>>>>;
}

impl ToInt for ConstInt<46> {
    type Output = B0<B1<B1<B1<B0<P1>>>>>;
}

impl ToInt for ConstInt<47> {
    type Output = B1<B1<B1<B1<B0<P1>>>>>;
}

impl ToInt for ConstInt<48> {
    type Output = B0<B0<B0<B0<B1<P1>>>>>;
}

impl ToInt for ConstInt<49> {
    type Output = B1<B0<B0<B0<B1<P1>>>>>;
}

impl ToInt for ConstInt<50> {
    type Output = B0<B1<B0<B0<B1<P1>>>>>;
}

impl ToInt for ConstInt<51> {
    type Output = B1<B1<B0<B0<B1<P1>>>>>;
}

impl ToInt for ConstInt<52> {
    type Output = B0<B0<B1<B0<B1<P1>>>>>;
}

impl ToInt for ConstInt<53> {
    type Output = B1<B0<B1<B0<B1<P1>>>>>;
}

impl ToInt for ConstInt<54> {
    type Output = B0<B1<B1<B0<B1<P1>>>>>;
}

impl ToInt for ConstInt<55> {
    type Output = B1<B1<B1<B0<B1<P1>>>>>;
}

impl ToInt for ConstInt<56> {
    type Output = B0<B0<B0<B1<B1<P1>>>>>;
}

impl ToInt for ConstInt<57> {
    type Output = B1<B0<B0<B1<B1<P1>>>>>;
}

impl ToInt for ConstInt<58> {
    type Output = B0<B1<B0<B1<B1<P1>>>>>;
}

impl ToInt for ConstInt<59> {
    type Output = B1<B1<B0<B1<B1<P1>>>>>;
}

impl ToInt for ConstInt<60> {
    type Output = B0<B0<B1<B1<B1<P1>>>>>;
}

impl ToInt for ConstInt<61> {
    type Output = B1<B0<B1<B1<B1<P1>>>>>;
}

impl ToInt for ConstInt<62> {
    type Output = B0<B1<B1<B1<B1<P1>>>>>;
}

impl ToInt for ConstInt<63> {
    type Output = B1<B1<B1<B1<B1<P1>>>>>;
}

// Special case for 64 (requires 7 bits)
impl ToInt for ConstInt<64> {
    type Output = B0<B0<B0<B0<B0<B0<P1>>>>>>;
}

// =============================================
//  Negative Numbers Representation / 负数表示
// =============================================
/* 
 * Representation Rules:
 * 1. MSB innermost, LSB outermost
 * 2. Innermost is N1 (negative sign)
 * 3. Two's complement representation
 *
 * 表示规则：
 * 1. 最高有效位在内，最低有效位在外
 * 2. 最内层为 N1 (负号)
 * 3. 补码表示
 */

// Group -1: Single number (-1)
impl ToInt for ConstInt<-1> {
    type Output = N1;
}

// Group -2: Single number (-2)
impl ToInt for ConstInt<-2> {
    type Output = B0<N1>;
}

// Group -3: 3-bit numbers (-3 to -4)
impl ToInt for ConstInt<-3> {
    type Output = B1<B0<N1>>;
}

impl ToInt for ConstInt<-4> {
    type Output = B0<B0<N1>>;
}

// Group -4: 4-bit numbers (-5 to -8)
impl ToInt for ConstInt<-5> {
    type Output = B1<B1<B0<N1>>>;
}

impl ToInt for ConstInt<-6> {
    type Output = B0<B1<B0<N1>>>;
}

impl ToInt for ConstInt<-7> {
    type Output = B1<B0<B0<N1>>>;
}

impl ToInt for ConstInt<-8> {
    type Output = B0<B0<B0<N1>>>;
}

// Group -5: 5-bit numbers (-9 to -16)
impl ToInt for ConstInt<-9> {
    type Output = B1<B1<B1<B0<N1>>>>;
}

impl ToInt for ConstInt<-10> {
    type Output = B0<B1<B1<B0<N1>>>>;
}

impl ToInt for ConstInt<-11> {
    type Output = B1<B0<B1<B0<N1>>>>;
}

impl ToInt for ConstInt<-12> {
    type Output = B0<B0<B1<B0<N1>>>>;
}

impl ToInt for ConstInt<-13> {
    type Output = B1<B1<B0<B0<N1>>>>;
}

impl ToInt for ConstInt<-14> {
    type Output = B0<B1<B0<B0<N1>>>>;
}

impl ToInt for ConstInt<-15> {
    type Output = B1<B0<B0<B0<N1>>>>;
}

impl ToInt for ConstInt<-16> {
    type Output = B0<B0<B0<B0<N1>>>>;
}

// Group -6: 6-bit numbers (-17 to -64)
impl ToInt for ConstInt<-17> {
    type Output = B1<B1<B1<B1<B0<N1>>>>>;
}

impl ToInt for ConstInt<-18> {
    type Output = B0<B1<B1<B1<B0<N1>>>>>;
}

impl ToInt for ConstInt<-19> {
    type Output = B1<B0<B1<B1<B0<N1>>>>>;
}

impl ToInt for ConstInt<-20> {
    type Output = B0<B0<B1<B1<B0<N1>>>>>;
}

impl ToInt for ConstInt<-21> {
    type Output = B1<B1<B0<B1<B0<N1>>>>>;
}

impl ToInt for ConstInt<-22> {
    type Output = B0<B1<B0<B1<B0<N1>>>>>;
}

impl ToInt for ConstInt<-23> {
    type Output = B1<B0<B0<B1<B0<N1>>>>>;
}

impl ToInt for ConstInt<-24> {
    type Output = B0<B0<B0<B1<B0<N1>>>>>;
}

impl ToInt for ConstInt<-25> {
    type Output = B1<B1<B1<B0<B0<N1>>>>>;
}

impl ToInt for ConstInt<-26> {
    type Output = B0<B1<B1<B0<B0<N1>>>>>;
}

impl ToInt for ConstInt<-27> {
    type Output = B1<B0<B1<B0<B0<N1>>>>>;
}

impl ToInt for ConstInt<-28> {
    type Output = B0<B0<B1<B0<B0<N1>>>>>;
}

impl ToInt for ConstInt<-29> {
    type Output = B1<B1<B0<B0<B0<N1>>>>>;
}

impl ToInt for ConstInt<-30> {
    type Output = B0<B1<B0<B0<B0<N1>>>>>;
}

impl ToInt for ConstInt<-31> {
    type Output = B1<B0<B0<B0<B0<N1>>>>>;
}

impl ToInt for ConstInt<-32> {
    type Output = B0<B0<B0<B0<B0<N1>>>>>;
}

// Group -7: 7-bit numbers (-33 to -？？)
impl ToInt for ConstInt<-33> {
    type Output = B1<B1<B1<B1<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-34> {
    type Output = B0<B1<B1<B1<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-35> {
    type Output = B1<B0<B1<B1<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-36> {
    type Output = B0<B0<B1<B1<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-37> {
    type Output = B1<B1<B0<B1<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-38> {
    type Output = B0<B1<B0<B1<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-39> {
    type Output = B1<B0<B0<B1<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-40> {
    type Output = B0<B0<B0<B1<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-41> {
    type Output = B1<B1<B1<B0<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-42> {
    type Output = B0<B1<B1<B0<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-43> {
    type Output = B1<B0<B1<B0<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-44> {
    type Output = B0<B0<B1<B0<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-45> {
    type Output = B1<B1<B0<B0<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-46> {
    type Output = B0<B1<B0<B0<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-47> {
    type Output = B1<B0<B0<B0<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-48> {
    type Output = B0<B0<B0<B0<B1<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-49> {
    type Output = B1<B1<B1<B1<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-50> {
    type Output = B0<B1<B1<B1<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-51> {
    type Output = B1<B0<B1<B1<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-52> {
    type Output = B0<B0<B1<B1<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-53> {
    type Output = B1<B1<B0<B1<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-54> {
    type Output = B0<B1<B0<B1<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-55> {
    type Output = B1<B0<B0<B1<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-56> {
    type Output = B0<B0<B0<B1<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-57> {
    type Output = B1<B1<B1<B0<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-58> {
    type Output = B0<B1<B1<B0<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-59> {
    type Output = B1<B0<B1<B0<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-60> {
    type Output = B0<B0<B1<B0<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-61> {
    type Output = B1<B1<B0<B0<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-62> {
    type Output = B0<B1<B0<B0<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-63> {
    type Output = B1<B0<B0<B0<B0<B0<N1>>>>>>;
}

impl ToInt for ConstInt<-64> {
    type Output = B0<B0<B0<B0<B0<B0<N1>>>>>>;
}