use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use chrono::Local;

fn main() {
    // Collect command line arguments, skipping the program name
    let args: Vec<String> = env::args().skip(1).collect();
    
    // If no arguments provided, print usage and exit
    if args.is_empty() {
        println!("Usage: projlog <message>");
        return;
    }

    // Join all arguments into a single message
    let message = args.join(" ");

    // Use "plog" as the log file
    let filename = "plog";

    // Get current timestamp
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Format log entry
    let log_entry = format!("[{}] {}\n", timestamp, message);

    // Append to file
    append_to_file(filename, &log_entry).expect("Failed to write to .proj file");
}

fn append_to_file(filename: &str, content: &str) -> std::io::Result<()> {
    let path = Path::new(filename);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)?;

    file.write_all(content.as_bytes())
}

