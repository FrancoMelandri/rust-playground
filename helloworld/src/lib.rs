pub fn get_greeting(args: Arguments) -> String {
    format!("Hello, {}!", args.name)
}

pub fn get_args() -> Arguments {
    Arguments::create(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| String::new()),
    )
}

#[derive(Debug)]
pub enum Errors {
    NoArguments,
}

#[derive(Debug)]
pub struct Arguments {
    pub name: String,
}

impl Default for Arguments {
    fn default() -> Self {
        Self::new()
    }
}

impl Arguments {
    pub fn new() -> Self {
        Arguments {
            name: String::new(),
        }
    }

    pub fn from(name: &str) -> Self {
        Arguments {
            name: name.to_owned(),
        }
    }

    pub fn create(name: String) -> Self {
        Arguments { name }
    }
}
