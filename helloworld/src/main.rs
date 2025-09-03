use helloworld::get_greeting;
use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let name = args.nth(1).unwrap_or_else(|| String::from("world"));
    println!("{}", get_greeting(name));
}
