use helloworld::Arguments;
use helloworld::Errors;
use helloworld::get_args;
use helloworld::get_greeting;

fn main() -> Result<(), Errors> {
    let arg_id: u32 = 1;
    let args: Arguments = get_args();

    match args.name.is_empty() {
        true => Err(Errors::NoArguments(arg_id)),
        false => {
            println!("{}", get_greeting(args));
            Ok(())
        }
    }
}
