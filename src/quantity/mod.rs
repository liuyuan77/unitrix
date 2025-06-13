//! Physical Quantity Calculation Library
//! 物理量计算库
//!
//! Provides type-safe physical quantity calculations with automatic unit deduction and SI prefix conversion.
//! 提供类型安全的物理量计算功能，支持单位自动推导和SI前缀转换

// use core::marker::PhantomData;
// use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};
// use crate::variable::{Numeric, Scalar, Var};

mod unitary;
pub use unitary::Unitary;

mod prefix;
pub use prefix::*;

mod dimension;
pub use dimension::*;

mod si;
pub use si::*;

mod ratio;
pub use ratio::*;

mod unit;
pub use unit::*;

//mod alias;
//pub use alias::*;