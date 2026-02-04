# PNA-Rust-Building-Blocks-1

> Practical Network Applications in Rust - Building Blocks 系列第一部分

一个用于学习 Rust CLI 开发和 Cargo 工具链的综合学习项目，包含实践代码和完整的中文参考文档。

## 📚 项目内容

### 1. CLI 应用示例

一个功能完整的 Rust CLI 应用，展示了以下最佳实践：

- ✅ CLI 参数解析（使用 clap）
- ✅ 配置管理（使用 config + dotenvy）
- ✅ 错误处理（自定义错误类型）
- ✅ 模块化设计
- ✅ 文件 IO 操作

### 2. Rust 参考文档（中文翻译）

本项目包含以下官方文档的完整中文翻译：

| 文档 | 说明 |
|------|------|
| [CARGO_MANIFEST_参考文档.md](./CARGO_MANIFEST_参考文档.md) | Cargo.toml 配置文件完整参考 |
| [CARGO_环境变量_参考文档.md](./CARGO_环境变量_参考文档.md) | Cargo 环境变量使用指南 |
| [RUST_API文档指南.md](./RUST_API文档指南.md) | Rust API 文档编写最佳实践 |

---

## 🎯 学习目标

通过本项目，你将学习：

1. **Cargo 工具链**
   - 理解 Cargo.toml 的所有配置选项
   - 掌握环境变量的使用
   - 学习构建脚本和特性管理

2. **CLI 开发**
   - 使用 clap 进行参数解析
   - 实现多层配置系统
   - 设计用户友好的命令行界面

3. **Rust 最佳实践**
   - 错误处理模式
   - 模块化设计
   - API 文档编写规范

---

## 📦 依赖

- `clap` - CLI 参数解析
- `config` - 配置管理
- `serde` - 序列化/反序列化
- `dotenvy` - .env 文件支持

---

## 🚀 使用方法

### 基本用法

```bash
# 处理文件（默认模式）
cargo run -- test.txt

# 使用详细模式
cargo run -- test.txt -v

# 使用自定义配置文件
cargo run -- test.txt -c config.toml
```

### 子命令

```bash
# 运行测试
cargo run -- test.txt test

# 运行测试（调试模式）
cargo run -- test.txt test --debug

# 显示配置
cargo run -- test.txt config
```

---

## ⚙️ 配置

配置优先级：环境变量 > 配置文件 > 默认值

### 环境变量

```bash
export MEOW_PORT=9000
export MEOW_MODE="production"
export MEOW_ZONE=8
export MEOW_AREA="Beijing"
```

### .env 文件

```env
MEOW_PORT=9000
MEOW_MODE=production
MEOW_ZONE=8
MEOW_AREA=Beijing
```

### config.toml

```toml
port = 9000
path = "/home/user/data"
mode = "production"
zone = 8
area = "Beijing"
```

---

## 📁 项目结构

```
.
├── src/
│   ├── main.rs       # 程序入口和 CLI 定义
│   ├── config.rs     # 配置管理
│   ├── error.rs      # 错误类型定义
│   └── processor.rs  # 业务逻辑处理
├── docs/
│   ├── CARGO_MANIFEST_参考文档.md
│   ├── CARGO_环境变量_参考文档.md
│   └── RUST_API文档指南.md
├── Cargo.toml        # 项目配置
└── README.md         # 本文件
```

---

## 🎓 学习要点

### 1. 错误处理

项目展示了如何创建自定义错误类型：

```rust
pub enum AppError {
    ConfigError(String),
    IoError(io::Error),
    FileNotFound(String),
    InvalidInput(String),
    TestFailed(String, u32),
}
```

### 2. 配置管理

三层配置系统：

```rust
// 1. 默认值
.set_default("port", 8000)?

// 2. 配置文件
.add_source(File::with_name("config").required(false))

// 3. 环境变量（最高优先级）
.add_source(Environment::with_prefix("MEOW"))
```

### 3. CLI 设计

使用 clap 的 derive API：

```rust
#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}
```

---

## 📖 参考文档使用指南

### Cargo Manifest 参考

查看 [CARGO_MANIFEST_参考文档.md](./CARGO_MANIFEST_参考文档.md) 了解：
- `[package]` 部分的所有配置选项
- 依赖管理（dependencies, dev-dependencies, build-dependencies）
- Features 和条件编译
- Lints 配置
- 发布到 crates.io 的要求

### Cargo 环境变量参考

查看 [CARGO_环境变量_参考文档.md](./CARGO_环境变量_参考文档.md) 了解：
- Cargo 读取的环境变量
- 编译时可用的环境变量
- 构建脚本（build.rs）中的环境变量
- 配置环境变量的使用

### Rust API 文档指南

查看 [RUST_API文档指南.md](./RUST_API文档指南.md) 了解：
- 如何编写高质量的 API 文档
- 文档示例的最佳实践
- 错误、Panic 和安全条件的文档要求
- Cargo.toml 元数据配置

---

## 🔧 开发

```bash
# 检查代码
cargo check

# 运行 clippy
cargo clippy

# 格式化代码
cargo fmt

# 构建
cargo build

# 运行
cargo run -- test.txt

# 运行测试
cargo test
```

---

## 📝 注意事项

- 这是一个**学习项目**，业务逻辑是示例性的
- 占位逻辑（如 `process_happy`）可以根据实际需求替换
- 测试框架已搭建，可以添加具体测试用例
- 参考文档基于 Rust 官方文档翻译，截至 2026-02-04

---

## 📚 扩展学习

如果想进一步学习，可以尝试：

1. **代码实践**
   - 添加单元测试（`#[cfg(test)]`）
   - 实现实际的业务逻辑
   - 添加日志支持（`log` + `env_logger`）
   - 添加更多子命令
   - 实现配置验证逻辑

2. **文档学习**
   - 深入阅读三份参考文档
   - 实践文档中的示例代码
   - 为自己的项目编写高质量文档

3. **工具链探索**
   - 尝试不同的 Cargo 配置选项
   - 使用环境变量控制构建行为
   - 编写构建脚本（build.rs）

---

## 🌟 特色

- ✅ **完整的中文文档**：三份核心参考文档的完整翻译
- ✅ **实践代码示例**：可运行的 CLI 应用
- ✅ **最佳实践**：遵循 Rust 社区的编码规范
- ✅ **模块化设计**：清晰的代码结构
- ✅ **配置灵活**：多层配置系统

---

## 📄 许可证

MIT

---

## 🔗 相关资源

- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

---

**项目维护：** 持续更新中
**最后更新：** 2026-02-04

