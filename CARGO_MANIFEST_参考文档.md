# Cargo.toml Manifest 格式参考

> 原文档：https://doc.rust-lang.org/cargo/reference/manifest.html
> 翻译日期：2026-02-04

## 概述

`Cargo.toml` 文件使用 TOML 格式编写，包含编译包所需的元数据。

---

## 1. `[package]` 部分

核心包元数据配置：

```toml
[package]
name = "hello_world"           # 必需：字母数字、- 或 _
version = "0.1.0"              # SemVer 格式 (major.minor.patch)
authors = ["Name <email>"]     # 可选（已弃用）
edition = "2024"               # Rust 版本 (2015, 2018, 2021, 2024)
rust-version = "1.56"          # 最低支持的 Rust 版本
description = "简短描述"        # 纯文本
documentation = "https://docs.rs/crate"
readme = "README.md"           # README 文件路径
homepage = "https://example.com"
repository = "https://github.com/user/repo"
license = "MIT OR Apache-2.0"  # SPDX 表达式
license-file = "LICENSE.txt"   # license 的替代方案
keywords = ["gamedev", "graphics"]  # 最多 5 个，每个 20 字符
categories = ["command-line-utilities"]  # 最多 5 个
workspace = "path/to/workspace"
build = "build.rs"             # 构建脚本路径
links = "git2"                 # 原生库名称
publish = false                # 或 ["registry-name"]
default-run = "binary-name"    # cargo run 的默认二进制文件
```

### 1.1 文件包含/排除

控制哪些文件被打包：

```toml
exclude = ["/ci", "images/", ".*"]
include = ["/src", "COPYRIGHT", "!/examples/big_example"]
```

### 1.2 元数据表

用于外部工具的自定义元数据：

```toml
[package.metadata.android]
package-name = "my-app"
assets = "path/to/static"
```

---

## 2. 目标表（Targets）

配置构建目标：

### 2.1 库目标

```toml
[lib]
name = "foo"
path = "src/lib.rs"
crate-type = ["lib"]
```

### 2.2 二进制目标

```toml
[[bin]]
name = "my-app"
path = "src/main.rs"
```

### 2.3 示例目标

```toml
[[example]]
name = "simple"
required-features = ["feature-name"]
```

### 2.4 测试目标

```toml
[[test]]
name = "integration"
```

### 2.5 基准测试目标

```toml
[[bench]]
name = "performance"
```

---

## 3. 依赖部分

### 3.1 基本依赖

```toml
[dependencies]
serde = "1.0"
```

### 3.2 开发依赖

仅用于测试和示例：

```toml
[dev-dependencies]
criterion = "0.3"
```

### 3.3 构建依赖

用于构建脚本：

```toml
[build-dependencies]
cc = "1.0"
```

### 3.4 平台特定依赖

```toml
[target.'cfg(windows)'.dependencies]
winapi = "0.3"
```

---

## 4. `[features]` 部分

条件编译功能：

```toml
[features]
default = ["std"]
std = []
extra = ["dep:optional-dep"]
```

**说明：**
- `default`：默认启用的功能
- 功能可以启用其他功能或可选依赖
- 使用 `dep:` 前缀引用可选依赖

---

## 5. `[lints]` 部分

配置 lint 级别（最低 Rust 版本：1.74）：

```toml
[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
enum_glob_use = "deny"
```

**Lint 级别：**
- `forbid`：禁止，无法覆盖
- `deny`：错误
- `warn`：警告
- `allow`：允许

**注意：** Lint 配置仅影响当前包，不影响依赖项。

---

## 6. `[hints]` 部分

编译提示（最低 Rust 版本：1.90）：

```toml
[hints]
# 目前没有稳定的提示可用
```

---

## 7. `[badges]` 部分

状态徽章（在 crates.io 上大多已弃用）：

```toml
[badges]
maintenance = { status = "actively-developed" }
```

---

## 8. 其他重要部分

### 8.1 `[profile.*]`
编译器优化设置

### 8.2 `[patch]`
覆盖依赖项

### 8.3 `[replace]`
覆盖依赖项（已弃用，使用 `[patch]` 代替）

### 8.4 `[workspace]`
工作空间定义

---

## 关键要点总结

### 发布到 crates.io 的必需字段

1. `name` - 包名称
2. `version` - 版本号
3. `license` 或 `license-file` - 许可证
4. `description` - 包描述

### 版本格式

使用 SemVer（语义化版本）：
- 格式：`major.minor.patch`
- 可选预发布版本：`1.0.0-alpha`
- 可选元数据：`1.0.0+meta`

### 模式匹配

使用 gitignore 风格的模式进行文件包含/排除：
- `*` - 匹配任意字符
- `?` - 匹配单个字符
- `**` - 匹配任意目录层级
- `!` - 否定模式

### 自动发现

Cargo 会自动发现标准位置的目标：
- `src/lib.rs` - 库
- `src/main.rs` - 主二进制文件
- `src/bin/*.rs` - 其他二进制文件
- `examples/*.rs` - 示例
- `tests/*.rs` - 集成测试
- `benches/*.rs` - 基准测试

---

## 最佳实践

1. **始终指定 `edition`**：明确 Rust 版本，避免兼容性问题
2. **使用 `rust-version`**：声明最低支持的 Rust 版本
3. **合理使用 features**：将可选功能模块化
4. **配置 lints**：提高代码质量
5. **文档完整**：填写 `description`、`readme`、`repository` 等字段
6. **许可证清晰**：使用标准 SPDX 表达式

---

**文档版本：** 截至 2026-02-04 的稳定功能
**参考来源：** [Cargo 官方文档](https://doc.rust-lang.org/cargo/reference/manifest.html)

