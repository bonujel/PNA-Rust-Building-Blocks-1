# Rust API 文档编写指南

> 原文档：https://rust-lang.github.io/api-guidelines/documentation.html
> 翻译日期：2026-02-04

## 概述

本指南提供了 Rust API 文档编写的最佳实践，确保 API 文档清晰、完整且对用户友好。遵循这些指南可以提高代码库的可维护性和用户体验。

---

## 核心文档要求

### C-CRATE-DOC: 全面的 Crate 级文档

**要求：** Crate 文档应该详尽且包含实用示例，遵循 RFC 1687 标准。

**说明：**
- Crate 级文档是用户了解你的库的第一入口
- 应该包含库的目的、主要功能、使用场景
- 提供快速入门示例和常见用例

**示例：**
```rust
//! # My Awesome Crate
//!
//! `my_crate` 提供了一套用于处理配置文件的工具。
//!
//! ## 快速开始
//!
//! ```rust
//! use my_crate::Config;
//!
//! let config = Config::from_file("config.toml")?;
//! println!("App name: {}", config.app_name);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## 主要功能
//!
//! - 支持多种配置格式（TOML、JSON、YAML）
//! - 类型安全的配置访问
//! - 环境变量覆盖
```

---

### C-EXAMPLE: 通用示例覆盖

**要求：** 每个公共 API 元素都应该包含示例。

**适用范围：**
- 模块（modules）
- Trait
- 结构体（structs）
- 枚举（enums）
- 函数（functions）
- 方法（methods）
- 宏（macros）
- 类型定义（type definitions）

**重点：** 示例应该展示**为什么**有人会使用某个项目，而不仅仅是机械地展示**如何**调用它。

**好的示例：**
```rust
/// 解析配置文件并返回 Config 实例。
///
/// 这个函数特别适用于需要在应用启动时
/// 加载配置的场景。
///
/// # 示例
///
/// ```rust
/// use my_crate::Config;
///
/// // 从文件加载配置
/// let config = Config::from_file("app.toml")?;
///
/// // 使用配置启动服务器
/// let server = Server::new(config.port, config.host);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn from_file(path: &str) -> Result<Config, Error> {
    // ...
}
```

**不好的示例：**
```rust
/// 从文件创建 Config
///
/// # 示例
///
/// ```rust
/// let config = Config::from_file("file.toml");
/// ```
pub fn from_file(path: &str) -> Result<Config, Error> {
    // ...
}
```

---

### C-QUESTION-MARK: 示例中使用现代错误处理

**要求：** 示例应该使用 `?` 操作符而不是 `try!` 或 `unwrap`。

**原因：** 用户经常会逐字复制示例代码，使用现代错误处理可以教导最佳实践。

**推荐模式：** 使用隐藏的设置代码来构建示例

```rust
/// 从文件读取配置
///
/// # 示例
///
/// ```rust
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use my_crate::Config;
///
/// let config = Config::from_file("config.toml")?;
/// println!("Loaded config: {:?}", config);
/// #     Ok(())
/// # }
/// ```
pub fn from_file(path: &str) -> Result<Config, Error> {
    // ...
}
```

**说明：**
- `#` 开头的行在文档中隐藏，但在测试时会执行
- 这样可以提供完整的可运行示例，同时保持文档简洁
- 用户看到的是干净的示例代码，但测试时有完整的上下文

---

### C-FAILURE: 记录错误、Panic 和安全条件

#### Errors 部分

**要求：** 为可能失败的函数记录错误条件。

**示例：**
```rust
/// 从文件读取配置
///
/// # Errors
///
/// 如果文件不存在、无法读取或包含无效的 TOML 格式，
/// 此函数将返回错误。
///
/// 具体错误类型：
/// - `Error::FileNotFound` - 文件不存在
/// - `Error::PermissionDenied` - 没有读取权限
/// - `Error::InvalidFormat` - TOML 格式无效
pub fn from_file(path: &str) -> Result<Config, Error> {
    // ...
}
```

**标准库示例：** `std::io::Read::read`
> "如果此函数遇到任何形式的 I/O 或其他错误，将返回错误变体。"

#### Panics 部分

**要求：** 记录 panic 条件。

**示例：**
```rust
/// 在指定索引处插入元素
///
/// # Panics
///
/// 如果 `index` 大于向量长度，此函数会 panic。
///
/// # 示例
///
/// ```rust
/// let mut vec = vec![1, 2, 3];
/// vec.insert(1, 4);
/// assert_eq!(vec, vec![1, 4, 2, 3]);
/// ```
pub fn insert(&mut self, index: usize, element: T) {
    // ...
}
```

**标准库示例：** `Vec::insert`
> "如果 `index` 越界则会 panic。"

#### Safety 部分

**要求：** 不安全函数必须解释调用者的责任。

**示例：**
```rust
/// 从原始指针创建字符串切片
///
/// # Safety
///
/// 调用者必须确保：
/// - `ptr` 指向有效的 UTF-8 数据
/// - `len` 不超过实际数据长度
/// - 数据在返回的切片生命周期内保持有效
/// - 数据不会被其他线程修改
///
/// 违反这些条件会导致未定义行为。
pub unsafe fn from_raw_parts(ptr: *const u8, len: usize) -> &str {
    // ...
}
```

**指南建议：**
- 记录有意义的 panic
- 避免过度记录每一个可能的情况
- 专注于用户需要知道的关键信息

---

### C-LINK: 超链接相关引用

**要求：** 文档内容应该使用 markdown 语法或 rustdoc 特定的反引号链接来包含超链接。

**Rustdoc 链接语法：**

```rust
/// 使用 [`Config`] 来配置应用程序。
///
/// 参见 [`from_file`](Config::from_file) 了解如何从文件加载配置。
///
/// 相关类型：
/// - [`Error`] - 错误类型
/// - [`Builder`] - 构建器模式
///
/// 外部链接：[RFC 1574](https://github.com/rust-lang/rfcs/pull/1574)
pub struct Config {
    // ...
}
```

**链接类型：**
- `[`Type`]` - 链接到类型
- `[`function`]` - 链接到函数
- `[`Type::method`]` - 链接到方法
- `[text](Type::method)` - 自定义链接文本
- `[text](https://...)` - 外部链接

**RFC 1574 建议：** "链接所有内容"

---

### C-METADATA: 完整的 Cargo.toml 元数据

**必需字段：**

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "简短的包描述"
license = "MIT OR Apache-2.0"
repository = "https://github.com/username/repo"
keywords = ["config", "toml", "settings"]
categories = ["config", "parsing"]
```

**可选字段：**

```toml
# 仅当托管在 docs.rs 之外时需要
documentation = "https://docs.example.com"

# 仅当与 repository/documentation 不同时需要
homepage = "https://example.com"

# README 文件
readme = "README.md"

# Rust 版本
edition = "2021"
rust-version = "1.56"
```

**说明：**
- `authors` - 作者信息（可选，已弃用）
- `description` - 简短描述（必需，用于 crates.io）
- `license` - 许可证（必需，使用 SPDX 表达式）
- `repository` - 源代码仓库（必需）
- `keywords` - 关键词（最多 5 个，每个最多 20 字符）
- `categories` - 分类（最多 5 个，从 crates.io 列表选择）

---

### C-RELNOTES: 维护发布说明

**要求：** 为每个版本记录重要更改，根据 RFC 1105 清楚地标识破坏性更改。

**CHANGELOG.md 示例：**

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- 新增 `Config::merge` 方法用于合并配置

### Changed
- **BREAKING**: `from_file` 现在返回 `Result<Config, Error>` 而不是 `Option<Config>`

### Fixed
- 修复了 TOML 解析中的 Unicode 处理问题

## [0.2.0] - 2026-01-15

### Added
- 支持 JSON 配置格式
- 新增 `Builder` 模式

### Changed
- 改进了错误消息

## [0.1.0] - 2025-12-01

### Added
- 初始版本
- 支持 TOML 配置文件
```

**版本控制系统标记：**

```bash
# 标记发布版本
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin v0.2.0
```

**最佳实践：**
- 使用 [Keep a Changelog](https://keepachangelog.com/) 格式
- 清楚标识破坏性更改（使用 **BREAKING** 标记）
- 遵循语义化版本（Semantic Versioning）
- 在版本控制系统中标记发布

---

### C-HIDDEN: 隐藏实现细节

**要求：** 使用 `#[doc(hidden)]` 和 `pub(crate)` 来防止内部实现细节出现在公共文档中。

**使用场景：**
- 内部辅助函数
- 实现细节
- 宏生成的代码
- 不稳定的 API

**示例：**

```rust
// 使用 pub(crate) 限制可见性
pub(crate) fn internal_helper() {
    // 仅在 crate 内部可见
}

// 使用 #[doc(hidden)] 隐藏文档
#[doc(hidden)]
pub fn __internal_macro_helper() {
    // 公开但不在文档中显示
}

// 组合使用
#[doc(hidden)]
pub struct __InternalState {
    // 内部状态，不应该被用户直接使用
}
```

**原则：** 只显示用户完整功能所需的 API

---

## 文档编写最佳实践总结

### 1. 文档结构

```rust
/// 简短的一句话描述（第一行）
///
/// 更详细的说明段落，解释功能的目的和用途。
///
/// # 示例
///
/// ```rust
/// // 实用的代码示例
/// ```
///
/// # Errors
///
/// 描述可能的错误情况
///
/// # Panics
///
/// 描述 panic 条件
///
/// # Safety
///
/// （仅用于 unsafe 函数）描述安全要求
pub fn my_function() {
    // ...
}
```

### 2. 示例编写原则

- ✅ 展示**为什么**使用，不只是**如何**使用
- ✅ 使用 `?` 操作符处理错误
- ✅ 提供完整的可运行示例
- ✅ 使用 `#` 隐藏样板代码
- ✅ 包含常见用例

### 3. 链接使用

- ✅ 链接到相关类型和函数
- ✅ 使用 rustdoc 链接语法
- ✅ 提供外部资源链接
- ✅ 交叉引用相关文档

### 4. 元数据完整性

- ✅ 填写所有必需字段
- ✅ 提供准确的描述
- ✅ 选择合适的关键词和分类
- ✅ 维护 CHANGELOG

### 5. 可见性控制

- ✅ 使用 `pub(crate)` 限制内部 API
- ✅ 使用 `#[doc(hidden)]` 隐藏实现细节
- ✅ 只暴露必要的公共 API

---

## 工具和资源

### 文档生成

```bash
# 生成文档
cargo doc

# 生成并打开文档
cargo doc --open

# 包含私有项
cargo doc --document-private-items
```

### 文档测试

```bash
# 运行文档测试
cargo test --doc

# 详细输出
cargo test --doc -- --nocapture
```

### 检查工具

```bash
# 检查文档链接
cargo doc --no-deps

# 使用 clippy 检查文档
cargo clippy -- -W missing_docs
```

---

## 参考资源

- [RFC 1687 - Crate-level documentation](https://github.com/rust-lang/rfcs/pull/1687)
- [RFC 1574 - More API documentation conventions](https://github.com/rust-lang/rfcs/pull/1574)
- [RFC 1105 - API Evolution](https://github.com/rust-lang/rfcs/pull/1105)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [The rustdoc book](https://doc.rust-lang.org/rustdoc/)

---

**文档版本：** 截至 2026-02-04
**参考来源：** [Rust API Guidelines - Documentation](https://rust-lang.github.io/api-guidelines/documentation.html)

