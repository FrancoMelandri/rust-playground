use helloworld::Arguments;
use helloworld::Errors;
use helloworld::get_args;
use helloworld::get_greeting;

fn main() -> Result<(), Errors> {
    let args: Arguments = get_args();
    if args.name.is_empty() {
        return Err(Errors::NoArguments);
    }
    println!("{}", get_greeting(args));
    Ok(())
}
