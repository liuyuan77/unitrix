//! 密封模式（Sealed Pattern）实现模块
//!
//! 此模块提供了使用密封模式限制 trait 实现的工具，
//! 确保只有当前 crate 内定义的类型能够实现特定 trait，
//! 从而保证 API 的稳定性。

/// 密封 trait，用于限制 trait 的实现范围
///
/// 这个 trait 是私有的，外部 crate 无法实现它。
/// 通过将它作为其他 trait 的 supertrait，
/// 可以确保只有当前 crate 内的类型能够实现那些 trait。
///
/// # 设计模式参考
/// 这是 Rust 中常用的"密封模式"(Sealed Pattern)的实现，
/// 用于控制 trait 实现的可见性范围。
pub trait Sealed {}