use minigrep::*;
use std::{env, error::Error, fs::File};

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
