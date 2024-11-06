use std::io::Read;

use crate::cli::base64::Base64Format;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let buf = read_data(input)?;

    let encodeed = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encodeed);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let buf = read_data(input)?;

    let decodeed = match format {
        Base64Format::Standard => STANDARD.decode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf),
    }?;
    // TODO: 解码后的数据可能不是字符串（但在这个例子中，我们假设)
    let decodeed = String::from_utf8(decodeed)?;
    println!("{}", decodeed);
    Ok(())
}

fn read_data(input: &str) -> anyhow::Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let buf = buf.trim_ascii().to_vec();
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        let result = process_encode(input, format);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtrues/base64.txt";
        let format = Base64Format::Standard;
        let result = process_decode(input, format);
        assert!(result.is_ok());
    }
}
