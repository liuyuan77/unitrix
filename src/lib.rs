//! quantity in pure Rust

#![no_std]  // 不依赖标准库，仅使用核心功能
#![forbid(unsafe_code)]  // 禁止不安全代码，保证内存安全
// #![deny(missing_docs)]  // 无条件要求文档
#![doc(html_root_url = "https://docs.rs/physunits/0.0.3")]  // 文档根URL
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]  // 文档生成时的特性配置

pub(crate) mod sealed; //非公开密封一些 trait不能被外部标记

mod null;
pub use null::Null;

pub mod constant;

pub mod quantity;

pub mod variable;