// units/unitary.rs
use core::fmt;

/// 所有单位的根特质（直接返回数组）
pub trait Unitary {
}

/// 可显示符号的特质（可选实现）
pub trait Symbol: Unitary {
    fn symbol() -> &'static str;
}

pub struct DisplayWrapper<T: Symbol>(pub T);

impl<T: Symbol> fmt::Display for DisplayWrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", T::symbol())
    }
}