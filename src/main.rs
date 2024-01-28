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

fn partition(list: &mut Vec<u64>, low: usize, high: usize) -> (usize, usize) {
    let pivot = list[(low + high) / 2];
    let mut lesser_index: usize = low;
    let mut equal_index: usize = low;
    let mut greater_index: usize = high;

    while equal_index <= greater_index {
        if list[equal_index] < pivot {
            list.swap(equal_index, lesser_index);
            lesser_index += 1;
            equal_index += 1;
        } else if list[equal_index] > pivot {
            list.swap(equal_index, greater_index);
            greater_index = greater_index.checked_sub(1).unwrap();
        } else {
            equal_index += 1;
        }
    }

    (lesser_index, greater_index)
}

fn quicksort(list: &mut Vec<u64>, low: usize, high: usize) {
    if low < high {
        let (lesser_index, greater_index) = partition(list, low, high);
        quicksort(list, low, lesser_index.checked_sub(1).unwrap_or(0));
        quicksort(list, greater_index + 1, high);
    }
}

fn main() -> anyhow::Result<()> {
    let mut num_list = get_num_list()?;

    let low = 0;
    let high = num_list.len() - 1;
    println!("Unsorted: {num_list:?}");
    quicksort(&mut num_list, low, high);
    println!("Sorted: {num_list:?}");

    let mut i = 1;
    while i < num_list.len() {
        if num_list[i-1] > num_list[i] {
            println!("Assertion failed at index: {i}. {} > {}", num_list[i-1], num_list[i]);
            assert!(false);
        }
        i += 1;
    }

    Ok(())
}
