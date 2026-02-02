use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

/// 应用配置结构
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// 服务器端口
    pub port: u16,

    /// 工作路径
    pub path: PathBuf,

    /// 运行模式
    pub mode: String,

    /// 时区
    pub zone: i32,

    /// 区域
    pub area: String,
}

impl AppConfig {
    /// 加载配置
    ///
    /// 优先级：环境变量 > 配置文件 > 默认值
    pub fn load(config_path: Option<&PathBuf>) -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            // 1. 设置默认值
            .set_default("port", 8000)?
            .set_default("path", "/home/foo/bar")?
            .set_default("mode", "happy mode")?
            .set_default("zone", 8)?
            .set_default("area", "Taipei")?;

        // 2. 从配置文件加载（如果提供）
        if let Some(path) = config_path {
            builder = builder.add_source(File::from(path.as_path()).required(false));
        } else {
            // 尝试加载默认配置文件
            builder = builder
                .add_source(File::with_name(".env").required(false))
                .add_source(File::with_name("config").required(false));
        }

        // 3. 从环境变量加载（覆盖配置文件）
        builder = builder.add_source(
            Environment::with_prefix("MEOW")
                .separator("_")
        );

        // 构建并反序列化
        builder.build()?.try_deserialize()
    }

    /// 从系统环境变量读取特定值
    pub fn get_system_env(key: &str) -> Option<String> {
        env::var(key).ok()
    }

    /// 显示配置信息
    pub fn display(&self) {
        println!("当前配置:");
        println!("  端口: {}", self.port);
        println!("  路径: {}", self.path.display());
        println!("  模式: {}", self.mode);
        println!("  时区: {}", self.zone);
        println!("  区域: {}", self.area);
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 8000,
            path: PathBuf::from("/home/foo/bar"),
            mode: "happy mode".to_string(),
            zone: 8,
            area: "Taipei".to_string(),
        }
    }
}
