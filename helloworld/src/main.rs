use helloworld::Arguments;
use helloworld::get_args;
use helloworld::get_greeting;

fn main() {
    let args: Arguments = get_args();

    println!("{}", get_greeting(args));
}
