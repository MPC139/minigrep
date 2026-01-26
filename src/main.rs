use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let path_file: &str;
    let query;

    let args: Vec<String> = env::args().collect();
    //dbg!(&args);
    if let Some(3) = Some(args.iter().len()) {
        query = args[1].as_bytes();
        path_file = &args[2];
    } else {
        return Err("Usage [query] [path_file]".into());
    }
    //dbg!(path_file, query);

    //The same of:
    // let mut f = match File::open(path_file) {
    // Ok(file) => file,         // If successful, extract the File
    // Err(e) => return Err(e.into()), // If failed, RETURN the error from the function NOW
    // };
    let f = File::open(path_file)?;
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
        for window in data.windows(query.len()) {
            if window == query {
                handle.write_all(&data)?;
                break;
            }
        }
    }
    Ok(())
}
