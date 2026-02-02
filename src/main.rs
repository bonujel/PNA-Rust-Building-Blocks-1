mod config;
mod error;
mod processor;

use clap::{Parser, Subcommand};
use config::AppConfig;
use error::{exit_codes, AppResult};
use processor::{FileProcessor, TestRunner};
use std::path::PathBuf;
use std::process;

/// Meow - 一个灵活的文件处理工具
#[derive(Parser)]
#[command(name = "meow")]
#[command(version = "0.1.0")]
#[command(about = "一个灵活的文件处理工具", long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Sets the input file to use
    #[arg(value_name = "INPUT")]
    input: String,

    /// Sets the level of verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Controls testing features
    Test {
        /// Print debug information
        #[arg(short, long)]
        debug: bool,
    },
    /// Show current configuration
    Config,
}

fn run() -> AppResult<()> {
    // 加载.env文件（如果存在）
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();

    // 加载配置
    let app_config = AppConfig::load(cli.config.as_ref())?;

    // 处理详细程度
    if cli.verbose > 0 {
        println!("详细模式: 级别 {}", cli.verbose);
        app_config.display();
    }

    // 处理子命令
    match &cli.command {
        Some(Commands::Test { debug }) => {
            println!("运行测试模式");
            TestRunner::run_tests(*debug)?;
        }
        Some(Commands::Config) => {
            println!("配置信息:");
            app_config.display();

            // 显示系统环境变量示例
            if let Some(home) = AppConfig::get_system_env("HOME") {
                println!("\n系统HOME目录: {}", home);
            }
        }
        None => {
            // 主处理流程
            println!("处理输入文件: {}", cli.input);

            // 读取文件
            let content = FileProcessor::read_input(&cli.input)?;

            if cli.verbose > 0 {
                println!("文件大小: {} 字节", content.len());
                println!("行数: {}", content.lines().count());
            }

            // 处理内容
            let result = FileProcessor::process_content(&content, &app_config.mode)?;

            // 输出结果到标准输出
            println!("\n{}", result);

            println!("\n处理完成!");
        }
    }

    Ok(())
}

fn main() {
    // 运行主逻辑并处理错误
    if let Err(e) = run() {
        // 输出错误到标准错误流
        eprintln!("错误: {}", e);

        // 根据错误类型退出
        let exit_code = e.exit_code();
        eprintln!("程序退出，退出码: {}", exit_code);

        process::exit(exit_code);
    }

    // 成功退出
    process::exit(exit_codes::SUCCESS);
}
