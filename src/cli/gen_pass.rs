use clap::Parser;

use crate::{process::gen_pass::process_gen_pass, CmdExector};
use zxcvbn::zxcvbn;

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

impl CmdExector for GenPassOpts {
    async fn execute(&self) -> anyhow::Result<()> {
        let password = process_gen_pass(
            self.length,
            self.lowercase,
            self.uppercase,
            self.number,
            self.symbol,
        )?;
        println!("{}", password);
        let password_strength = zxcvbn(&password, &[]);
        eprintln!(
            "密码强度：{:?}(max 4)",
            Into::<u8>::into(password_strength.score())
        );
        Ok(())
    }
}
