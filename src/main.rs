mod args;

use args::{
    AddCommand,
    TodoArgs,
    TodoCommand,
};
use clap::Parser;

fn add(args: &AddCommand) {
    println!("Adding a new todo item");
    println!("Args: {:?}", args);
}


fn main() {
    let args: TodoArgs = TodoArgs::parse();

    if args.common.verbose {
        println!("Verbose mode is enabled");
    } else {
        println!("Verbose mode is disabled");
    }

    match &args.command {
        TodoCommand::Add(args) => add(args),
    }
}
