//! Unitrix: A dimensional analysis library with const-powered matrix transformations.
//! Unitrix: 基于常量化矩阵的带量纲分析计算库
//!
//! Provides compile-time unit checking and 2D geometric operations through:
//! 通过以下方式提供编译期单位检查和2D几何运算：
//! - Zero-cost physical quantity arithmetic
//!   - 零开销的物理量算术运算
//! - Const-optimized matrix operations
//!   - 常量优化的矩阵运算
//! - `no_std` compatible architecture
//!   - 兼容`no_std`的架构设计
//!
//! # Core Features
//! # 核心特性
//! - **Type-safe units** with dimensional analysis
//!   - **类型安全单位**（带量纲分析）
//! - **Const-native** calculations
//!   - **原生常量**计算
//! - **Unit-preserving** 2D transforms
//!   - **保持单位**的2D变换
//!
//! # Modules
//! # 模块说明
//! - [`quantity`] - Physical quantities with unit tracking
//!   - [`quantity`] - 带单位追踪的物理量
//! - [`matrix`] - Unit-aware 2D transformations
//!   - [`matrix`] - 单位感知的2D变换
//! - [`number`] - Numeric type foundations
//!   - [`number`] - 数值类型基础

#![no_std] // 不依赖标准库
#![forbid(unsafe_code)] // 禁止不安全代码，保证内存安全
// #![deny(missing_docs)]  // 无条件使用注释文档
#![doc(html_root_url = "https://docs.rs/unitrix/0.0.5")]  // 文档根URL
#![cfg_attr(docsrs, feature(doc_auto_cfg))]  // 文档生成时的特性配置

/// Sealed traits and internal implementation details
/// 密封trait和内部实现细节
#[allow(missing_docs)]  // 显式豁免内部模块
pub(crate) mod sealed;

/// Fundamental numeric types and operations
/// 基础数值类型和运算
pub mod number;

/// Physical quantity implementation with unit tracking
/// 带单位追踪的物理量实现
pub mod quantity;

// Unit-preserving 2D transformation matrices
/// 保持单位的2D变换矩阵
pub mod matrix;