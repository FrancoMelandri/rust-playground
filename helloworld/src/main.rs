use helloworld::get_greeting;
use helloworld::get_args;
use helloworld::Arguments;

fn main() {
    let args: Arguments = get_args();

    println!("{}", get_greeting(args));
}
