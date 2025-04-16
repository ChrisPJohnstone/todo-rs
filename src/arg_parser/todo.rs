use clap:: {Parser, Subcommand};

use super::{CreateArgs, CommonArgs};

#[derive(Debug, Parser)]
#[clap(version)]
pub struct TodoParser {
    #[clap(subcommand)]
    pub command: TodoSubcommand,

    #[command(flatten)]
    pub common: CommonArgs,
}

#[derive(Debug, Subcommand)]
pub enum TodoSubcommand {
    /// Create a new todo item
    #[clap(name = "add")]
    Create(CreateArgs),
}
