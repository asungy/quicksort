#![allow(
    clippy::all,
    missing_debug_implementations,
)]

use clap::{ Arg, Command };
use serde_json::Value;

const NAME: &str = env!("CARGO_BIN_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const JSON_ARG: &str = "json";

fn cli() -> Command {
    Command::new(NAME)
        .version(VERSION)
        .about("Performs quicksort on list of unsigned integers")
        .args([
            Arg::new(JSON_ARG)
                .short('j')
                .help("JSON file containing list to sort")
                .required(true),
        ])
}

fn get_num_list() -> anyhow::Result<Vec<u64>> {
    let mut matches = cli().get_matches();

    let file_path = matches.get_one::<String>(JSON_ARG).expect("Expected json file argument");
    let data = std::fs::read_to_string(file_path)?;

    let num_list: Value = serde_json::from_str(&data)?;

    let num_list = num_list.get("num_list").expect("\"num_list\" key does not exist")
        .as_array().expect("Could not parse array")
        .iter().map(|e| e.as_u64().unwrap())
            .collect();

    Ok(num_list)
}

fn quicksort(list: &mut Vec<u64>) {

}

fn main() -> anyhow::Result<()> {
    let num_list = get_num_list()?;

    println!("{num_list:?}");

    Ok(())
}
