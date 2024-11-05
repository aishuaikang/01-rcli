use clap::{Parser, Subcommand};
use std::path::Path;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filepath: &str) -> Result<String, &'static str> {
    if Path::new(filepath).exists() {
        Ok(filepath.to_string())
    } else {
        Err("文件不存在")
    }
}
