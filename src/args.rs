use clap:: {Args, Parser, Subcommand};

#[derive(Debug, Args)]
pub struct CommonArgs {
    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Parser)]
#[clap(version)]
pub struct TodoArgs {
    #[clap(subcommand)]
    pub command: TodoCommand,

    #[command(flatten)]
    pub common: CommonArgs,
}

#[derive(Debug, Subcommand)]
pub enum TodoCommand {
    /// Add a new todo item
    Add(AddCommand),
}

#[derive(Debug, Args)]
pub struct AddCommand {
    /// The message of the todo item
    #[arg(num_args = 1..)]
    pub message: Vec<String>,

    /// When the todo item is due
    #[arg(short, long, default_value = "later")]
    pub due: Option<String>,

    #[command(flatten)]
    pub common: CommonArgs,
}
