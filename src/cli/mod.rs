pub mod base64;
pub mod csv;
pub mod gen_pass;

use std::path::Path;

use base64::Base64SubCommand;
use clap::{Parser, Subcommand};
use csv::CsvOpts;
use gen_pass::GenPassOpts;

#[derive(Debug, Parser)]
#[command(version, about = "一个处理CSV文件的工具")]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(about = "显示CSV，或将它转换为其他格式")]
    Csv(CsvOpts),
    #[command(about = "生成密码")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64编码或解码")]
    Base64(Base64SubCommand),
}

fn verify_input_file(filepath: &str) -> anyhow::Result<String> {
    if filepath == "-" || Path::new(filepath).exists() {
        Ok(filepath.to_string())
    } else {
        anyhow::bail!("文件不存在")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert!(verify_input_file("-").is_ok());
        assert!(verify_input_file("*").is_err());
        assert!(verify_input_file("Cargo.toml").is_ok());
        assert!(verify_input_file("Cargo.toml1").is_err());
    }
}
