use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let path_file: &str;
    let query: &str;

    let args: Vec<String> = env::args().collect();
    //dbg!(&args);
    if let Some(3) = Some(args.iter().len()) {
        query = &args[1];
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
    let mut data = String::new();

    loop {
        data.clear();
        let bytes = buffer.read_line(&mut data)?;
        if bytes == 0 {
            break;
        }

        if data.contains(query) {
            let colored_line = data.replace(query, &format!("\x1b[31m{}\x1b[0m", query));
            print!("{}", colored_line);
        }
    }

    Ok(())
}
