# Quantity - 量纲安全的SI单位库 / Type-Safe SI Units Library

[![Crates.io](https://img.shields.io/crates/v/quantity)](https://crates.io/crates/quantity)
[![文档|Documentation](https://docs.rs/quantity/badge.svg)](https://docs.rs/quantity)
[![构建状态|CI Status](https://github.com/liuyuan77/quantity/actions/workflows/ci.yml/badge.svg)](https://github.com/liuyuan77/quantity/actions)

```toml
[package]
name = "quantity"  # 必须是唯一的（全crates.io范围内）| Must be unique (across all crates.io)
version = "0.0.1"  # 遵循语义化版本控制 | Follows semantic versioning
edition = "2024"
description = "A type-safe library for handling physical quantities with units."
license = "MIT OR Apache-2.0"  # 必须使用SPDX标识符 | Must use SPDX identifiers
authors = ["liuyuan <375798574@qq.com>"]
repository = "https://github.com/liuyuan77/quantity"
documentation = "https://docs.rs/quantity"
keywords = ["quantity", "SI", "units", "physics", "dimensional"]  # 最多5个 | Max 5 keywords

[dependencies]
# 本库目前没有第三方依赖 | Currently no third-party dependencies

```