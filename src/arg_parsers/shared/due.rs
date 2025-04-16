use clap::Args;

#[derive(Debug, Args)]
pub struct DueArgs {
    /// The due date of the todo item
    #[arg(short, long, default_value = "later")]
    pub due: String,
}
