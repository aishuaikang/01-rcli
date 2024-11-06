use clap::{Parser, Subcommand};
use std::{fmt::Display, path::Path, str::FromStr};

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
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    // Toml,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, default_value_t = OutputFormat::Json,value_parser = parse_format)]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub number: bool,
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

fn verify_input_file(filepath: &str) -> anyhow::Result<String> {
    if Path::new(filepath).exists() {
        Ok(filepath.to_string())
    } else {
        anyhow::bail!("文件不存在")
    }
}

fn parse_format(format: &str) -> anyhow::Result<OutputFormat> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            // OutputFormat::Toml => "toml",
        }
    }
}

impl TryFrom<&str> for OutputFormat {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => OutputFormat::Toml,
            _ => anyhow::bail!("不支持的格式"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}
