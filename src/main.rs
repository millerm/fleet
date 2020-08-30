use clap::{Arg, App, AppSettings, SubCommand};
use std::process;

fn main() {
    let _matches = App::new("Fleet")
        .setting(AppSettings::ColorAlways)
        .version("1.0")
        .author("Marshall M. <MarshallDavidMiller@gmail.com>")
        .about("A CLI tool to quickly add/edit/manage notes")
        .subcommand(SubCommand::with_name("create")
            .about("Creates a new note with the content supplied")
            .arg(Arg::with_name("content")
                .short("c")
                .long("content")
                .value_name("CONTENT")
                .required(true)
                .help("content that should be added to the list of notes")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("read")
            .about("Reads contents from a file"))
        .subcommand(SubCommand::with_name("delete")
            .about("Deletes a note from the notes list")
            .arg(Arg::with_name("id")
                .short("i")
                .long("id")
                .value_name("ID")
                .required(true)
                .help("id of the note that should be deleted")
                .takes_value(true)))
        .get_matches();

    if let Some(_matches) = _matches.subcommand_matches("read") {
        fleet::core::get_all().unwrap_or_else(|err| {
            eprintln!("An error while trying to read your notes: {}", err);
            process::exit(1);
        });
    }

    if let Some(_matches) = _matches.subcommand_matches("create") {
        let _value = _matches.value_of("content").unwrap();

        fleet::core::insert(&_value).unwrap_or_else(|err| {
            eprintln!("An error while trying to create your note: {}", err);
            process::exit(1);
        });
    }

    if let Some(_matches) = _matches.subcommand_matches("delete") {
        if _matches.is_present("id") {
            let _id = _matches.value_of("id").unwrap();
            let id_int: i32 = _id.parse().unwrap();

            fleet::core::delete_note(&id_int).unwrap_or_else(|err| {
                eprintln!("Error while trying to delete your note: {}", err);
                process::exit(1);
            });
        } else {
            eprintln!("Please provide the id of the note you wish to delete!");
            process::exit(1);
        }
    }
}
