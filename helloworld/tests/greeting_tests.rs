use helloworld::get_greeting;
use helloworld::Arguments;

#[test]
fn test_get_greeting() {
    assert_eq!(get_greeting(Arguments::from("world")), "Hello, world!");
    assert_eq!(get_greeting(Arguments::from("Rust")), "Hello, Rust!");
}
