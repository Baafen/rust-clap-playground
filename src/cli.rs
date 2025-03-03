use clap::{Parser, Subcommand};

mod interface;

use interface::SetInterfaceArgs;

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

#[derive(Subcommand, Debug)]
pub enum SetCommand {
    #[clap(alias = "if")]
    Interface(SetInterfaceArgs),
    #[clap(alias = "fw")]
    Firewall,
}
