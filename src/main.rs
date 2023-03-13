mod models;

use crate::models::{ApiResponse, Crate, Length};
use std::vec::Vec;

use clap::{arg, command};

const ID_NAME: &str = "name";
const ROOT_PADDING: &str = "  ";

fn main() -> Result<(), String> {
    let matches = command!()
        .arg(arg!(<NAME>).id(ID_NAME).required(true).help("crates name"))
        .get_matches();
    let name: &String = matches.get_one(ID_NAME).unwrap();
    let crates = get_crates(name)?;
    let length = parse_length(&crates);

    show_columns(&length);
    for c in crates {
        show_crate(&c, &length);
    }

    Ok(())
}

fn get_crates(name: &str) -> Result<Vec<Crate>, String> {
    let url = format!("https://crates.io/api/v1/crates?q={}", name);
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(url)
        .header("User-Agent", "check-crates-client")
        .send()
        .map_err(|e| format!("failed to call api :{}", e))?;

    let text = response
        .text()
        .map_err(|e| format!("failed to parse response: {e}"))?;
    let api_response: ApiResponse =
        serde_json::from_str(&text).map_err(|e| format!("failed to deserialize: {e}"))?;

    let result: Vec<Crate> = api_response.crates;

    Ok(result)
}

fn parse_length(crates: &Vec<Crate>) -> Length {
    let mut list_id: Vec<usize> = crates.iter().map(|c| c.id.len()).collect();
    let mut list_name: Vec<usize> = crates.iter().map(|c| c.name.len()).collect();
    let mut list_max_version: Vec<usize> = crates.iter().map(|c| c.max_version.len()).collect();
    let mut list_max_stable_version: Vec<usize> = crates
        .iter()
        .map(|c| &c.max_stable_version)
        .filter(|v| v.is_some())
        .map(|v| v.clone().unwrap().len())
        .collect();
    let mut list_updated_at: Vec<usize> = crates.iter().map(|c| c.updated_at.len()).collect();

    list_id.push("id".len());
    list_name.push("name".len());
    list_max_version.push("max_version".len());
    list_max_stable_version.push("max_stable_version".len());
    list_updated_at.push("updated_at".len());

    Length {
        id: list_id.iter().max().unwrap().clone(),
        name: list_name.iter().max().unwrap().clone(),
        max_version: list_max_version.iter().max().unwrap().clone(),
        max_stable_version: list_max_stable_version.iter().max().unwrap().clone(),
        updated_at: list_updated_at.iter().max().unwrap().clone(),
    }
}

fn set_padding(t: &str, s: usize) -> String {
    let repeat = s - t.len();
    format!("{}{}", " ".repeat(repeat), t)
}

fn show_columns(length: &Length) {
    println!(
        "{}{}  {}  {}",
        ROOT_PADDING,
        set_padding("name", length.name),
        set_padding("max_stable_version", length.max_stable_version),
        set_padding("updated_at", length.updated_at)
    );
    println!(
        "{}{}  {}  {}",
        ROOT_PADDING,
        "=".repeat(length.name),
        "=".repeat(length.max_stable_version),
        "=".repeat(length.updated_at)
    );
}

fn show_crate(c: &Crate, length: &Length) {
    let max_stable_version = match &c.max_stable_version {
        Some(t) => t.clone(),
        None => "".to_string(),
    };
    println!(
        "{}{}  {}  {}",
        ROOT_PADDING,
        set_padding(&c.name, length.name),
        set_padding(&max_stable_version, length.max_stable_version),
        set_padding(&c.updated_at, length.updated_at)
    );
}
