use std::process::exit;

use clap::Parser;

use crate::cli::Cli;

mod cli;
mod commands;
mod envfile;

fn main() {
    let cli = Cli::parse();

    if cli.command.is_none() {
        println!("Nothing to do. Use `envy --help` to see available commands.");
        exit(1);
    }

    match cli.command.unwrap() {
        cli::Command::Set => todo!(),
        cli::Command::Remove => todo!(),
        cli::Command::List => todo!(),
    }
}
