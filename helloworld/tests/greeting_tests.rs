use helloworld::Arguments;
use helloworld::get_greeting;

mod greeting_tests {
    use super::*;

    #[test]
    fn test_get_greeting() {
        assert_eq!(get_greeting(Arguments::from("world")), "Hello, world!");
        assert_eq!(get_greeting(Arguments::from("Rust")), "Hello, Rust!");
    }
}
