use clap::Parser;
pub mod set;

use crate::cli::set::SetCommand;

#[derive(Debug, Parser)]
#[command(long_about = None, multicall = true)]
pub enum Command {
    #[clap(
        subcommand,
        override_usage = "set [OBJECT] [ARGS]...",
        disable_help_subcommand = true
        )]
    Set(SetCommand),

    Edit {
        path: Vec<String>,
    },

    Exit,
}
