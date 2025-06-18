use core::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign};
use crate::number::Var;


/// 定义 VarType trait，用于物理量取值
/// 在 Primitive 基础上增加了 MulAssign 约束
pub trait VarType:
    Neg<Output = Self> +
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    AddAssign +
    SubAssign +
    MulAssign +
    Copy +
    Clone +
    Default +
    From<i32> +
    Sized +
    'static
{}

// 为 Var 类型实现 VarType trait
// impl VarType for Var<i32> {}
impl VarType for Var<i64> {}
//impl VarType for Var<f32> {}
impl VarType for Var<f64> {}