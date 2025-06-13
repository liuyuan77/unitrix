//! 类型级整数表示模块
//!
//! 本模块提供了一种在类型系统中表示整数的方法，使用二进制补码形式。
//! 正数和负数有不同的表示规则，详见下面的文档说明。
mod constant;
pub(super) use constant::*;

mod variable;
pub use variable::*;

pub mod not;

pub mod neg;

mod add1;
pub use add1::*;

pub mod sub1;
pub use sub1::*;

mod add;
pub use add::*;

mod sub;
pub use sub::*;

pub mod mul;

pub mod div;

pub(super) mod shl;

pub(super) mod shr;

mod standardization;
pub(crate) use standardization::*;

mod to_int;
pub use to_int::Const;