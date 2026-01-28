use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

pub struct Config {
    pub query: Vec<u8>,
    pub file_path: String,
}
//'static: This is the lifetime. 'static is a special lifetime in Rust that means "this data is valid for the entire duration of the program."
impl Config {
    /// Builds a `Config` instance from an iterator of strings (typically command-line arguments).
    ///
    /// # Panics
    ///
    /// This function does not panic.
    ///
    /// # Errors
    ///
    /// Returns an error string if:
    /// * The query string is missing.
    /// * The file path is missing.
    ///
    /// # Safety
    ///
    /// This function is safe to use and does not contain `unsafe` code.
    ///
    /// # Examples
    ///
    /// ```
    /// use minigrep::Config;
    /// let args = vec!["program_name".to_string(), "query".to_string(), "file.txt".to_string()];
    /// let config = Config::build(args.into_iter()).unwrap();
    /// assert_eq!(config.query, b"query");
    /// assert_eq!(config.file_path, "file.txt");
    /// ```
    pub fn build<T: Iterator<Item = String>>(mut args: T) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(i) => Vec::from(i.as_bytes()),
            None => return Err("Error to find query"),
        };
        let file_path = match args.next() {
            Some(i) => i,
            None => return Err("Error to find file path"),
        };

        Ok(Config { query, file_path })
    }
}

/// Searches for a query string within a file and prints matching lines to standard output
/// with the query highlighted in red.
///
/// # Panics
///
/// This function does not panic under normal conditions.
///
/// # Errors
///
/// Returns a `Box<dyn Error>` if:
/// * The file cannot be read.
/// * Standard output cannot be written to.
///
/// # Safety
///
/// This function is safe to use and handles I/O errors gracefully via the `Result` type.
///
/// # Examples
///
/// ```no_run
/// use minigrep::{Config, mygrep};
/// use std::fs::File;
///
/// let config = Config {
///     query: b"search_term".to_vec(),
///     file_path: "test.txt".to_string(),
/// };
/// let file = File::open(&config.file_path).unwrap();
/// mygrep(&file, &config).unwrap();
/// ```
pub fn mygrep(f: &File, config: &Config) -> Result<(), Box<dyn Error>> {
    let mut buffer = BufReader::new(f);
    let mut data: Vec<u8> = Vec::new();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    loop {
        data.clear();
        let bytes = buffer.read_until(b'\n', &mut data)?;
        if bytes == 0 {
            break;
        }

        let query_len = config.query.len();
        if query_len == 0 {
            continue;
        }

        let mut matches = Vec::new();
        let mut i = 0;
        // Find all non-overlapping matches
        if data.len() >= query_len {
            while i <= data.len() - query_len {
                if &data[i..i + query_len] == config.query.as_slice() {
                    matches.push(i);
                    i += query_len;
                } else {
                    i += 1;
                }
            }
        }

        if !matches.is_empty() {
            let mut last_pos = 0;
            for pos in matches {
                // Write content before the match
                handle.write_all(&data[last_pos..pos])?;
                // Write Red Color Code
                handle.write_all(b"\x1b[31m")?;
                // Write the matched query
                handle.write_all(&config.query)?;
                // Write Reset Color Code
                handle.write_all(b"\x1b[0m")?;
                last_pos = pos + query_len;
            }
            // Write remaining content
            handle.write_all(&data[last_pos..])?;
        }
    }
    Ok(())
}
