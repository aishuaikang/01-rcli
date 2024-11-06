use clap::Parser;
use rcli::{
    opts::{Opts, SubCommand},
    process::process_csv,
};

fn main() -> anyhow::Result<()> {
    let args = Opts::parse();
    match args.command {
        SubCommand::Csv(opts) => {
            let output = opts
                .output
                .unwrap_or_else(|| format!("output.{}", opts.format));
            process_csv(&opts.input, &output, opts.format)?
        }
    }
    Ok(())
}
