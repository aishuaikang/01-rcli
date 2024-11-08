pub mod base64;
pub mod csv;
pub mod gen_pass;
pub mod text;

use std::path::{Path, PathBuf};

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
    #[command(subcommand, about = "文本签名或验证")]
    Text(text::TextSubCommand),
}

fn verify_file(filepath: &str) -> anyhow::Result<String> {
    if filepath == "-" || Path::new(filepath).exists() {
        Ok(filepath.to_string())
    } else {
        anyhow::bail!("文件不存在")
    }
}

fn verify_path(path: &str) -> anyhow::Result<PathBuf> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(p.into())
    } else {
        anyhow::bail!("路径不存在或不是目录")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert!(verify_file("-").is_ok());
        assert!(verify_file("*").is_err());
        assert!(verify_file("Cargo.toml").is_ok());
        assert!(verify_file("Cargo.toml1").is_err());
    }
}
