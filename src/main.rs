mod arg_parser;

use arg_parser::{
    CreateArgs,
    CreateCommand,
    TodoParser,
};
use clap::Parser;

fn create(args: &CreateArgs) {
    println!("Creating a new todo item");
    println!("Args: {:?}", args);
}


fn main() {
    let args: TodoParser = TodoParser::parse();
    match &args.command {
        // TODO: Implement the commented out commands

        // Complete(args) => complete(args),
        // Count(args) => count(args),
        CreateCommand(args) => create(args),
        // Delete(args) => delete(args),
        // List(args) => list(args),
        // Query(args) => query(args),
        // Show(args) => show(args),
        // Update(args) => update(args),
    }
}
