use anyhow::Ok;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut ret = Vec::with_capacity(100);
    let mut reader = Reader::from_path(input)?;
    for result in reader.deserialize() {
        let player: Player = result?;
        ret.push(player);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
