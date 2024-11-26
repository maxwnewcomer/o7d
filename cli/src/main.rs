use clap::{Arg, Command};

mod cmds;

fn main() {
    let matches = Command::new("o7d")
        .version(env!("CARGO_PKG_VERSION"))
        .about("The o7d management tool")
        .subcommand(Command::new("setup").about("Runs the setup process and installs dependencies"))
        .subcommand(
            Command::new("init")
                .about("Initializes an `o7d` repository in the current directory")
                .arg(
                    Arg::new("environments")
                        .short('e')
                        .long("envs")
                        .help("Specify custom environments (space-separated)")
                        .num_args(1..)
                        .value_delimiter(' '),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("setup", _)) => {
            cmds::setup::run_setup();
        }
        Some(("init", sub_matches)) => {
            let envs: Vec<&str> = sub_matches
                .get_many::<String>("environments")
                .map(|values| values.map(|s| s.as_str()).collect())
                .unwrap_or_else(Vec::new);
            cmds::init::init_dir(envs);
        }
        _ => {
            println!("Use --help for available commands");
        }
    }
}
