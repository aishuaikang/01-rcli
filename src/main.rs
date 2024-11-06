use clap::Parser;
use rcli::{
    opts::{Opts, SubCommand},
    process::{csv::process_csv, gen_pass::process_gen_pass},
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
        SubCommand::GenPass(opts) => process_gen_pass(
            opts.length,
            opts.lowercase,
            opts.uppercase,
            opts.number,
            opts.symbol,
        )?,
    }
    Ok(())
}
