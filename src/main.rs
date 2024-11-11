use std::fs;

use clap::Parser;
use rcli::{
    cli::{
        base64::Base64SubCommand,
        http::HttpSubCommand,
        text::{TextSignFormat, TextSubCommand},
        Opts, SubCommand,
    },
    process::{
        base64::{process_decode, process_encode},
        csv::process_csv,
        gen_pass::process_gen_pass,
        http::process_http,
        text::{process_decrypt, process_encrypt, process_gen_key, process_sign, process_verify},
    },
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Opts::parse();
    match args.command {
        SubCommand::Csv(opts) => {
            let output = opts
                .output
                .unwrap_or_else(|| format!("output.{}", opts.format));
            process_csv(&opts.input, &output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            let password = process_gen_pass(
                opts.length,
                opts.lowercase,
                opts.uppercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", password);

            let password_strength = zxcvbn(&password, &[]);
            eprintln!(
                "密码强度：{:?}(max 4)",
                Into::<u8>::into(password_strength.score())
            );
        }
        SubCommand::Base64(sub_command) => match sub_command {
            Base64SubCommand::Encode(opts) => {
                let encodeed = process_encode(&opts.input, opts.format)?;
                println!("{}", encodeed);
            }
            Base64SubCommand::Decode(opts) => {
                let decodeed = process_decode(&opts.input, opts.format)?;
                println!("{}", String::from_utf8(decodeed)?);
            }
        },
        SubCommand::Text(sub_command) => match sub_command {
            TextSubCommand::Sign(opts) => {
                let signed = process_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_gen_key(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                    TextSignFormat::ChaCha20Poly1305 => {
                        let name = opts.output.join("chacha20poly1305.key");
                        fs::write(name, &key[0])?;
                    }
                }
            }
            TextSubCommand::Encrypt(opts) => {
                let encrypted = process_encrypt(&opts.input, &opts.key)?;
                println!("{}", encrypted);
            }
            TextSubCommand::Decrypt(opts) => {
                let decrypted = process_decrypt(&opts.input, &opts.key)?;
                println!("{}", decrypted);
            }
        },
        SubCommand::Http(sub_command) => match sub_command {
            HttpSubCommand::Serve(opts) => {
                process_http(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}
