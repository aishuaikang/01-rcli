// use std::fs;

use std::fs;

use csv::Reader;
// use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts::OutputFormat;

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct Player {
//     name: String,
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit_number: u8,
// }

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let mut result = Vec::with_capacity(headers.len());
    for item in reader.records() {
        let record = item?;
        let json = headers.iter().zip(record.iter()).collect::<Value>();
        result.push(json);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&result)?,
        OutputFormat::Yaml => serde_yaml::to_string(&result)?,
        // OutputFormat::Toml => todo!(),
    };
    fs::write(output, content)?;
    Ok(())
}
