use std::{env, error::Error, fs::File, io::Read};

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

    let mut f = File::open(path_file)?;
    //The same of:
    // let mut f = match File::open(path_file) {
    // Ok(file) => file,         // If successful, extract the File
    // Err(e) => return Err(e.into()), // If failed, RETURN the error from the function NOW
    // };

    let mut data = String::new();

    f.read_to_string(&mut data)?;

    //dbg!(&data);

    let lines: Vec<&str> = data.split('\n').collect();
    //dbg!(&lines);

    let lines_match: Vec<_> = lines.iter().filter(|x| x.contains(query)).collect();
    if let true = lines_match.is_empty() {
        return Err("query not found".into());
    }
    //dbg!(&line_match);

    let lines_match = lines_match
        .into_iter()
        .map(|x| x.replace(query, &format!("\x1b[31m{}\x1b[0m", query)));

    //dbg!(&lines_match);
    for line in lines_match {
        println!("{}", line);
    }
    Ok(())
}
