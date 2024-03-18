use csv::{ReaderBuilder, StringRecord, Writer};
use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use std::process;

const FILENAME: &str = "/Users/congding/courses/537/ostep-projects/initial-kv/data.txt";

const DELETE_MARKER: &str = "DELETED";

fn put(k: i32, v: String) {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(FILENAME)
        .unwrap();

    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&[k.to_string(), v]).unwrap();
    wtr.flush().unwrap();
}

fn get(k: i32) {
    // If the file does not exist, return early.
    if !Path::new(FILENAME).exists() {
        println!("{} not found", k);
        return;
    }

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(FILENAME)
        .unwrap();

    // Read all records into a vector
    let records: Vec<StringRecord> = rdr.records().filter_map(Result::ok).collect();
    let mut found = false;
    // Iterate over the records in reverse until you find a matching key.
    // This is because the last record with the matching key is the most recent.
    for record in records.iter().rev() {
        let current_k = record[0].parse::<i32>().unwrap();
        let current_v = record[1].to_string();
        if current_k == k && current_v != DELETE_MARKER {
            println!("{},{}", current_k, current_v);
            found = true;
            break; // If the key is found, exit the loop.
        } else if current_k == k && current_v == DELETE_MARKER {
            break;
        }
    }

    if !found {
        println!("{} not found", k);
    }
}

fn clear() {
    if Path::new(FILENAME).exists() {
        std::fs::remove_file(FILENAME).unwrap();
    }
}

fn delete(k: i32) {
    put(k, DELETE_MARKER.to_string());
}

fn all() {
    if !Path::new(FILENAME).exists() {
        return;
    }

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(FILENAME)
        .unwrap();
    let records: Vec<StringRecord> = rdr.records().filter_map(Result::ok).collect();

    // Replay the records into a hashmap to remove duplicates
    let mut hashmap: HashMap<i32, String> = HashMap::<i32, String>::new();
    for record in records.iter() {
        let k = record[0].parse::<i32>().unwrap();
        let v = record[1].to_string();
        hashmap.insert(k, v);
    }

    for (k, v) in hashmap.iter() {
        if v == DELETE_MARKER {
            continue;
        }
        println!("{},{}", k, v);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Loop through each argument provided
    for arg in args.iter() {
        // Split the argument by comma to get individual values
        let values: Vec<&str> = arg.split(',').collect();
        match values[0] {
            "p" => {
                if values.len() != 3 {
                    println!("p command requires 2 arguments");
                    process::exit(1);
                }
                let k = values[1].parse::<i32>().unwrap();
                let v = values[2].to_string();
                if v.contains(",") {
                    println!("value cannot contain comma");
                    process::exit(1);
                }
                if v == DELETE_MARKER {
                    println!("value cannot be {}", DELETE_MARKER);
                    process::exit(1);
                }
                put(k, v);
            }
            "g" => {
                if values.len() != 2 {
                    println!("g command requires 1 argument");
                    process::exit(1);
                }
                let k = values[1].parse::<i32>().unwrap();
                get(k);
            }
            "d" => {
                if values.len() != 2 {
                    println!("d command requires 1 argument");
                    process::exit(1);
                }
                let k = values[1].parse::<i32>().unwrap();
                delete(k);
            }
            "c" => {
                if values.len() != 1 {
                    println!("c command requires 0 arguments");
                    process::exit(1);
                }
                clear();
            }
            "a" => {
                if values.len() != 1 {
                    println!("a command requires 0 arguments");
                    process::exit(1);
                }
                all();
            }
            _ => {
                println!("bad command");
            }
        }
    }
}
