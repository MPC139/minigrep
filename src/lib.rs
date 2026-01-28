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
