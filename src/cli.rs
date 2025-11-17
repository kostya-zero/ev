use clap::{Parser, Subcommand};

// An environment manager you'll envy
#[derive(Parser)]
#[command(
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    subcommand_required = false,
    arg_required_else_help = false,
    disable_version_flag = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Set a value for variable. Can overwrite existing one.
    Set,

    /// Remove a variable.
    Remove,

    /// List all available variables.
    List,
}
