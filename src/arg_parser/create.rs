use clap:: {Args};

use super::shared::{CommonArgs, DueArgs};

#[derive(Debug, Args)]
pub struct CreateArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    /// The message of the todo item
    #[arg(num_args = 1..)]
    pub message: Vec<String>,
    // TODO: This should be mandatory

    #[command(flatten)]
    pub due: DueArgs,
}
