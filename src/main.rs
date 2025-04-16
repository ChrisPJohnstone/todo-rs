mod arg_parsers;
mod scripts;
mod utils;

use crate::arg_parsers::{
    // CompleteCommand,
    // CountCommand,
    CreateCommand,
    // DeleteCommand,
    // ListCommand,
    TodoParser,
    // QueryCommand,
    // ShowCommand,
    // UpdateCommand,
};
use crate::scripts::{
    // complete,
    // count,
    create,
    // delete,
    // list,
    // query,
    // show,
    // update,
};
use clap::Parser;

fn main() {
    let args: TodoParser = TodoParser::parse();
    match &args.command {
        // TODO: Implement the commented out commands

        // CompleteCommand(args) => complete(args),
        // CountCommand(args) => count(args),
        CreateCommand(args) => create(args),
        // DeleteCommand(args) => delete(args),
        // ListCommand(args) => list(args),
        // QueryCommand(args) => query(args),
        // ShowCommand(args) => show(args),
        // UpdateCommand(args) => update(args),
    }
}
