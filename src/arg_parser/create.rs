use clap:: {Args};

use super::CommonArgs;

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// The message of the todo item
    #[arg(num_args = 1..)]
    pub message: Vec<String>,

    /// When the todo item is due
    #[arg(short, long, default_value = "later")]
    pub due: Option<String>,

    #[command(flatten)]
    pub common: CommonArgs,
}
