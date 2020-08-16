use clap::{Arg, App, AppSettings, SubCommand};

pub mod fleet {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::fs::OpenOptions;

    pub fn read_file(filename: &str) {
        match File::open(filename) {
            Ok(mut file) => {
                let mut content = String::new();

                file.read_to_string(&mut content).unwrap();

                println!("{}", content);
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

            fleet::write_to_file(&filename, &value);
        }
    }
}
