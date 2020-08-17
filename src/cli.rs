use clap::{Arg, App, AppSettings, SubCommand};
use serde::{Serialize, Deserialize, Serializer};

#[derive(Debug)]
#[derive(Default, Serialize, Deserialize)]
pub struct NoteConfig {
    pub id: String,
    pub content: String,
}

mod storage {
    use std::error::Error;
    use sled::{Config, IVec, Result};
    use serde::ser::{Serialize, SerializeStruct, Serializer};
    use bincode::{serialize, deserialize};
    use crate::cli::NoteConfig;

    pub fn insert(table: String, note: NoteConfig) {
        println!("Inserting into {}", &table);
        println!("{:?}", note);

        let config = sled::Config::new().temporary(true);
        let tree = config.open().expect("Error opening");

        let bytes = bincode::serialize(&note).expect("error serializing");

        let k = b"yo";

        tree.insert(&k, bytes);

        println!("successful insert");


        println!("YO: {:?}", tree.get(&k));

    }

   // pub fn get_all(table: String) {}
}

pub mod fleet {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::fs::OpenOptions;
    use chrono::prelude::*;
    use crate::cli::NoteConfig;


    pub fn add_note(table: &str, content: &str) {
        let new_note = NoteConfig {
            id: String::from("1"),
            content: content.to_string(),
        };

        crate::cli::storage::insert(table.to_string(), new_note);
    }

    pub fn read_file(filename: &str) {
        match File::open(filename) {
            Ok(mut file) => {
                let mut content = String::new();

                file.read_to_string(&mut content).unwrap();

                format!("{}", content);
            },

            Err(error) => {
                println!("Error opening file {}: {}", filename, error);
            }
        }
    }

    pub fn write_to_file(filename: &str, value: &str) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)
            .expect("Cannot open file");

        let mut formatted_value = String::from(value);

        let check_mark = vec![226, 152, 145];

        let mut check_mark_str = String::from_utf8(check_mark).unwrap();

        let space = vec![32];

        let space_str = String::from_utf8(space).unwrap();

        check_mark_str.push_str(&space_str);

        formatted_value.insert_str(0, &check_mark_str);

        formatted_value.push_str("\n");

        file.write_all(formatted_value.as_bytes()).expect("Write failed");
    }
}

pub fn run() {
    let matches = App::new("Fleet")
        .setting(AppSettings::ColorAlways)
        .version("1.0")
        .author("Marshall M. <MarshallDavidMiller@gmail.com>")
        .about("A CLI tool to quickly add/edit/manage notes")
        .subcommand(SubCommand::with_name("write")
            .about("Writes to a file")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .required(true)
                .help("name of the file that should be written to")
                .takes_value(true))
            .arg(Arg::with_name("value")
                .short("v")
                .long("value")
                .value_name("VALUE")
                .required(true)
                .help("value that should be appended to the file")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("read")
            .about("Reads contents from a file")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .required(true)
                .help("name of the file that should be written to")
                .takes_value(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("read") {
        if matches.is_present("file") {
            let filename = matches.value_of("file").unwrap();

            fleet::read_file(&filename);
        }
    }

    if let Some(matches) = matches.subcommand_matches("write") {
        if matches.is_present("file") {
            let filename = matches.value_of("file").unwrap();
            let value = matches.value_of("value").unwrap();

//            fleet::write_to_file(&filename, &value);
            fleet::add_note(&filename, &value);
        }
    }
}
