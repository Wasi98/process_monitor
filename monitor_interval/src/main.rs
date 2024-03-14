use std::env;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::thread;
use std::time::{Duration, SystemTime};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::{Local, format::strftime::StrftimeItems, Timelike};
use serde_json;

// struct to represent a monitor
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
    result: Option<Result>, 
    code: String,
}

//struct to represent the Result
#[derive(Debug, Deserialize, Serialize)]
struct Result {
    value: Option<i32>,
    processed_at: Option<i64>,
}

// Defining a struct to represent the JSON structure
#[derive(Debug, Deserialize, Serialize)]
struct MonitorsJson {
    monitors: Vec<Monitor>,
}

// Function for command-line arguments
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

// Function to update monitors
fn update_monitors(monitors: Arc<Mutex<Vec<Monitor>>>, running: Arc<AtomicBool>) {
    while running.load(Ordering::Relaxed) {
        let mut monitors = monitors.lock().unwrap();
        for monitor in monitors.iter_mut() {
            let processed_at = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
            monitor.result = Some(Result {
                value: Some(rand::random::<i32>() % 100), // Random value for demonstration
                processed_at: Some(processed_at),
            });
        }
        drop(monitors); // Release the lock before sleeping
        thread::sleep(Duration::from_secs(30)); // Update every 30 seconds
    }
}

// Function to store monitors as JSON files
fn store_monitors(monitors: Arc<Mutex<Vec<Monitor>>>, running: Arc<AtomicBool>) {
    let mut last_minute = 0;
    while running.load(Ordering::Relaxed) {
        let monitors_clone = monitors.clone(); // Clone monitors
        let current_time = Local::now(); // Get current local time
        let minute = current_time.minute();
        
        if minute != last_minute {
            let items = StrftimeItems::new("%-I_%M%P"); // Format hour, minute, and am/pm
            let filename = format_time_with_suffix("%-I:%M%P", items, "_monitors");
            
            let mut file = match File::create(&filename) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error creating file: {}", err);
                    continue;
                }
            };
            let monitors = monitors_clone.lock().unwrap(); // Lock the cloned value
            match serde_json::to_writer_pretty(&mut file, &*monitors) {
                Ok(_) => println!("Monitors stored in file: {}", filename),
                Err(err) => eprintln!("Error writing to file {}: {}", filename, err),
            }
            drop(monitors); // Release the lock before sleeping
            last_minute = minute;
        }
        thread::sleep(Duration::from_secs(60)); // Sleep for 1 minute
    }
}

fn format_time_with_suffix(_format: &str, items: StrftimeItems, suffix: &str) -> String {
    let formatted_time = format!("{}", Local::now().format_with_items(items.clone()));
    let formatted_time = formatted_time.replace(":", "_");
    format!("{}_{}{}", formatted_time, suffix, ".json")
}

// Function to process monitors
fn process_monitors(monitors: Arc<Mutex<Vec<Monitor>>>, duration: Duration) {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone_update = running.clone();
    let running_clone_store = running.clone();
    let update_monitors_arc = monitors.clone();
    let store_monitors_arc = monitors.clone();

    // Spawn threads for updating and storing monitors
    let update_handle = thread::spawn(move || {
        update_monitors(update_monitors_arc, running_clone_update);
    });
    let store_handle = thread::spawn(move || {
        store_monitors(store_monitors_arc, running_clone_store);
    });

    // Sleep for the specified duration before terminating
    thread::sleep(duration);

    // Terminate threads
    running.store(false, Ordering::Relaxed); 
    update_handle.join().unwrap();
    store_handle.join().unwrap();
}

fn main() {
    // Parsing CLI arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file");
        return;
    }

    // Extract monitor file path
    let monitor_file_path = process_command_line_args(args.clone());
    match monitor_file_path {
        Some(path) => {
            println!("Path to monitor file: {}", path);

            // Load and process JSON file
            let file = match File::open(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error opening file: {}", err);
                    return;
                }
            };
            let reader = std::io::BufReader::new(file);
            let monitors_json: MonitorsJson = match serde_json::from_reader(reader) {
                Ok(json) => json,
                Err(err) => {
                    eprintln!("Error parsing JSON: {}", err);
                    return;
                }
            };
            
            // Convert monitors to Arc<Mutex<_>> for thread safety
            let monitors_arc = Arc::new(Mutex::new(monitors_json.monitors));

            // Invoke process_monitors function
            let duration = Duration::from_secs(5 * 60); // Five minutes duration
            process_monitors(monitors_arc, duration);
        }
        None => return,
    }
}
