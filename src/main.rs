use std::process::exit;

use clap::Parser;

use colored::Colorize;
use envy::{
    cli::{self, Cli},
    commands::root,
};

fn main() {
    let cli = Cli::parse();

    if cli.command.is_none() {
        let _ = root::handle_list();
        exit(0);
    }

    let result = match cli.command.unwrap() {
        cli::Command::New => root::handle_new(),
        cli::Command::Set(args) => root::handle_set(args),
        cli::Command::List => root::handle_list(),
        cli::Command::Remove(args) => root::handle_remove(args),
    };

    if let Err(e) = result {
        eprintln!("{}: {e}", "error".bright_red().bold());
        exit(1)
    }
}
