use clap::{
    Subcommand,
    Args
};

#[derive(Args, Debug)]
pub struct SetInterfaceArgs {
    interface: String,
    #[command(subcommand)]
    tbd: Option<SetInterfaceTbd>,
}

#[derive(Subcommand, Debug)]
pub enum SetInterfaceTbd {
    #[clap(alias = "addr")]
    Address(InterfaceAddress),
    #[clap(alias = "desc")]
    Description {
        text: String,
    },
}

#[derive(Args, Debug)]
pub struct InterfaceAddress {
    addr: String,
}
