use crate::arg_parsers::CreateArgs;

pub fn create(args: &CreateArgs) {
    println!("Creating a new todo item");
    println!("Args: {:?}", args);
}
