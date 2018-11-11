#![allow(dead_code)]

pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const ANSI_COLOR_YELLOW: &str = "\x1b[33m";
pub const ANSI_COLOR_BLUE: &str = "\x1b[34m";
pub const ANSI_COLOR_MAGENTA: &str = "\x1b[35m";
pub const ANSI_COLOR_CYAN: &str = "\x1b[36m";
pub const ANSI_BOLD: &str = "\x1b[1m";

pub const RESET: &str = "\x1b[0m";

pub fn error_logger(string: String) {
    println!("{}{}{}", RED, string, RESET);
}

pub fn success_logger(string: String) {
    println!("{}{}{}", GREEN, string, RESET);
}
