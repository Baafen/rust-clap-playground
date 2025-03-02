use clap::Subcommand;

pub mod interface;

use interface::SetInterfaceArgs;

#[derive(Subcommand, Debug)]
pub enum SetCommand {
    #[clap(alias = "if")]
    Interface(SetInterfaceArgs),
    #[clap(alias = "fw")]
    Firewall,
}
