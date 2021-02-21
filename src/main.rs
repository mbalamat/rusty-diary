use dirs;
use chrono::prelude::*;
use std::fs;
use std::env;
use std::process;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DiaryEntry {
    datetime: String,
    entry: String,
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.drain(0..1);
    if args.len() == 0 {
        println!("Nothing to do here. Come back when you have something to add to your diary. Have a nice day!");
        process::exit(0);
    }
    let entry = args.join(" ");
    let mut data_file = dirs::home_dir()
        .expect("Error in reading home_dir");
    data_file.push(".rusty-diary");
    // Create data dir if it doesn't exist
    fs::create_dir_all(&data_file)
        .expect("Error creating the diary data directory");
    data_file.push("data.json");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&data_file)
        .expect("Error while reading the diary data file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error while reading the diary data file contents");
    if contents.len() == 0 {
        // If we just created the data file it's empty
        // so change contents to a valid JSON array
        contents = "[]".to_string()
    }
    let mut entries: Vec<DiaryEntry> = serde_json::from_str(&contents)
        .expect("Error while parsing JSON data.");
    let now: DateTime<Utc> = Utc::now();
    entries.push(DiaryEntry {
        datetime: now.to_rfc3339(),
        entry
    });
    let output = serde_json::to_string(&entries)
        .expect("Error while converting entries to JSON");
    fs::write(data_file, output)
        .expect("Error while saving your entry");
    println!("Ok!");
}
