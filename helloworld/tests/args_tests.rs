use helloworld::Arguments;

mod args_tests {
    use super::*;

    #[test]
    fn test_arguments_creation() {
        let args1 = Arguments::new();
        assert_eq!(args1.name, "");

        let args2 = Arguments::default();
        assert_eq!(args2.name, "");

        let args3 = Arguments::from("test");
        assert_eq!(args3.name, "test");

        let args4 = Arguments::create(String::from("test"));
        assert_eq!(args4.name, "test");
    }

    #[test]
    fn test_arguments_debug() {
        let args = Arguments::from("test");
        let debug_output = format!("{:?}", args);
        assert!(debug_output.contains("test"));
    }
}
