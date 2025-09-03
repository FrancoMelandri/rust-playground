use helloworld::get_greeting;

#[test]
fn test_get_greeting() {
    assert_eq!(get_greeting(), "Hello, world!");
}
