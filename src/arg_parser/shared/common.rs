use clap::Args;

#[derive(Debug, Args)]
pub struct CommonArgs {
    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}
