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

    // /// Complete a todo item
    // #[clap(name = "complete")]
    // CompleteCommand(CompleteArgs),

    // /// Count todo items
    // #[clap(name = "count")]
    // CountCommand(CountArgs),

    /// Create a new todo item
    #[clap(name = "add", aliases = ["create"])]
    CreateCommand(CreateArgs),

    // /// Delete a todo item
    // #[clap(name = "rm", ailases = ["delete", "remove"])]
    // DeleteCommand(DeleteArgs),

    // /// List todo items
    // #[clap(name = "ls", aliases = ["list"])]
    // ListCommand(ListArgs),

    // /// Query todo items
    // #[clap(name = "query")]
    // QueryCommand(QueryArgs),

    // /// Show a todo item
    // #[clap(name = "show")]
    // ShowCommand(ShowArgs),

    // /// Update a todo item
    // #[clap(name = "update")]
    // UpdateCommand(UpdateArgs),
}
