fn get_greeting() -> String {
    String::from("Hello, world!")
}

fn main() {
    println!("{}", get_greeting());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_greeting() {
        assert_eq!(get_greeting(), "Hello, world!");
    }
}
