use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn write_csv(directory: &Path) -> io::Result<()> {
    // Create a CSV file in the current directory
    let csv_path = directory.join("files.csv");
    let mut csv_file = File::create(&csv_path)?;

    // Collect and sort entries alphabetically
    let mut entries: Vec<_> = fs::read_dir(directory)?
        .filter_map(|entry| entry.ok().map(|e| e.file_name().into_string().unwrap_or_default()))
        .collect();
    entries.sort();

    // Write each entry to the CSV
    for entry in entries {
        writeln!(csv_file, "{}", entry)?;
    }

    println!("Created CSV: {}", csv_path.display());
    Ok(())
}

fn main() {
    // Get the current working directory
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Generate the CSV file for the current directory
    if let Err(e) = write_csv(&current_dir) {
        eprintln!("Error: {}", e);
    }
}
