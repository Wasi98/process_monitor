#![allow(unused_variables)]
use std::env;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

// Define a struct to represent a monitor
#[derive(Debug, Deserialize, Serialize)]
struct Monitor {
    #[serde(rename = "monitor_id")]
    monitor_id: Option<u32>,
    name: String,
    #[serde(default)]
    #[serde(rename = "script")]
    script: Option<String>,
    #[serde(rename = "type")]
    monitor_type: Option<String>,
    result: Option<Result>, // Include the Result struct as an option
    code: String,
}

// Define a struct to represent the Result
#[derive(Debug, Deserialize, Serialize)]
struct Result {
    value: Option<i32>,
    processed_at: Option<i64>,
}

// Define a struct to represent the JSON structure
#[derive(Debug, Deserialize, Serialize)]
struct MonitorsJson {
    monitors: Vec<Monitor>,
}

// Function to process command-line arguments
fn process_command_line_args(args: Vec<String>) -> Option<String> {
    if args.len() != 3 {
        println!("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file");
        return None;
    } else if args[1] != "-monitorFile" {
        println!("Invalid argument: {}", args[1]);
        return None;
    } else {
        return Some(args[2].clone());
    }
}


fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file");
        return;
    }

    // Extract monitor file path
    let monitor_file_path = process_command_line_args(args);
    match monitor_file_path {
        Some(path) => {
            println!("Path to monitor file: {}", path);
            // Now you can proceed with using monitor_file_path in your program.

            // Load and process JSON file
            let file = match File::open(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error opening file: {}", err);
                    return;
                }
            };
            let reader = BufReader::new(file);
            let monitors_json: MonitorsJson = match serde_json::from_reader(reader) {
                Ok(json) => json,
                Err(err) => {
                    eprintln!("Error parsing JSON: {}", err);
                    return;
                }
            };
            // println!("this is monitor's json file: {:?}", monitors_json);
            // Print monitor details
            // for monitor in monitors_json.monitors {
            //     println!("Monitor Name: {}", monitor.name);
            //     println!("Monitor ID: {:?}", monitor.monitor_id);
            //     println!("Monitor Script: {:?}", monitor.script);
            //     println!("Monitor Type: {:?}", monitor.monitor_type);
            //     println!("Monitor Result: {:?}", monitor.result);
            // }
        }
        None => return,
    }
}

//command
//cargo run -- -monitorFile C:\Users\User\Desktop\process_monitor\assets\monitors.json
