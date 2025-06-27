//! 类型级整数表示模块
//!
//! 本模块提供了一种在类型系统中表示整数的方法，使用二进制补码形式。
//! 正数和负数有不同的表示规则，详见下面的文档说明。
mod bit;
pub use bit::*;

mod cmp;
pub use cmp::*;

mod types;
pub(super) use types::*;

mod method;
pub use method::*;

// 基本体及特质
mod traits;
pub use traits::*;

pub mod special;
// pub mod not_a_number;
// pub mod positive_infinity;
pub mod z0;



pub mod from;


pub mod variable;

mod to_int; //常量转变量
pub use to_int::Const;

mod standardization;
pub use standardization::*;

// 自定义加一减一运算
mod add1;
pub use add1::*;

pub mod sub1;
pub use sub1::*;

// 算术操作符
mod add;
pub use add::*;

mod sub;
pub use sub::*;

pub mod mul;

pub mod div;//需完善
// 求余 未实现 rem

// 位操作符
pub mod bitand;
pub mod bitor;
pub mod bitxor;
pub mod shl;
pub mod shr;

// 一元运算符
pub mod not;
pub mod neg;

// 算术运算符











