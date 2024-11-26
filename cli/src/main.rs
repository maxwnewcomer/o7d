use clap::Command;

mod cmds;

fn main() {
    let matches = Command::new("o7d")
        .version(env!("CARGO_PKG_VERSION"))
        .about("The o7d management tool")
        .subcommand(Command::new("setup").about("Runs the setup process"))
        .get_matches();

    match matches.subcommand() {
        Some(("setup", _)) => {
            cmds::setup::run_setup();
        }
        _ => {
            println!("Use --help for available commands");
        }
    }
}
