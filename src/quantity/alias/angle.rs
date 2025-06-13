/// Angle units / 角度单位
/// 参考GB 3102.1 1-1 要求

use core::ops::{Add, Sub, AddAssign, SubAssign};

use Crate::quantity::{Dimension, Unit, Si, Z0, P1, N1, Integer, NonZero, NoPrefix, Milli, Micro};

// ========== Angle Type Definitions ==========
// ========== 角度类型定义 ==========

/// Generic angle in radians with SI prefixes / 带SI前缀的通用弧度角度
/// 
/// # Type Parameters / 类型参数
/// - `V`: Value type (f32, f64, etc.) / 值类型 (如 f32, f64 等)
/// - `Pr`: SI prefix type (NoPrefix, Milli, Micro, etc.) / SI前缀类型 (无前缀, 毫, 微等)
pub type Angle<V = P1, Pr = NoPrefix> = Si<V, Dimension<Z0, Z0, Z0, Z0, Z0, Z0, Z0>, Pr>;

/// Radians / 弧度
/// 
/// # Type Parameters / 类型参数
/// - `V`: Value type (f32, f64, etc.) / 值类型 (如 f32, f64 等)
pub type Radians<V = f64> = Angle<V, NoPrefix>;

/// Milliradians / 毫弧度
/// 
/// # Type Parameters / 类型参数
/// - `V`: Value type (f32, f64, etc.) / 值类型 (如 f32, f64 等)
pub type Milliradians<V> = Angle<V, Milli>;

/// Microradians / 微弧度
/// 
/// # Type Parameters / 类型参数
/// - `V`: Value type (f32, f64, etc.) / 值类型 (如 f32, f64 等)
pub type Microradians<V> = Angle<V, Micro>;

/// Degrees angle / 度 必须改？？？？？？？？？？？？？？？？？？？？？？？？？？？？？？？？？？？？？、
/// 
/// # Type Parameters / 类型参数
/// - `V`: Value type that implements Copy (f32, f64, etc.) / 实现了Copy的值类型 (如 f32, f64 等)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Degrees<V: Copy>(pub V);

impl<V: Copy> Degrees<V> {
    /// Create new angle in degrees / 创建新的角度值(度)
    /// 
    /// # Arguments / 参数
    /// - `degrees`: Angle value in degrees / 角度值(度)
    /// 
    /// # Returns / 返回值
    /// New Degrees instance / 新的度实例
    pub fn new(degrees: V) -> Self {
        Self(degrees)
    }

    /// Extract inner value / 解构获取内部值
    /// 
    /// # Returns / 返回值
    /// The contained angle value / 包含的角度值
    pub fn into_inner(self) -> V {
        self.0
    }
}

/// Gradians (gons) angle / 哥恩(百分度)
/// 
/// # Type Parameters / 类型参数
/// - `V`: Value type that implements Copy (f32, f64, etc.) / 实现了Copy的值类型 (如 f32, f64 等)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gradians<V: Copy>(pub V);

impl<V: Copy> Gradians<V> {
    /// Create new angle in gradians / 创建新的角度值(哥恩)
    /// 
    /// # Arguments / 参数
    /// - `gradians`: Angle value in gradians / 角度值(哥恩)
    /// 
    /// # Returns / 返回值
    /// New Gradians instance / 新的哥恩实例
    pub fn new(gradians: V) -> Self {
        Self(gradians)
    }

    /// Extract inner value / 解构获取内部值
    /// 
    /// # Returns / 返回值
    /// The contained angle value / 包含的角度值
    pub fn into_inner(self) -> V {
        self.0
    }
}

// ========== Conversion Implementations ==========
// ========== 转换实现 ==========

// Degrees <-> Radians conversions
impl<V: Copy + From<f64> + Into<f64>> From<Degrees<V>> for Radians<V> {
    /// Convert degrees to radians / 度转弧度
    /// 
    /// Formula: rad = deg * (π/180) / 公式: rad = deg * (π/180)
    fn from(deg: Degrees<V>) -> Self {
        Radians::new(V::from(deg.0.into() * std::f64::consts::PI / 180.0))
    }
}

impl<V: Copy + From<f64> + Into<f64>> From<Radians<V>> for Degrees<V> {
    /// Convert radians to degrees / 弧度转度
    /// 
    /// Formula: deg = rad * (180/π) / 公式: deg = rad * (180/π)
    fn from(rad: Radians<V>) -> Self {
        Degrees::new(V::from(rad.into_inner().into() * 180.0 / std::f64::consts::PI))
    }
}

// Gradians <-> Radians conversions
impl<V: Copy + From<f64> + Into<f64>> From<Gradians<V>> for Radians<V> {
    /// Convert gradians to radians / 哥恩转弧度
    /// 
    /// Formula: rad = gon * (π/200) / 公式: rad = gon * (π/200)
    fn from(gon: Gradians<V>) -> Self {
        Radians::new(V::from(gon.0.into() * std::f64::consts::PI / 200.0))
    }
}

impl<V: Copy + From<f64> + Into<f64>> From<Radians<V>> for Gradians<V> {
    /// Convert radians to gradians / 弧度转哥恩
    /// 
    /// Formula: gon = rad * (200/π) / 公式: gon = rad * (200/π)
    fn from(rad: Radians<V>) -> Self {
        Gradians::new(V::from(rad.into_inner().into() * 200.0 / std::f64::consts::PI))
    }
}

// Degrees <-> Gradians conversions
impl<V: Copy + From<f64> + Into<f64>> From<Degrees<V>> for Gradians<V> {
    /// Convert degrees to gradians / 度转哥恩
    /// 
    /// Formula: gon = deg * (10/9) / 公式: gon = deg * (10/9)
    fn from(deg: Degrees<V>) -> Self {
        Gradians::new(V::from(deg.0.into() * 10.0 / 9.0))
    }
}

impl<V: Copy + From<f64> + Into<f64>> From<Gradians<V>> for Degrees<V> {
    /// Convert gradians to degrees / 哥恩转度
    /// 
    /// Formula: deg = gon * (9/10) / 公式: deg = gon * (9/10)
    fn from(gon: Gradians<V>) -> Self {
        Degrees::new(V::from(gon.0.into() * 9.0 / 10.0))
    }
}

// Milliradians <-> Radians conversions
impl<V: Copy + From<f64> + Into<f64>> From<Milliradians<V>> for Radians<V> {
    /// Convert milliradians to radians / 毫弧度转弧度
    /// 
    /// Formula: rad = mrad / 1000 / 公式: rad = mrad / 1000
    fn from(mrad: Milliradians<V>) -> Self {
        mrad.convert_to::<NoPrefix>()
    }
}

impl<V: Copy + From<f64> + Into<f64>> From<Radians<V>> for Milliradians<V> {
    /// Convert radians to milliradians / 弧度转毫弧度
    /// 
    /// Formula: mrad = rad * 1000 / 公式: mrad = rad * 1000
    fn from(rad: Radians<V>) -> Self {
        rad.convert_to::<Milli>()
    }
}

// Microradians <-> Radians conversions
impl<V: Copy + From<f64> + Into<f64>> From<Microradians<V>> for Radians<V> {
    /// Convert microradians to radians / 微弧度转弧度
    /// 
    /// Formula: rad = μrad / 1_000_000 / 公式: rad = μrad / 1_000_000
    fn from(μrad: Microradians<V>) -> Self {
        μrad.convert_to::<NoPrefix>()
    }
}

impl<V: Copy + From<f64> + Into<f64>> From<Radians<V>> for Microradians<V> {
    /// Convert radians to microradians / 弧度转微弧度
    /// 
    /// Formula: μrad = rad * 1_000_000 / 公式: μrad = rad * 1_000_000
    fn from(rad: Radians<V>) -> Self {
        rad.convert_to::<Micro>()
    }
}