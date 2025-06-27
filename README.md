# unitrix · 单位算阵

**Unitrix: Normalized physical unit management and 2D geometry computing through constified matrices.
Delivers zero-cost abstractions with no_std support.**  
**单位算阵：通过常量化矩阵实现物理量单位化与2D几何计算规范化。
提供零成本抽象，支持no_std环境。**  

[![Crates.io](https://img.shields.io/crates/v/unitrix)](https://crates.io/crates/unitrix)
[![Docs.rs](https://img.shields.io/docsrs/unitrix)](https://docs.rs/unitrix)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Key Advantages / 核心优势

1. **No dependencies** - Pure Rust implementation without external dependencies  
   **无依赖库** - 纯Rust实现，不依赖任何外部库

2. **Typenum-like constant calculation** - Full compile-time dimensional analysis with constant evaluation capabilities  
   **类Typenum的常量计算** - 完整的编译期量纲分析与常量计算能力

3. **Var structure bridge** - Seamless integration between variable and constant calculations  
   **Var结构桥接** - 完美衔接变量与常量的混合计算

4. **Complete operator overloading** - Supports all arithmetic operations with dimensional correctness  
   **完整运算符重载** - 支持所有算术运算并保持量纲正确性

## Core Architecture / 核心架构

### 1. Constant System / 常量系统

```rust
// 基础常量类型
pub struct B0<Other>(PhantomData<Other>);  // 二进制0节点
pub struct B1<Other>(PhantomData<Other>);  // 二进制1节点
pub struct Z0;                     // 0
pub struct P1;                     // +1
pub struct N1;                     // -1
pub struct FixedPoint<IntPart, FracPart>(PhantomData<(IntPart, FracPart)>);    //小数
pub struct Float<Significand, Exponent>(PhantomData<(Significand, Exponent)>)  //浮点数
```

### 2. Var Structure / 变量结构

```rust
/// 变量结构体，桥接常量与变量计算
#[derive(Debug, Clone, Copy)]
pub struct Var<T>(pub T);

// 支持的运算类型
impl<T: Primitive> Add for Var<T> {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Var(self.0 + b.0)
    }
}

// 与常量的混合运算
impl<T: Primitive, C: Integer> Add<C> for Var<T> {
    type Output = C::Output;
    fn add(self, c: C) -> Self::Output {
        c + self  // 调用常量的加法实现
    }
}
```

### 3. Dimension System / 量纲系统

```rust
/// 国际单位制7大量纲
pub struct Dimension<
    METER: Integer,     // 长度
    KILOGRAM: Integer,  // 质量
    SECOND: Integer,    // 时间
    AMPERE: Integer,    // 电流
    KELVIN: Integer,    // 温度
    MOLE: Integer,      // 物质的量
    CANDELA: Integer    // 发光强度
>(PhantomData<(METER, KILOGRAM, SECOND, AMPERE, KELVIN, MOLE, CANDELA)>);

// 量纲运算示例
impl<M1, M2, KG1, KG2> Mul<Dimension<M2, KG2>> for Dimension<M1, KG1> {
    type Output = Dimension<Sum<M1, M2>, Sum<KG1, KG2>>;
    fn mul(self, _: Dimension<M2, KG2>) -> Self::Output {
        Dimension::new()  // 量纲指数相加
    }
}
```

### 4. Unit System / 单位系统

```rust
/// SI基础单位结构
pub struct Si<
    Value,
    D:Dimensional,
    Pr:Prefixed,
>(
    pub Value,
    pub PhantomData<(D, Pr)>
);

/// 复合单位结构
pub struct Unit<S: Sied, R>(pub S,pub PhantomData<R>);
```

## Features / 特性

| Feature | 功能描述 |
|---------|----------|
| 📏 Compile-time dimensional safety | 编译期量纲安全 |
| ⚡ Zero runtime overhead | 零运行时开销 |
| 🔢 Integer & float support | 支持整数和浮点数 |
| 🔄 Automatic unit conversion | 自动单位转换 |
| 🏷️ Runtime prefix handling | 运行时词头处理 |
| 🧮 Typenum-like constant math | 类Typenum的常量计算 |
| 🌉 Var-based mixed calculation | 基于Var的混合计算 |
| 🔧 Full operator overloading | 完整运算符重载 |

## Development Progress Checklist / 开发进度清单

### Core Features / 核心功能

⚠️ Constant calculation system / 常量计算系统
✅ Var structure bridge / 变量结构桥接
✅ Dimensional analysis / 量纲分析系统
✅ SI unit framework / SI单位框架
⚠️ Full operator overloading / 完整运算符重载

### Test Coverage / 测试覆盖

⚠️ Constant ops tests (10% coverage) / 常量运算测试(10%覆盖率)
⚠️ Mixed calculation tests / 混合计算测试
⚠️ Dimensional validation / 量纲验证测试
⚠️ Unit conversion suite / 单位转换测试套件
⚠️ Edge case tests (in progress) / 边界条件测试(进行中)

### Documentation / 文档

⚠️ Core architecture / 核心架构文档
⚠️ API reference / API参考文档
⚠️ Tutorial examples (10% complete) / 教程示例(10%完成)
❌ Advanced usage guide / 高级用法指南

## Advanced Features / 高级特性

1. 常量计算系统

+ 二进制编码的常量类型 (B0, B1)

+ 基础常量值 (Z0, P1, N1)

+ 支持所有算术运算的trait实现

2. Var结构桥接

+ 同时支持变量与常量的运算

+ 自动类型转换系统

+ 完整的运算符重载 (+, -, *, /, +=, -=, *=, /=)

+ 支持i8-i64和f32、f64基础类型

3. 量纲系统

+ 7大基本量纲的编译期检查

+ 量纲运算自动推导

+ 支持幂运算 (pow()方法)

+ 零开销抽象

## Comparison / 对比

| Feature/特性          | unitrix | uom   | nalgebra |
|--------------|----------|-------|----------|
| Dim Safety/量纲安全      | ✅ | ✅ | ❌ |
| Integer Support/整数支持 | ✅ | ⚠️ | ❌ |
| Runtime Prefix/运行时词头 | ✅ | ❌ | ❌ |
| No Deps/无依赖           | ✅ | ❌ | ❌ |
| Const Math/常量计算      | ✅ | ⚠️ | ❌ |
| Var Bridge/变量桥接      | ✅ | ❌ | ❌ |

## Installation / 安装

```toml
[dependencies]
unitrix = "0.0.5"
```

## Contributing / 贡献指南

We welcome issues and PRs! / 欢迎提交 Issue 和 PR！

Key needs: / 重点需求：

+ More unit definitions / 更多单位定义

+ Real-world physics test cases / 实际物理测试案例

+ Better error messages / 更好的错误提示

+ Constant calculation optimization / 常量计算优化

+ WASM compatibility / WASM兼容性
