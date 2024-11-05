use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let records = reader
        .deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<Player>>();

    let json = serde_json::to_string_pretty(&records)?;
    fs::write(output, json)?;

    Ok(())
}
