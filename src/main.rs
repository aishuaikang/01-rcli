use clap::Parser;
use rcli::{
    opts::{Opts, SubCommand},
    process::process_csv,
};

fn main() -> anyhow::Result<()> {
    let args = Opts::parse();
    match args.command {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
