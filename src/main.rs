use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("build")
                .about("does build things")
        )
        .subcommand(
            Command::new("new")
                .about("Generates a new mod with given parameters")
                .arg(arg!([name] "Name of the mod").action(ArgAction::Set))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        println!("Building things!~")
    }

    if let Some(matches) = matches.subcommand_matches("new") {
        println!("{}", *matches.get_one::<String>("name").expect("defaulted by clap"));
    }
}