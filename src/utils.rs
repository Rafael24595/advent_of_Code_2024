use std::{fmt, fs, str};

pub(crate) enum ConsoleColors {
    CONSOLE_RESET,
    CONSOLE_SUCESS,
    CONSOLE_FAIL,
    CONSOLE_POWER,
    CONSOLE_RESULT
}

impl ConsoleColors {

    pub fn as_str(&self) -> &'static str {
        match self {
            ConsoleColors::CONSOLE_RESET => "\x1b[0m",
            ConsoleColors::CONSOLE_SUCESS => "\x1b[32m",
            ConsoleColors::CONSOLE_FAIL => "\x1b[31m",
            ConsoleColors::CONSOLE_POWER => "\x1b[34m",
            ConsoleColors::CONSOLE_RESULT => "\x1b[33m",
        }
    }

    pub fn wrap<T: fmt::Display>(&self, string: T) -> String {
        match self {
            ConsoleColors::CONSOLE_RESET => format!("{}{}{}", Self::CONSOLE_RESET, string, Self::CONSOLE_RESET),
            ConsoleColors::CONSOLE_SUCESS => format!("{}{}{}", Self::CONSOLE_SUCESS, string, Self::CONSOLE_RESET),
            ConsoleColors::CONSOLE_FAIL => format!("{}{}{}", Self::CONSOLE_FAIL, string, Self::CONSOLE_RESET),
            ConsoleColors::CONSOLE_POWER => format!("{}{}{}", Self::CONSOLE_POWER, string, Self::CONSOLE_RESET),
            ConsoleColors::CONSOLE_RESULT => format!("{}{}{}", Self::CONSOLE_RESULT, string, Self::CONSOLE_RESET),
        }
    }

}

impl fmt::Display for ConsoleColors {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }

}

pub(crate) fn print_title(title: &str) {
    let len = title.len() + 4;
    let line = String::from("-").repeat(len);
    println!("\n");
    println!("{}", line);
    println!("| {} |", title);
    println!("{}", line);
    println!("\n");
}

pub(crate) fn print_result<T: fmt::Display>(result: T) {
    println!("\nResult: {}\n", ConsoleColors::CONSOLE_RESULT.wrap(result));
}

pub(crate) fn read_to_string(file: &str) -> String {
    return fs::read_to_string(format!("./resources/{}", file))
        .expect("Oh! Something happens! Merry Christmas!");
}