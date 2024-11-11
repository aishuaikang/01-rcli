use std::path::PathBuf;

use clap::{Parser, Subcommand};

use super::verify_path;

#[derive(Debug, Subcommand)]
pub enum HttpSubCommand {
    #[command(about = "通过HTTP服务文件")]
    Serve(ServeOpts),
}

#[derive(Debug, Parser)]
pub struct ServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}
