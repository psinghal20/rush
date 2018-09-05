extern crate libc;
use std::env;
use std::io;
use std::path::Path;
use std::process::Command;
pub mod colors;

fn main() {
    loop {
        print_prompt();
        let mut command_string = String::new();
        io::stdin()
            .read_line(&mut command_string)
            .expect("Failed to read the user command");

        let commands_tokens = tokenize_commands(&mut command_string);

        for mut command_tokens in commands_tokens {
            let mut is_background = false;
            if let Some(&"&") = command_tokens.last() {
                is_background = true;
                command_tokens.pop();
            }
            match command_tokens[0] {
                "exit" => std::process::exit(0),
                "cd" => change_dir(command_tokens[1]),
                _ => execute_command(command_tokens, is_background),
            }
        }
    }
}

fn print_prompt() {
    let path = env::current_dir().unwrap();
    println!("{}>> RUSHING IN {}{}{}",colors::ANSI_BOLD, colors::ANSI_COLOR_CYAN, path.display(), colors::RESET);
}

fn tokenize_commands(command_string: &mut String) -> Vec<Vec<&str> > {
    command_string.pop();
    let commands: Vec<&str> = command_string.split(';').collect();
    let mut command_tokens: Vec<Vec<&str> > = Vec::new();
    for command in commands.iter() {
        command_tokens.push(command.split(' ').collect());
    }
    command_tokens
}

fn execute_command(command_tokens: Vec<&str>, is_background: bool) {
    let mut command_instance = Command::new(command_tokens[0]);
    if let Ok(mut child) = command_instance.args(&command_tokens[1..]).spawn() {
        if is_background == false {
            child.wait().expect("command wasn't running");
        }
        else {
           colors::success_logger(format!("{} started!", child.id()));
        }
    } else {
        colors::error_logger("Command not found!".to_string());
    }
}

fn change_dir(new_path: &str) {
    let new_path = Path::new(new_path);
    match env::set_current_dir(&new_path) {
        Err(err) => println!("Failed to change the directory!\n{}", err),
        _ => (),
    }
}
