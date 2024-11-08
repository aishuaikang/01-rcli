use std::{path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};

use super::{verify_file, verify_path};

#[derive(Debug, Subcommand)]
pub enum TextSubCommand {
    #[command(about = "使用私钥/共享密钥签名消息")]
    Sign(TextSignOpts),
    #[command(about = "验证已签名消息")]
    Verify(TextVerifyOpts),
    #[command(about = "生成密钥")]
    Generate(TextKeyGenerateOpts),
    #[command(about = "ChaCha20Poly1305 加密消息")]
    Encrypt(TextEncryptOpts),
    #[command(about = "ChaCha20Poly1305 解密消息")]
    Decrypt(TextDecryptOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub sig: String,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
    ChaCha20Poly1305,
}

fn parse_format(value: &str) -> anyhow::Result<TextSignFormat> {
    value.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            "chacha20poly1305" => Ok(TextSignFormat::ChaCha20Poly1305),
            _ => anyhow::bail!("不支持的格式"),
        }
    }
}
