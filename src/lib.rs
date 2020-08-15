use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(short = "c", long = "text")]
    text: String,

    #[structopt(short = "t", long = "tag")]
    tag: String,
}

pub struct NoteConfig {
    pub text: String,
    pub tag: String,
}

impl NoteConfig {
    pub fn new(args: &Cli) -> Result<NoteConfig, &'static str> {
        let text = args.text.clone();
        let tag = args.tag.clone();

        Ok(NoteConfig { text, tag })
    }
}


pub fn run(config: NoteConfig) -> Result<(), Box<dyn Error>> {
    let mut file;

    if config.tag.len() > 0 {
        let mut filename = config.tag.clone();

        filename += ".txt";

        file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(filename)
            .expect("Error opening file!");

    } else {
        file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("notes.txt")
        .expect("Error opening notes!");

    }

    file.write_all(config.text.as_bytes()).expect("Write failed");
    file.write_all("\n".as_bytes()).expect("Write failed");

    println!("Completed the write");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
