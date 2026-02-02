# Meow - Rust CLI 学习项目

一个用于学习 Rust CLI 开发的示例项目，展示了以下最佳实践：

## 🎯 学习目标

- ✅ CLI 参数解析（使用 clap）
- ✅ 配置管理（使用 config + dotenvy）
- ✅ 错误处理（自定义错误类型）
- ✅ 模块化设计
- ✅ 文件 IO 操作

## 📦 依赖

- `clap` - CLI 参数解析
- `config` - 配置管理
- `serde` - 序列化/反序列化
- `dotenvy` - .env 文件支持

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

## 📁 项目结构

```
src/
├── main.rs       # 程序入口和 CLI 定义
├── config.rs     # 配置管理
├── error.rs      # 错误类型定义
└── processor.rs  # 业务逻辑处理
```

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

## 📝 注意事项

- 这是一个**学习项目**，业务逻辑是示例性的
- 占位逻辑（如 `process_happy`）可以根据实际需求替换
- 测试框架已搭建，可以添加具体测试用例

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
```

## 📚 扩展学习

如果想进一步学习，可以尝试：

1. 添加单元测试（`#[cfg(test)]`）
2. 实现实际的业务逻辑
3. 添加日志支持（`log` + `env_logger`）
4. 添加更多子命令
5. 实现配置验证逻辑

## 📄 许可证

MIT
