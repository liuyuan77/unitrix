# PhysUnits · 物理单位库

**Type-safe physical quantities with dimensional analysis**  
**带量纲分析的类型安全物理量库**  

[![Crates.io](https://img.shields.io/crates/v/physunits)](https://crates.io/crates/physunits)
[![Docs.rs](https://img.shields.io/docsrs/physunits)](https://docs.rs/physunits)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust library for safe unit operations /  
Rust实现的类型安全单位计算库  

## Core Design / 核心设计

### 1. Dimension / 量纲

```rust
/// Base SI dimensions / 国际单位制基本量纲
pub struct Dimension<
    L: Integer,  // Length (m) / 长度(米)
    M: Integer,  // Mass (kg) / 质量(千克)
    T: Integer,  // Time (s) / 时间(秒)
    I: Integer,  // Current (A) / 电流(安培)
    Th: Integer, // Temperature (K) / 温度(开尔文) 
    N: Integer,  // Amount (mol) / 物质的量(摩尔)
    J: Integer   // Luminosity (cd) / 发光强度(坎德拉)
>(PhantomData<(L, M, T, I, Th, N, J)>);
```

### 2. Unit / 单位

```rust
/// Unit conversion rules / 单位转换规则
pub trait Unit {
    type Dimension;
    
    /// Convert to base unit / 转换到基准单位
    fn to_base(value: f64) -> f64;
    
    /// Unit symbol / 单位符号
    const SYMBOL: &'static str;
}
```

### 3. Quantity / 物理量

```rust
/// Physical quantity with value and unit / 带单位和值的物理量
pub struct Quantity<V, U: Unit> {
    /// Scalar value / 标量值
    pub value: V,
    _unit: PhantomData<U>
}

impl<V, U: Unit> Quantity<V, U> {
    /// Create new quantity / 创建新物理量
    pub fn new(value: V) -> Self {
        Self { value, _unit: PhantomData }
    }
}
```

## Features / 特性

| Feature | 功能描述 |
|---------|----------|
| 📏 Compile-time dimensional safety | 编译期量纲安全 |
| ⚡ Zero runtime overhead | 零运行时开销 |
| 🔢 Integer & float support | 支持整数和浮点数 |
| 🔄 Automatic unit conversion | 自动单位转换 |
| 🏷️ Runtime prefix handling | 运行时词头处理 |

## Usage / 使用示例

### Basic Conversion / 基础转换

```rust
use physunits::{Meter, Inch, Quantity};

// Create length / 创建长度
let length = Quantity::<f64, Meter>::new(2.0);

// Convert units / 单位转换
let inches = length.convert::<Inch>();
println!("{} m = {} in", length.value, inches.value);
```

### Temperature / 温度转换

```rust
use physunits::{Celsius, Fahrenheit};

let boiling = Quantity::<f64, Celsius>::new(100.0);
let fahr = boiling.convert::<Fahrenheit>();
println!("Water boils at {} °F", fahr.value); 
```

### Force Calculation / 力的计算

```rust
use physunits::{kg, m, s, N};

let mass = 5.0 * kg;
let acceleration = 9.8 * m / (s * s);
let force: Quantity<f64, N> = mass * acceleration;
println!("Force: {} N", force.value);
```

### Unit Math / 单位运算

```rust
let force = 5.0 * kg * m / (s * s);
let energy = force * (2.0 * m); // 10 J / 10焦耳
```

## Comparison / 对比

| Feature         | PhysUnits | uom   | nalgebra |
|----------------|----------|-------|----------|
| Dim Safety     | ✅        | ✅     | ❌        |
| Integer Support| ✅        | ⚠️     | ❌        |
| Runtime Prefix | ✅        | ❌     | ❌        |

| 特性          | PhysUnits | uom   | nalgebra |
|--------------|----------|-------|----------|
| 量纲安全      | ✅        | ✅     | ❌        |
| 整数支持      | ✅        | ⚠️     | ❌        |
| 运行时词头    | ✅        | ❌     | ❌        |

## Installation / 安装

```rust
[dependencies]
physunits = "0.0.1"
```

## Contributing / 贡献指南

We welcome issues and PRs! / 欢迎提交 Issue 和 PR！

Key needs: / 重点需求：

- More unit definitions / 更多单位定义

- Real-world physics test cases / 实际物理测试案例

- Better error messages / 更好的错误提示
