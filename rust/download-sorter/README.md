# Download Sorter

This Rust program automatically sorts files in your Downloads directory into categorized subdirectories. The program continuously monitors the Downloads directory and moves files into appropriate subdirectories based on their file extensions.

## Features

- Automatically creates a `_SORTED` directory in your Downloads folder.
- Creates subdirectories for different file types:
  - Images (`jpg`, `jpeg`, `png`, `gif`)
  - Documents (`pdf`, `doc`, `docx`, `txt`)
  - Executables (`exe`, `msi`)
  - Archives (`zip`, `rar`, `7z`)
  - Miscellaneous (all other file types)
- Continuously monitors the Downloads directory and sorts new files every minute.

## Usage

1. Ensure you have Rust installed on your system. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone this repository and navigate to the `download-sorter` directory.
3. Build the project using Cargo:
    ```sh
    cargo build --release
    ```
4. Run the executable:
    ```sh
    ./target/release/download-sorter
    ```

## Code Overview

The main logic of the program is in the [main.rs](http://_vscodecontentref_/0) file. The program performs the following steps:

1. Determines the Downloads directory using the [dirs](http://_vscodecontentref_/1) crate.
2. Creates a `_SORTED` directory and subdirectories for different file types.
3. Enters an infinite loop where it:
   - Reads the contents of the Downloads directory.
   - Moves files to the appropriate subdirectory based on their file extension.
   - Sleeps for 1 minute before repeating the process.

## Dependencies

- [dirs](http://_vscodecontentref_/2): For determining the Downloads directory.
- [std](http://_vscodecontentref_/3): Standard library for file system operations, threading, and time management.