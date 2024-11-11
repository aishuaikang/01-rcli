use std::{fmt::Display, str::FromStr};

use clap::{Parser, Subcommand};

use crate::{
    process::base64::{process_decode, process_encode},
    CmdExector,
};

use super::verify_file;

#[derive(Debug, Subcommand)]
pub enum Base64SubCommand {
    #[command(about = "Base64编码")]
    Encode(Base64EncodeOpts),
    #[command(about = "Base64解码")]
    Decode(Base64DecodeOpts),
}

impl CmdExector for Base64SubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => opts.execute().await,
            Base64SubCommand::Decode(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExector for Base64EncodeOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let encodeed = process_encode(&self.input, self.format)?;
        println!("{}", encodeed);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExector for Base64DecodeOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let decodeed = process_decode(&self.input, self.format)?;
        println!("{}", String::from_utf8(decodeed)?);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(value: &str) -> anyhow::Result<Base64Format> {
    value.parse()
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => anyhow::bail!("不支持的格式"),
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
