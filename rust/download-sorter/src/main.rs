use std::fs;
use std::ffi::OsStr;
use std::thread;
use std::time::Duration;

fn main() {
    let downloads_dir = dirs::download_dir().expect("Could not find the Downloads directory");
    let sorted_dir = downloads_dir.join("_SORTED");

    // Create the _SORTED directory and subdirectories
    fs::create_dir_all(&sorted_dir).expect("Could not create _SORTED directory");
    let subdirs = vec!["Images", "Documents", "Executables", "Archives", "Misc"];
    for subdir in &subdirs {
        fs::create_dir_all(sorted_dir.join(subdir)).expect("Could not create subdirectory");
    }

    loop {
        // Iterate over the files in the Downloads directory
        for entry in fs::read_dir(&downloads_dir).expect("Could not read Downloads directory") {
            let entry = entry.expect("Could not read entry");
            let path = entry.path();
            if path.is_file() {
                let extension = path.extension().and_then(OsStr::to_str).unwrap_or("").to_lowercase();
                let destination = match extension.as_str() {
                    "jpg" | "jpeg" | "png" | "gif" => sorted_dir.join("Images"),
                    "pdf" | "doc" | "docx" | "txt" => sorted_dir.join("Documents"),
                    "exe" | "msi" => sorted_dir.join("Executables"),
                    "zip" | "rar" | "7z" => sorted_dir.join("Archives"),
                    _ => sorted_dir.join("Misc"),
                };
                fs::rename(&path, destination.join(path.file_name().unwrap())).expect("Could not move file");
            }
        }
        // Sleep for 1 minute
        thread::sleep(Duration::from_secs(60));
    }
}