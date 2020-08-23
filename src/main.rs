use clap::{Arg, App, AppSettings, SubCommand};

fn main() {
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
            let _filename = matches.value_of("file").unwrap();
        }
    }

    if let Some(matches) = matches.subcommand_matches("write") {
        if matches.is_present("file") {
            let _filename = matches.value_of("file").unwrap();
            let value = matches.value_of("value").unwrap();

            notes::fleet::create_note(&_filename, &value);
        }
    }
}
