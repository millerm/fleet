use clap::{Arg, App, AppSettings, SubCommand};

fn main() {
    let _matches = App::new("Fleet")
        .setting(AppSettings::ColorAlways)
        .version("1.0")
        .author("Marshall M. <MarshallDavidMiller@gmail.com>")
        .about("A CLI tool to quickly add/edit/manage notes")
        .subcommand(SubCommand::with_name("write")
            .about("Writes to a file")
            .arg(Arg::with_name("value")
                .short("v")
                .long("value")
                .value_name("VALUE")
                .required(true)
                .help("value that should be appended to the file")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("read")
            .about("Reads contents from a file"))
        .get_matches();

    if let Some(_matches) = _matches.subcommand_matches("read") {
        notes::fleet::get_all().unwrap();
    }

    if let Some(_matches) = _matches.subcommand_matches("write") {
        let _value = _matches.value_of("value").unwrap();
        notes::fleet::insert(&_value).unwrap();
    }
}
