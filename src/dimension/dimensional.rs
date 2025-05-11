//! Trait definitions for Dimension
//! 物理单位的Trait定义

use typenum::Integer;
use super::Dimension;

/// Trait marking valid Dimension types
/// 标记有效单位类型的Trait
pub trait Dimensional: Sized {
}

impl<M: Integer, KG: Integer, S: Integer, A: Integer, K: Integer, MOL: Integer, CD: Integer>
    Dimensional for Dimension<M, KG, S, A, K, MOL, CD>{
}