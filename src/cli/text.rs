use std::{path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use tokio::fs;

use crate::{
    process::text::{
        process_decrypt, process_encrypt, process_gen_key, process_sign, process_verify,
    },
    CmdExector,
};

use super::{verify_file, verify_path};

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExector)]
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

// impl CmdExector for TextSubCommand {
//     async fn execute(&self) -> anyhow::Result<()> {
//         match self {
//             TextSubCommand::Sign(opts) => opts.execute().await,
//             TextSubCommand::Verify(opts) => opts.execute().await,
//             TextSubCommand::Generate(opts) => opts.execute().await,
//             TextSubCommand::Encrypt(opts) => opts.execute().await,
//             TextSubCommand::Decrypt(opts) => opts.execute().await,
//         }
//     }
// }

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

impl CmdExector for TextSignOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let signed = process_sign(&self.input, &self.key, self.format)?;
        println!("{}", signed);
        Ok(())
    }
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

impl CmdExector for TextVerifyOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let verified = process_verify(&self.input, &self.key, &self.sig, self.format)?;
        println!("{}", verified);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

impl CmdExector for TextKeyGenerateOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let key = process_gen_key(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3");
                fs::write(name, &key[0]).await?;
            }
            TextSignFormat::Ed25519 => {
                fs::write(self.output.join("ed25519.sk"), &key[0]).await?;
                fs::write(self.output.join("ed25519.pk"), &key[1]).await?;
            }
            TextSignFormat::ChaCha20Poly1305 => {
                let name = self.output.join("chacha20poly1305.key");
                fs::write(name, &key[0]).await?;
            }
        };
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
}

impl CmdExector for TextEncryptOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let encrypted = process_encrypt(&self.input, &self.key)?;
        println!("{}", encrypted);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
}

impl CmdExector for TextDecryptOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let decrypted = process_decrypt(&self.input, &self.key)?;
        println!("{}", decrypted);
        Ok(())
    }
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
