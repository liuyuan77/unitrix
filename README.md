# PhysUnits — Type-Safe Physical Quantities for Rust

[![Crates.io](https://img.shields.io/crates/v/quantity)](https://crates.io/crates/physunits)
[![Docs.rs](https://img.shields.io/docsrs/quantity)](https://docs.rs/physunits)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
**PhysUnits** is a Rust library for type-safe physical quantities with compile-time dimensional analysis and zero-cost abstractions.

```rust
use quantity::si::{Meter, Second, Quantity};

let speed: Quantity<f64, MeterPerSecond> = 5.0.into();
let length = Quantity::<i32, Kilometer>::from(3);
// Compile-time error: speed + length ❌
```

##### Design Philosophy

###### Core Components

1. **Dimension** (Type-level exponents)

   ```rust
   Dimension<L, M, T, I, Th, N, J> // Base SI units
   ```

2. **Unit** (Conversion rules)

   ```rust
   trait Unit {
       type Dimension;
       fn to_base(value: f64) -> f64;
       const SYMBOL: &'static str;
   }
   ```

3. **Quantity** (Value container)

   ```rust
   struct Quantity<V, U: Unit> { value: V, .. }
   ```

###### Features

- 📏 **Dimensional safety** at compile time
- ⚡ **Zero runtime overhead**
- 🔢 **Dual numeric support** (integers/floats)
- 🔄 **Runtime prefix handling**

##### Usage

###### Basic Example

```rust
use quantity::si::{Kelvin, Celsius, Quantity};

let boiling: Quantity<f64, Celsius> = 100.0.into();
let kelvin = boiling.convert::<Kelvin>(); // 373.15 K
```

###### Unit Math

```rust
let force = 5.0 * kg * m / (s * s);
let energy = force * (2.0 * m); // 10 J
```

##### Comparison

| Feature         | Quantity | uom   | nalgebra |
|----------------|----------|-------|----------|
| Dim Safety     | ✅        | ✅     | ❌        |
| Integer Support| ✅        | ⚠️     | ❌        |
| Runtime Prefix | ✅        | ❌     | ❌        |

---

#### PhysUnits—— Rust 类型安全物理量库


[![Crates.io](https://img.shields.io/crates/v/quantity)](https://crates.io/crates/physunits)
[![Docs.rs](https://img.shields.io/docsrs/quantity)](https://docs.rs/physunits)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**PhysUnits** 是一个提供编译期量纲检查和零成本抽象的 Rust 物理量库。

```rust
use quantity::si::{Meter, Second, Quantity};

let speed: Quantity<f64, MeterPerSecond> = 5.0.into();
let length = Quantity::<i32, Kilometer>::from(3);
// 编译期错误: speed + length ❌
```

##### 设计理念

###### 核心组件

1. **量纲** (类型级指数)

   ```rust
   Dimension<L, M, T, I, Th, N, J> // 国际单位制基本量
   ```

2. **单位** (转换规则)

   ```rust
   trait Unit {
       type Dimension;
       fn to_base(value: f64) -> f64;
       const SYMBOL: &'static str;
   }
   ```

3. **物理量** (值容器)

   ```rust
   struct Quantity<V, U: Unit> { value: V, .. }
   ```

###### 特性

- 📏 **编译期量纲安全**
- ⚡ **零运行时开销**
- 🔢 **整数/浮点双支持**
- 🔄 **运行时词头处理**

##### 使用示例

###### 基础用法

```rust
use quantity::si::{Kelvin, Celsius, Quantity};

let boiling: Quantity<f64, Celsius> = 100.0.into();
let kelvin = boiling.convert::<Kelvin>(); // 373.15 K
```

###### 单位运算

```rust
let force = 5.0 * kg * m / (s * s);
let energy = force * (2.0 * m); // 10 焦耳
```

##### 对比

| 特性          | Quantity | uom   | nalgebra |
|--------------|----------|-------|----------|
| 量纲安全      | ✅        | ✅     | ❌        |
| 整数支持      | ✅        | ⚠️     | ❌        |
| 运行时词头    | ✅        | ❌     | ❌        |

##### Installation 安装

```rust
[dependencies]
quantity = "0.1"
```

##### Contributing 贡献

We welcome issues and PRs! 欢迎提交 Issue 和 PR！

Key needs:

- More unit definitions

- Real-world physics test cases

重点需求：

- 更多单位定义

- 实际物理测试案例
