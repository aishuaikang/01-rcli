use clap::Parser;
use rcli::{
    cli::{base64::Base64SubCommand, Opts, SubCommand},
    process::{
        base64::{process_decode, process_encode},
        csv::process_csv,
        gen_pass::process_gen_pass,
    },
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
        SubCommand::Base64(sub_command) => match sub_command {
            Base64SubCommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
            Base64SubCommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
        },
    }
    Ok(())
}
