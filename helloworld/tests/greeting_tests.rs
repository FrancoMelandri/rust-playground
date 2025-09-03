use helloworld::get_greeting;

#[test]
fn test_get_greeting() {
    assert_eq!(get_greeting(String::from("world")), "Hello, world!");
    assert_eq!(get_greeting(String::from("Rust")), "Hello, Rust!");
}
