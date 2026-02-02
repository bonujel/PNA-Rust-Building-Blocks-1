use std::fmt;
use std::io;

/// 应用错误类型
#[derive(Debug)]
pub enum AppError {
    /// 配置错误
    ConfigError(String),

    /// 文件IO错误
    IoError(io::Error),

    /// 文件不存在
    FileNotFound(String),

    /// 无效的输入
    InvalidInput(String),

    /// 测试失败
    TestFailed(String, u32), // 错误消息和失败数量
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ConfigError(msg) => {
                write!(f, "配置错误: {}", msg)
            }
            AppError::IoError(err) => {
                write!(f, "IO错误: {}", err)
            }
            AppError::FileNotFound(path) => {
                write!(f, "文件不存在: {}", path)
            }
            AppError::InvalidInput(msg) => {
                write!(f, "无效输入: {}", msg)
            }
            AppError::TestFailed(msg, count) => {
                write!(f, "测试失败: {} (失败数量: {})", msg, count)
            }
        }
    }
}

impl std::error::Error for AppError {}

// 从io::Error自动转换
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err)
    }
}

// 从config::ConfigError自动转换
impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::ConfigError(err.to_string())
    }
}

/// 应用Result类型别名
pub type AppResult<T> = Result<T, AppError>;

/// 退出码定义
pub mod exit_codes {
    /// 成功
    pub const SUCCESS: i32 = 0;

    /// 配置错误
    pub const CONFIG_ERROR: i32 = 1;

    /// 文件错误
    pub const FILE_ERROR: i32 = 2;

    /// 输入错误
    pub const INPUT_ERROR: i32 = 3;

    /// 测试失败
    pub const TEST_FAILED: i32 = 4;
}

impl AppError {
    /// 获取对应的退出码
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::ConfigError(_) => exit_codes::CONFIG_ERROR,
            AppError::IoError(_) | AppError::FileNotFound(_) => exit_codes::FILE_ERROR,
            AppError::InvalidInput(_) => exit_codes::INPUT_ERROR,
            AppError::TestFailed(_, _) => exit_codes::TEST_FAILED,
        }
    }
}
