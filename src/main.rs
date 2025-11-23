use std::process::exit;

use clap::Parser;

use envy::{
    cli::{self, Cli},
    commands::root,
};

fn main() {
    let cli = Cli::parse();

    if cli.command.is_none() {
        println!("Nothing to do. Use `envy --help` to see available commands.");
        exit(1);
    }

    let result = match cli.command.unwrap() {
        cli::Command::Set(args) => root::handle_set(args),
        cli::Command::List => root::handle_list(),
        cli::Command::Remove => todo!(),
    };

    if let Err(e) = result {
        println!("Error: {e}");
        exit(1)
    }
}
