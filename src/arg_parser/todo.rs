use clap:: {Parser, Subcommand};

use super::CreateArgs;
use super::shared::CommonArgs;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct TodoParser {
    #[command(flatten)]
    pub common: CommonArgs,

    #[clap(subcommand)]
    pub command: TodoCommand,
}

#[derive(Debug, Subcommand)]
pub enum TodoCommand {
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
