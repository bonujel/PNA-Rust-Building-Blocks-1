# Cargo 环境变量参考文档

> 原文档：https://doc.rust-lang.org/cargo/reference/environment-variables.html
> 翻译日期：2026-02-04

## 概述

本文档描述了 Cargo 使用和设置的环境变量，包括 Cargo 读取的配置变量、为 crate 设置的编译时变量、以及构建脚本可用的变量。

---

## 1. Cargo 读取的环境变量

这些环境变量可以由用户设置来控制 Cargo 的行为：

### 1.1 核心配置变量

| 变量名 | 说明 |
|--------|------|
| `CARGO_HOME` | 覆盖默认缓存位置（默认为 `~/.cargo`） |
| `CARGO_TARGET_DIR` | 指定构建产物的存放位置 |
| `RUSTC` | 使用自定义的 Rust 编译器 |
| `RUSTDOC` | 使用自定义的文档生成器 |

**示例：**
```bash
# 设置自定义构建目录
export CARGO_TARGET_DIR=/tmp/my-build

# 使用自定义编译器
export RUSTC=/path/to/custom/rustc
```

### 1.2 编译器标志变量

| 变量名 | 说明 |
|--------|------|
| `RUSTFLAGS` | 向所有编译器调用传递标志（空格分隔） |
| `RUSTDOCFLAGS` | 向所有文档生成器调用传递标志 |
| `CARGO_ENCODED_RUSTFLAGS` | 使用 `0x1f` 分隔的标志（比空格分隔更可靠） |

**示例：**
```bash
# 启用所有警告
export RUSTFLAGS="-W warnings"

# 使用编码格式（更可靠）
export CARGO_ENCODED_RUSTFLAGS="-W\x1fwarnings"
```

### 1.3 构建优化变量

| 变量名 | 说明 |
|--------|------|
| `RUSTC_WRAPPER` | 包装 rustc 调用（对于像 sccache 这样的构建缓存很有用） |
| `CARGO_INCREMENTAL` | 强制启用（1）或禁用（0）增量编译 |

**示例：**
```bash
# 使用 sccache 加速编译
export RUSTC_WRAPPER=sccache

# 禁用增量编译
export CARGO_INCREMENTAL=0
```

---

## 2. Cargo 为 Crate 设置的环境变量

这些环境变量在编译期间可用，可以使用 `env!()` 宏访问：

### 2.1 包信息变量

| 变量名 | 说明 |
|--------|------|
| `CARGO_PKG_NAME` | 包名称 |
| `CARGO_PKG_VERSION` | 完整版本号 |
| `CARGO_PKG_VERSION_MAJOR` | 主版本号 |
| `CARGO_PKG_VERSION_MINOR` | 次版本号 |
| `CARGO_PKG_VERSION_PATCH` | 补丁版本号 |
| `CARGO_PKG_VERSION_PRE` | 预发布版本 |
| `CARGO_PKG_AUTHORS` | 作者列表（用 `:` 分隔） |
| `CARGO_PKG_DESCRIPTION` | 包描述 |
| `CARGO_PKG_HOMEPAGE` | 主页 URL |
| `CARGO_PKG_REPOSITORY` | 仓库 URL |
| `CARGO_PKG_LICENSE` | 许可证 |
| `CARGO_PKG_LICENSE_FILE` | 许可证文件路径 |

**使用示例：**
```rust
// 在代码中访问包信息
const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    println!("{} version {}", NAME, VERSION);
}
```

### 2.2 路径变量

| 变量名 | 说明 |
|--------|------|
| `CARGO_MANIFEST_DIR` | 包含 Cargo.toml 的目录（绝对路径） |
| `OUT_DIR` | 构建脚本输出目录 |

**使用示例：**
```rust
// 读取项目根目录下的文件
const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
let config_path = format!("{}/config.toml", MANIFEST_DIR);
```

### 2.3 Crate 信息变量

| 变量名 | 说明 |
|--------|------|
| `CARGO_CRATE_NAME` | 当前正在编译的 crate 名称 |
| `CARGO_BIN_NAME` | 当前正在编译的二进制文件名称 |
| `CARGO_BIN_EXE_<name>` | 指定二进制可执行文件的路径（用于集成测试） |

**使用示例：**
```rust
// 在集成测试中运行二进制文件
#[test]
fn test_binary() {
    let exe = env!("CARGO_BIN_EXE_my_app");
    let output = std::process::Command::new(exe)
        .arg("--version")
        .output()
        .unwrap();
    assert!(output.status.success());
}
```

---

## 3. 构建脚本的环境变量

这些环境变量在构建脚本（`build.rs`）运行时可用，使用 `std::env::var()` 访问：

### 3.1 基本变量

| 变量名 | 说明 |
|--------|------|
| `OUT_DIR` | 构建脚本输出的存放位置 |
| `TARGET` | 目标三元组（例如 `x86_64-unknown-linux-gnu`） |
| `HOST` | 主机三元组 |
| `PROFILE` | 构建配置（`debug` 或 `release`） |
| `OPT_LEVEL` | 优化级别（0-3） |
| `DEBUG` | 是否启用调试信息（`true` 或 `false`） |

**使用示例：**
```rust
// build.rs
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();

    println!("Building for target: {}", target);
    println!("Output directory: {}", out_dir);
}
```

### 3.2 配置变量

| 变量名 | 说明 |
|--------|------|
| `CARGO_CFG_<cfg>` | 配置选项（例如 `CARGO_CFG_TARGET_OS=linux`） |
| `CARGO_CFG_TARGET_OS` | 目标操作系统 |
| `CARGO_CFG_TARGET_ARCH` | 目标架构 |
| `CARGO_CFG_TARGET_FAMILY` | 目标系列（`unix` 或 `windows`） |

**使用示例：**
```rust
// build.rs
use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "linux" => println!("cargo:rustc-link-lib=dl"),
        "macos" => println!("cargo:rustc-link-lib=System"),
        "windows" => println!("cargo:rustc-link-lib=user32"),
        _ => {}
    }
}
```

### 3.3 特性变量

| 变量名 | 说明 |
|--------|------|
| `CARGO_FEATURE_<name>` | 激活的特性（大写，`-` 转换为 `_`） |

**使用示例：**
```rust
// build.rs
use std::env;

fn main() {
    // 检查是否启用了 "ssl" 特性
    if env::var("CARGO_FEATURE_SSL").is_ok() {
        println!("cargo:rustc-link-lib=ssl");
        println!("cargo:rustc-link-lib=crypto");
    }
}
```

### 3.4 依赖元数据变量

| 变量名 | 说明 |
|--------|------|
| `DEP_<links>_<key>` | 来自带有 `links` 的依赖项的元数据 |

**使用示例：**
```rust
// build.rs
use std::env;

fn main() {
    // 获取 libgit2-sys 提供的元数据
    if let Ok(include_path) = env::var("DEP_GIT2_INCLUDE") {
        println!("cargo:include={}", include_path);
    }
}
```

---

## 4. 配置环境变量

Cargo 支持所有配置值的 `CARGO_<CONFIG_PATH>` 格式。配置路径使用下划线分隔，并转换为大写。

### 4.1 常用配置变量

| 变量名 | 说明 | 对应配置 |
|--------|------|----------|
| `CARGO_BUILD_JOBS` | 并行作业数 | `build.jobs` |
| `CARGO_BUILD_TARGET` | 默认目标三元组 | `build.target` |
| `CARGO_NET_OFFLINE` | 离线模式 | `net.offline` |
| `CARGO_TERM_COLOR` | 颜色输出控制 | `term.color` |
| `CARGO_TERM_VERBOSE` | 详细输出 | `term.verbose` |
| `CARGO_HTTP_PROXY` | HTTP 代理 | `http.proxy` |
| `CARGO_HTTP_TIMEOUT` | HTTP 超时 | `http.timeout` |

**示例：**
```bash
# 设置并行作业数
export CARGO_BUILD_JOBS=4

# 启用离线模式
export CARGO_NET_OFFLINE=true

# 设置颜色输出
export CARGO_TERM_COLOR=always

# 设置 HTTP 代理
export CARGO_HTTP_PROXY=http://proxy.example.com:8080
```

### 4.2 颜色输出选项

`CARGO_TERM_COLOR` 可选值：
- `auto`（默认）：自动检测终端支持
- `always`：始终使用颜色
- `never`：从不使用颜色

---

## 5. 实用技巧

### 5.1 在 .cargo/config.toml 中设置环境变量

```toml
[env]
RUST_LOG = "debug"
DATABASE_URL = "postgres://localhost/mydb"
```

### 5.2 条件编译示例

```rust
// 根据目标平台条件编译
#[cfg(target_os = "linux")]
fn platform_specific() {
    println!("Running on Linux");
}

#[cfg(target_os = "windows")]
fn platform_specific() {
    println!("Running on Windows");
}
```

### 5.3 构建脚本输出指令

```rust
// build.rs
fn main() {
    // 链接库
    println!("cargo:rustc-link-lib=static=mylib");

    // 添加链接搜索路径
    println!("cargo:rustc-link-search=native=/path/to/lib");

    // 设置环境变量
    println!("cargo:rustc-env=MY_VAR=value");

    // 重新运行条件
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CC");
}
```

---

## 6. 最佳实践

1. **使用 `env!()` 宏进行编译时检查**：如果环境变量不存在，编译会失败
   ```rust
   const VERSION: &str = env!("CARGO_PKG_VERSION");
   ```

2. **使用 `option_env!()` 宏处理可选变量**：
   ```rust
   const CUSTOM_VAR: Option<&str> = option_env!("MY_CUSTOM_VAR");
   ```

3. **在构建脚本中使用 `std::env::var()`**：
   ```rust
   let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
   ```

4. **避免在代码中硬编码路径**：使用 `CARGO_MANIFEST_DIR`
   ```rust
   let config = format!("{}/config.toml", env!("CARGO_MANIFEST_DIR"));
   ```

5. **使用 `CARGO_ENCODED_RUSTFLAGS` 而不是 `RUSTFLAGS`**：
   - 更可靠，避免空格分隔的问题
   - 特别是在 CI/CD 环境中

---

## 7. 常见问题

### Q: 如何在运行时访问编译时环境变量？

A: 使用 `env!()` 宏将环境变量嵌入到二进制文件中：
```rust
const BUILD_TIME: &str = env!("BUILD_TIME");
```

### Q: 构建脚本的输出去哪了？

A: 输出到 `OUT_DIR` 指定的目录，通常在 `target/debug/build/<package>/out/`

### Q: 如何调试构建脚本？

A: 使用 `cargo build -vv` 查看详细输出，包括构建脚本的 stdout

---

**文档版本：** 截至 2026-02-04 的稳定功能
**参考来源：** [Cargo 官方文档](https://doc.rust-lang.org/cargo/reference/environment-variables.html)

