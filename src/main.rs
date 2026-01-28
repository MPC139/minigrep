use minigrep::*;
use std::{env, error::Error, fs::File};

/// The entry point of the minigrep application.
///
/// Parses command-line arguments, opens the target file, and executes the search logic.
///
/// # Panics
///
/// This function does not panic. All errors are handled via the `Result` return type.
///
/// # Errors
///
/// Returns an error if:
/// * Argument parsing fails (e.g., missing query or file path).
/// * The specified file cannot be opened.
/// * The search process encounters an I/O error.
///
/// # Safety
///
/// This function is safe and follows idiomatic Rust error handling.
///
/// # Examples
///
/// While `main` is typically called by the system, it represents the following workflow:
/// ```bash
/// cargo run -- "search_query" path/to/file.txt
/// ```
fn main() -> Result<(), Box<dyn Error>> {
    //The same of:
    // let mut f = match File::open(path_file) {
    // Ok(file) => file,         // If successful, extract the File
    // Err(e) => return Err(e.into()), // If failed, RETURN the error from the function NOW
    // };
    let config = Config::build(env::args())?;
    let f = File::open(&config.file_path)?;
    mygrep(&f, &config)?;
    Ok(())
}
