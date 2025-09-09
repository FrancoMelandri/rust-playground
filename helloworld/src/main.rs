use helloworld::{Arguments, Errors, Stopwatch, get_args, get_greeting};

fn main() -> Result<(), Errors> {
    const WRONG_ARG_ID: u32 = 1;
    let sw: Stopwatch = Stopwatch::start();
    let args: Arguments = get_args();

    match args.name.is_empty() {
        true => Err(Errors::NoArguments(WRONG_ARG_ID)),
        false => {
            println!("{} in {}", get_greeting(args), sw.stop().elapsed);
            Ok(())
        }
    }
}
