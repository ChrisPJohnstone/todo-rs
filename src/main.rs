mod arg_parser;

use arg_parser::{
    CreateArgs,
    TodoParser,
    TodoSubcommand,
};
use clap::Parser;

fn create(args: &CreateArgs) {
    println!("Creating a new todo item");
    println!("Args: {:?}", args);
}


fn main() {
    let args: TodoParser = TodoParser::parse();

    if args.common.verbose {
        println!("Verbose mode is enabled");
    } else {
        println!("Verbose mode is disabled");
    }

    match &args.command {
        TodoSubcommand::Create(args) => create(args),
    }
}
