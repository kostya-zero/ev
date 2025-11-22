use std::process::exit;

use clap::Parser;

use crate::{cli::Cli, commands::root};

mod cli;
mod commands;
mod envfile;
mod loader;
mod terminal;

fn main() {
    let cli = Cli::parse();

    if cli.command.is_none() {
        println!("Nothing to do. Use `envy --help` to see available commands.");
        exit(1);
    }

    let result = match cli.command.unwrap() {
        cli::Command::Set(args) => root::handle_set(args),
        cli::Command::Remove => todo!(),
        cli::Command::List => root::handle_list(),
    };

    if let Err(e) = result {
        println!("Error: {e}");
        exit(1)
    }
}
