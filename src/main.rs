extern crate libc;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::os::unix::process::CommandExt;
pub mod colors;

fn main() {
    unsafe{
        libc::signal(libc::SIGINT, libc::SIG_IGN);
        libc::signal(libc::SIGQUIT, libc::SIG_IGN);
    }
    let mut last_exit_status = true;
    loop {
        print_prompt(last_exit_status);
        let mut command_string = String::new();
        io::stdout()
            .flush()
            .expect("Failed to flush the Buffer");
        io::stdin()
            .read_line(&mut command_string)
            .expect("Failed to read the user command");
        command_string.pop();
        while command_string.chars().last() == Some('\\') {
            command_string.pop();
            let mut next_string = String::new();
            io::stdin()
                .read_line(&mut next_string)
                .expect("Failed to read the next line");
            next_string.pop();
            command_string.push_str(&next_string);
        }
        let commands_tokens = tokenize_commands(&mut command_string);

        for mut command_tokens in commands_tokens {
            let mut is_background = false;
            if let Some(&"&") = command_tokens.last() {
                is_background = true;
                command_tokens.pop();
            }
            match command_tokens[0] {
                "exit" => std::process::exit(0),
                "cd" => {
                    last_exit_status = change_dir(command_tokens[1])
                },
                _ => {
                    last_exit_status = execute_command(command_tokens, is_background);
                }
            }
        }
    }
}

fn print_prompt(last_exit_status: bool) {
    let path = env::current_dir().unwrap();
    println!("{}RUSHING IN {}{}{}  ",colors::ANSI_BOLD, colors::ANSI_COLOR_CYAN, path.display(), colors::RESET);
    if last_exit_status {
        print!("{}{}\u{2ba1}{}  ",colors::ANSI_BOLD,colors::GREEN,colors::RESET);
    }
    else{
        print!("{}{}\u{2ba1}{}  ",colors::ANSI_BOLD,colors::RED,colors::RESET);
    }
}

fn tokenize_commands(command_string: &mut String) -> Vec<Vec<&str> > {
    let commands: Vec<&str> = command_string.split(';').collect();
    let mut command_tokens: Vec<Vec<&str> > = Vec::new();
    for command in commands.iter() {
        command_tokens.push(command.split_whitespace().collect());
    }
    command_tokens
}

fn execute_command(command_tokens: Vec<&str>, is_background: bool) -> bool {
    let mut command_instance = Command::new(command_tokens[0]);
    if let Ok(mut child) = command_instance.args(&command_tokens[1..])
        .before_exec(|| {
            unsafe{
                libc::signal(libc::SIGINT, libc::SIG_DFL);
                libc::signal(libc::SIGQUIT, libc::SIG_DFL);
            }
            Result::Ok(())
        })
        .spawn() {
            if is_background == false {
                return child.wait().expect("command wasn't running").success();
            }
            else {
               colors::success_logger(format!("{} started!", child.id()));
               true
            }
        } else {
            colors::error_logger("Command not found!".to_string());
            false
        }
}

fn change_dir(new_path: &str) -> bool {
    let new_path = Path::new(new_path);
    match env::set_current_dir(&new_path) {
        Err(err) => {
            colors::error_logger(format!("Failed to change the directory!\n{}", err));
            return false;
        },
        _ => (),
    }
    return true;
}
