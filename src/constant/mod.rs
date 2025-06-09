//! 类型级整数表示模块
//!
//! 本模块提供了一种在类型系统中表示整数的方法，使用二进制补码形式。
//! 正数和负数有不同的表示规则，详见下面的文档说明。
mod basic;
pub(super) use basic::*;

pub mod not;//没有共享内容

mod neg;
pub use neg::*;

mod add1;
pub use add1::*;

pub mod sub1;
pub use sub1::*;

mod add;
pub use add::*;

mod sub;
pub use sub::*;

mod mul;
pub use mul::*;

mod div;
pub use mul::*;

pub(super) mod shl;
//pub use shl::*;

pub(super) mod shr;

pub(super) mod standardization;

/// 类型级整数别名
/// 
/// 将编译时常量 N 转换为对应的类型表示
/// 例如：`Const<42>` 会展开为 `B0<B1<B0<B1<B0<Z0>>>>>`
pub type Const<const N: i32> = <Constant<N> as ToInt>::Output;

/// 用于在泛型上下文中表示整数的包装类型
pub struct Constant<const N: i32>;

/// 将整数常量转换为类型表示的 trait
pub trait ToInt {
    /// 对应的类型表示
    type Output;
}

/* ========== 正数表示规则 ==========
 * 1. 最高有效位(MSB)在内，最低位(LSB)在外，依次嵌套
 * 2. 最内层必须是 Z0，表示正号，也表示本位和更高位是0
 * 3. 数值部分使用 B0/B1 表示二进制位
 * 
 
 * 例如 +5 (101):
 * 
 *   Z0 (符号位和终止符,表示本位为0，更高位也是0)
 *   B1 <- MSB
 *   B0
 *   B1 <- LSB
 * 表示为: B1<B0<P1>>
 */
impl ToInt for Constant<0> {
    type Output = Z0;  // 0 的特殊表示
}

impl ToInt for Constant<1> {
    type Output = P1;
}

impl ToInt for Constant<2> {
    type Output = B0<P1>;
}

impl ToInt for Constant<3> {
    type Output = P1;
}

impl ToInt for Constant<4> {
    type Output = B0<B0<Z0>>;
}

impl ToInt for Constant<5> {
    type Output = B1<B0<Z0>>;
}

impl ToInt for Constant<6> {
    type Output = B0<P1>;
}

impl ToInt for Constant<7> {
    type Output = B1<P1>;
}

impl ToInt for Constant<8> {
    type Output = B0<B0<B0<Z0>>>;
}

impl ToInt for Constant<9> {
    type Output = B1<B0<B0<Z0>>>;
}

impl ToInt for Constant<10> {
    type Output = B0<B1<B0<Z0>>>;
}

impl ToInt for Constant<11> {
    type Output = B1<B1<B0<Z0>>>;
}

impl ToInt for Constant<12> {
    type Output = B0<B0<P1>>;
}

impl ToInt for Constant<13> {
    type Output = B1<B0<P1>>;
}

impl ToInt for Constant<14> {
    type Output = B0<B1<P1>>;
}

impl ToInt for Constant<15> {
    type Output = B1<B1<P1>>;
}

impl ToInt for Constant<16> {
    type Output = B0<B0<B0<B0<Z0>>>>;
}

impl ToInt for Constant<17> {
    type Output = B1<B0<B0<B0<Z0>>>>;
}

impl ToInt for Constant<18> {
    type Output = B0<B1<B0<B0<Z0>>>>;
}

impl ToInt for Constant<19> {
    type Output = B1<B1<B0<B0<Z0>>>>;
}

impl ToInt for Constant<20> {
    type Output = B0<B0<B1<B0<Z0>>>>;
}

impl ToInt for Constant<21> {
    type Output = B1<B0<B1<B0<Z0>>>>;
}

impl ToInt for Constant<22> {
    type Output = B0<B1<B1<B0<Z0>>>>;
}

impl ToInt for Constant<23> {
    type Output = B1<B1<B1<B0<Z0>>>>;
}

impl ToInt for Constant<24> {
    type Output = B0<B0<B0<P1>>>;
}

impl ToInt for Constant<25> {
    type Output = B1<B0<B0<P1>>>;
}

impl ToInt for Constant<26> {
    type Output = B0<B1<B0<P1>>>;
}

impl ToInt for Constant<27> {
    type Output = B1<B1<B0<P1>>>;
}

impl ToInt for Constant<28> {
    type Output = B0<B0<B1<P1>>>;
}

impl ToInt for Constant<29> {
    type Output = B1<B0<B1<P1>>>;
}

impl ToInt for Constant<30> {
    type Output = B0<B1<B1<P1>>>;
}

impl ToInt for Constant<31> {
    type Output = B1<B1<B1<P1>>>;
}

impl ToInt for Constant<32> {
    type Output = B0<B0<B0<B0<B0<Z0>>>>>;
}

impl ToInt for Constant<33> {
    type Output = B1<B0<B0<B0<B0<Z0>>>>>;
}

impl ToInt for Constant<34> {
    type Output = B0<B1<B0<B0<B0<Z0>>>>>;
}

impl ToInt for Constant<35> {
    type Output = B1<B1<B0<B0<B0<Z0>>>>>;
}

impl ToInt for Constant<36> {
    type Output = B0<B0<B1<B0<B0<Z0>>>>>;
}

impl ToInt for Constant<37> {
    type Output = B1<B0<B1<B0<B0<Z0>>>>>;
}

impl ToInt for Constant<38> {
    type Output = B0<B1<B1<B0<B0<Z0>>>>>;
}

impl ToInt for Constant<39> {
    type Output = B1<B1<B1<B0<B0<Z0>>>>>;
}

impl ToInt for Constant<40> {
    type Output = B0<B0<B0<B1<B0<Z0>>>>>;
}

impl ToInt for Constant<41> {
    type Output = B1<B0<B0<B1<B0<Z0>>>>>;
}

impl ToInt for Constant<42> {
    type Output = B0<B1<B0<B1<B0<Z0>>>>>;
}

impl ToInt for Constant<43> {
    type Output = B1<B1<B0<B1<B0<Z0>>>>>;
}

impl ToInt for Constant<44> {
    type Output = B0<B0<B1<B1<B0<Z0>>>>>;
}

impl ToInt for Constant<45> {
    type Output = B1<B0<B1<B1<B0<Z0>>>>>;
}

impl ToInt for Constant<46> {
    type Output = B0<B1<B1<B1<B0<Z0>>>>>;
}

impl ToInt for Constant<47> {
    type Output = B1<B1<B1<B1<B0<Z0>>>>>;
}

impl ToInt for Constant<48> {
    type Output = B0<B0<B0<B0<P1>>>>;
}

impl ToInt for Constant<49> {
    type Output = B1<B0<B0<B0<P1>>>>;
}

impl ToInt for Constant<50> {
    type Output = B0<B1<B0<B0<P1>>>>;
}

impl ToInt for Constant<51> {
    type Output = B1<B1<B0<B0<P1>>>>;
}

impl ToInt for Constant<52> {
    type Output = B0<B0<B1<B0<P1>>>>;
}

impl ToInt for Constant<53> {
    type Output = B1<B0<B1<B0<P1>>>>;
}

impl ToInt for Constant<54> {
    type Output = B0<B1<B1<B0<P1>>>>;
}

impl ToInt for Constant<55> {
    type Output = B1<B1<B1<B0<P1>>>>;
}

impl ToInt for Constant<56> {
    type Output = B0<B0<B0<B1<P1>>>>;
}

impl ToInt for Constant<57> {
    type Output = B1<B0<B0<B1<P1>>>>;
}

impl ToInt for Constant<58> {
    type Output = B0<B1<B0<B1<P1>>>>;
}

impl ToInt for Constant<59> {
    type Output = B1<B1<B0<B1<P1>>>>;
}

impl ToInt for Constant<60> {
    type Output = B0<B0<B1<B1<P1>>>>;
}

impl ToInt for Constant<61> {
    type Output = B1<B0<B1<B1<P1>>>>;
}

impl ToInt for Constant<62> {
    type Output = B0<B1<B1<B1<P1>>>>;
}

impl ToInt for Constant<63> {
    type Output = B1<B1<B1<B1<P1>>>>;
}

impl ToInt for Constant<64> {
    type Output = B0<B0<B0<B0<B0<B0<Z0>>>>>>;
}

/* ========== 负数表示规则 ==========
 * 1. 最高有效位(MSB)在内，最低位(LSB)在外，依次嵌套
 * 2. 最内层必须是 N1，表示负数的负号位，也表示本位及更高位为1
 * 3. 数值部分使用补码表示，使用 B0/B1 表示二进制位
 * 
 * 例如 -5 (补码 1011):
 *   N1 <- MSB (符号位和终止符，也表示本位为1，更高位都是1)
 *   B0
 *   B1
 *   B1 <- LSB
 * 表示为: B1<B1<B0<N1>>>
 */
impl ToInt for Constant<-1> {
    type Output = N1;
}

impl ToInt for Constant<-2> {
    type Output = B0<N1>;
}

impl ToInt for Constant<-3> {
    type Output = B1<B0<N1>>;
}

impl ToInt for Constant<-4> {
    type Output = B0<B0<N1>>;
}

impl ToInt for Constant<-5> {
    type Output = B1<B1<B0<N1>>>;
}

impl ToInt for Constant<-6> {
    type Output = B0<B1<B0<N1>>>;
}

impl ToInt for Constant<-7> {
    type Output = B1<B0<B0<N1>>>;
}

impl ToInt for Constant<-8> {
    type Output = B0<B0<B0<N1>>>;
}

impl ToInt for Constant<-9> {
    type Output = B1<B1<B1<B0<N1>>>>;
}

impl ToInt for Constant<-10> {
    type Output = B0<B1<B1<B0<N1>>>>;
}

impl ToInt for Constant<-11> {
    type Output = B1<B0<B1<B0<N1>>>>;
}

impl ToInt for Constant<-12> {
    type Output = B0<B0<B1<B0<N1>>>>;
}

impl ToInt for Constant<-13> {
    type Output = B1<B1<B0<B0<N1>>>>;
}

impl ToInt for Constant<-14> {
    type Output = B0<B1<B0<B0<N1>>>>;
}

impl ToInt for Constant<-15> {
    type Output = B1<B0<B0<B0<N1>>>>;
}

impl ToInt for Constant<-16> {
    type Output = B0<B0<B0<B0<N1>>>>;
}

impl ToInt for Constant<-17> {
    type Output = B1<B1<B1<B1<B0<N1>>>>>;
}

impl ToInt for Constant<-18> {
    type Output = B0<B1<B1<B1<B0<N1>>>>>;
}

impl ToInt for Constant<-19> {
    type Output = B1<B0<B1<B1<B0<N1>>>>>;
}

impl ToInt for Constant<-20> {
    type Output = B0<B0<B1<B1<B0<N1>>>>>;
}

impl ToInt for Constant<-21> {
    type Output = B1<B1<B0<B1<B0<N1>>>>>;
}

impl ToInt for Constant<-22> {
    type Output = B0<B1<B0<B1<B0<N1>>>>>;
}

impl ToInt for Constant<-23> {
    type Output = B1<B0<B0<B1<B0<N1>>>>>;
}

impl ToInt for Constant<-24> {
    type Output = B0<B0<B0<B1<B0<N1>>>>>;
}

impl ToInt for Constant<-25> {
    type Output = B1<B1<B1<B0<B0<N1>>>>>;
}

impl ToInt for Constant<-26> {
    type Output = B0<B1<B1<B0<B0<N1>>>>>;
}

impl ToInt for Constant<-27> {
    type Output = B1<B0<B1<B0<B0<N1>>>>>;
}

impl ToInt for Constant<-28> {
    type Output = B0<B0<B1<B0<B0<N1>>>>>;
}

impl ToInt for Constant<-29> {
    type Output = B1<B1<B0<B0<B0<N1>>>>>;
}

impl ToInt for Constant<-30> {
    type Output = B0<B1<B0<B0<B0<N1>>>>>;
}

impl ToInt for Constant<-31> {
    type Output = B1<B0<B0<B0<B0<N1>>>>>;
}

impl ToInt for Constant<-32> {
    type Output = B0<B0<B0<B0<B0<N1>>>>>;
}

impl ToInt for Constant<-33> {
    type Output = B1<B1<B1<B1<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-34> {
    type Output = B0<B1<B1<B1<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-35> {
    type Output = B1<B0<B1<B1<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-36> {
    type Output = B0<B0<B1<B1<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-37> {
    type Output = B1<B1<B0<B1<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-38> {
    type Output = B0<B1<B0<B1<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-39> {
    type Output = B1<B0<B0<B1<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-40> {
    type Output = B0<B0<B0<B1<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-41> {
    type Output = B1<B1<B1<B0<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-42> {
    type Output = B0<B1<B1<B0<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-43> {
    type Output = B1<B0<B1<B0<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-44> {
    type Output = B0<B0<B1<B0<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-45> {
    type Output = B1<B1<B0<B0<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-46> {
    type Output = B0<B1<B0<B0<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-47> {
    type Output = B1<B0<B0<B0<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-48> {
    type Output = B0<B0<B0<B0<B1<B0<N1>>>>>>;
}

impl ToInt for Constant<-49> {
    type Output = B1<B1<B1<B1<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-50> {
    type Output = B0<B1<B1<B1<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-51> {
    type Output = B1<B0<B1<B1<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-52> {
    type Output = B0<B0<B1<B1<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-53> {
    type Output = B1<B1<B0<B1<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-54> {
    type Output = B0<B1<B0<B1<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-55> {
    type Output = B1<B0<B0<B1<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-56> {
    type Output = B0<B0<B0<B1<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-57> {
    type Output = B1<B1<B1<B0<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-58> {
    type Output = B0<B1<B1<B0<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-59> {
    type Output = B1<B0<B1<B0<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-60> {
    type Output = B0<B0<B1<B0<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-61> {
    type Output = B1<B1<B0<B0<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-62> {
    type Output = B0<B1<B0<B0<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-63> {
    type Output = B1<B0<B0<B0<B0<B0<N1>>>>>>;
}

impl ToInt for Constant<-64> {
    type Output = B0<B0<B0<B0<B0<B0<N1>>>>>>;
}

/*
pub type AddOne<A> = <A as Add1>::Output;

pub type SubOne<I> = <I as Sub1>::Output;

/// 除法运算的类型别名：`Quot<A, B> = <A as Div<B>>::Output`
pub type Quot<A, B> = <A as Div<B>>::Output; */
