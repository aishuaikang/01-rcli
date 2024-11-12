use std::time::Duration;

use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use humantime::parse_duration;

use crate::{
    process::jwt::{process_jwt_sign, process_jwt_verify},
    CmdExector,
};

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(about = "JWT签名")]
    Sign(JwtSignOpts),
    #[command(about = "JWT验证")]
    Verify(JwtVerifyOpts),
}

// RUST_LOG=trace cargo run -- jwt sign --sub acme --aud device1 --exp 14d --key xiaoai
#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long, default_value = "1h", value_parser = parse_duration)]
    pub exp: Duration,
    #[arg(long)]
    pub key: String,
}

impl CmdExector for JwtSignOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let token_str = process_jwt_sign(&self.sub, &self.aud, self.exp, &self.key)?;
        println!("{}", token_str);
        Ok(())
    }
}

// RUST_LOG=trace cargo run -- jwt verify --token xxx --key xiaoai
#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long)]
    pub token: String,
    #[arg(long)]
    pub key: String,
}

impl CmdExector for JwtVerifyOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let claims = process_jwt_verify(&self.token, &self.key)?;
        println!("{:#?}", claims);

        Ok(())
    }
}
