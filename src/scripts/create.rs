use chrono::{DateTime, Local};

use crate::arg_parsers::CreateArgs;
use crate::utils::parse_date;

pub fn create(args: &CreateArgs) {
    println!("Creating a new todo item");

    let message: String = args.message.join(" ");
    println!("Message: {}", message);

    let due: DateTime<Local> = parse_date(&args.due.due);
    println!("Due: {}", due);
}
