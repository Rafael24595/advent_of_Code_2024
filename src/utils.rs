use std::{fmt, fs, io::{self, Write}, str, time::{SystemTime, UNIX_EPOCH}};

pub(crate) enum ConsoleColors {
    CONSOLE_RESET,
    CONSOLE_SUCESS,
    CONSOLE_FAIL,
    CONSOLE_POWER,
    CONSOLE_RESULT,
    CONSOLE_BOLD
}

impl ConsoleColors {

    pub fn as_str(&self) -> &'static str {
        match self {
            ConsoleColors::CONSOLE_RESET => "\x1b[0m",
            ConsoleColors::CONSOLE_SUCESS => "\x1b[32m",
            ConsoleColors::CONSOLE_FAIL => "\x1b[31m",
            ConsoleColors::CONSOLE_POWER => "\x1b[34m",
            ConsoleColors::CONSOLE_RESULT => "\x1b[33m",
            ConsoleColors::CONSOLE_BOLD => "\x1b[1m",
        }
    }

    pub fn wrap<T: fmt::Display>(&self, string: T) -> String {
        match self {
            ConsoleColors::CONSOLE_RESET => format!("{}{}{}", Self::CONSOLE_RESET, string, Self::CONSOLE_RESET),
            ConsoleColors::CONSOLE_SUCESS => format!("{}{}{}", Self::CONSOLE_SUCESS, string, Self::CONSOLE_RESET),
            ConsoleColors::CONSOLE_FAIL => format!("{}{}{}", Self::CONSOLE_FAIL, string, Self::CONSOLE_RESET),
            ConsoleColors::CONSOLE_POWER => format!("{}{}{}", Self::CONSOLE_POWER, string, Self::CONSOLE_RESET),
            ConsoleColors::CONSOLE_RESULT => format!("{}{}{}", Self::CONSOLE_RESULT, string, Self::CONSOLE_RESET),
            ConsoleColors::CONSOLE_BOLD => format!("{}{}{}", Self::CONSOLE_BOLD, string, Self::CONSOLE_RESET),
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

pub(crate) fn print_result<T: fmt::Display>(result: T, start: u128, end: u128) {
    let time = format_time(start, end);
    println!("\nResult: {} - Time: {}\n", ConsoleColors::CONSOLE_RESULT.wrap(result), ConsoleColors::CONSOLE_RESULT.wrap(time));
}

pub(crate) fn clean_screen() { 
    print!("\x1B[2J\x1B[H");
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();
}

pub(crate) fn reestore_cursor() {
    print!("\x1B[?25h");
    io::stdout().flush().unwrap();
}

pub(crate) fn read_to_string(file: &str) -> String {
    return fs::read_to_string(format!("./resources/{}", file))
        .expect("Oh! Something happens! Merry Christmas!");
}

pub(crate) fn now() -> u128 {
    let start_time = SystemTime::now();
    let duration_since_epoch = start_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    return duration_since_epoch.as_nanos();
}

fn format_time(start: u128, end: u128) -> String {
     let time = end - start;

    let millis = time / 1_000_000;
    let nanoseconds = time % 1_000_000_000;

    let hours = millis / 3_600_000;
    let minutes = (millis % 3_600_000) / 60_000;
    let seconds = (millis % 60_000) / 1_000;
    let milliseconds = millis % 1_000;
    let remaining_nanos = nanoseconds % 1_000_000;
    let remaining_nanos = (remaining_nanos + 5_000) / 1_000;

 
    if hours > 0 {
        return format!("{hours}h {minutes}m {seconds}s {milliseconds}ms {remaining_nanos}ns");
    }
    if minutes > 0 {
        return format!("{minutes}m {seconds}s {milliseconds}ms {remaining_nanos}ns");
    }
    if seconds > 0 {
        return format!("{seconds}s {milliseconds}ms {remaining_nanos}ns");
    }
    if milliseconds > 0 {
        return format!("{milliseconds}ms {remaining_nanos}ns");
    }

    return format!("{remaining_nanos}ns");
}

