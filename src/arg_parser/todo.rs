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
    // TODO: Implement the commented out commands

    // /// Complete a new todo item
    // #[clap(name = "complete")]
    // CompleteCommand(CompleteArgs),

    // /// Count a new todo item
    // #[clap(name = "count")]
    // CountCommand(CountArgs),

    /// Create a new todo item
    #[clap(name = "add")]
    CreateCommand(CreateArgs),

    // /// Delete a new todo item
    // #[clap(name = "rm")]
    // DeleteCommand(DeleteArgs),

    // /// List a new todo item
    // #[clap(name = "ls")]
    // ListCommand(ListArgs),

    // /// Query a new todo item
    // #[clap(name = "query")]
    // QueryCommand(QueryArgs),

    // /// Show a new todo item
    // #[clap(name = "show")]
    // ShowCommand(ShowArgs),

    // /// Update a new todo item
    // #[clap(name = "update")]
    // UpdateCommand(UpdateArgs),
}
