use notes::NoteConfig;
use structopt::StructOpt;
use notes::Cli;

fn main() {
    let args = Cli::from_args();

    let note_config = NoteConfig::new(&args).unwrap();

    notes::run(note_config).unwrap_or_else(|err| eprintln!("Error! {}", err));
}
