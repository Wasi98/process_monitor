use serde::{Serialize, Deserialize};
use rand::{Rng, thread_rng};
use std::fs;

#[derive(Serialize, Deserialize)]
struct MonitorData {
    monitors: Vec<Monitor>,
}

#[derive(Serialize, Deserialize)]
struct Monitor {
    name: String,
    script: Option<String>,
    result: Option<Result>,
    code: String,
}

#[derive(Serialize, Deserialize)]
struct Result {
    value: i64,
    processed_at: i64,
}

fn main() {
    //reading file path
    let file_path = r#"C:\Users\User\Desktop\process_monitor\assets\monitors.json"#;

    let monitors_json = match fs::read_to_string(&file_path) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Failed to read JSON file: {}", err);
            return;
        }
    };

    let mut monitor_data: MonitorData = match serde_json::from_str(&monitors_json) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Failed to deserialize JSON: {}", err);
            return;
        }
    };

    //generating random results
    let mut rng = thread_rng();
    for monitor in &mut monitor_data.monitors {
        if monitor.result.is_none() {
            monitor.result = Some(Result {
                value: rng.gen_range(0..100),
                processed_at: chrono::Utc::now().timestamp(), 
            });
        } else {
            // Update existing result with new data
            if let Some(result) = &mut monitor.result {
                result.value = rng.gen_range(0..100); 
                result.processed_at = chrono::Utc::now().timestamp(); 
            }
        }
    }

     // Serializing the modified monitor data back to JSON
    let new_monitors_json = match serde_json::to_string_pretty(&monitor_data) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Failed to serialize JSON: {}", err);
            return;
        }
    };

    // Writing the serialized JSON data to a new file
    let new_file_path = r#"C:\Users\User\Desktop\process_monitor\j_conv\updated_monitors.json"#; //C:\Users\User\Desktop\process_monitor\j_conv
    match fs::write(new_file_path, &new_monitors_json) {
        Ok(()) => println!("Updated JSON data written to file: {}", new_file_path),
        Err(err) => eprintln!("Failed to write JSON data to file: {}", err),
    }
}
