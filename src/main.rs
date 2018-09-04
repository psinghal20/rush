extern crate libc;
use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        println!("RUSH!!!>>>>");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read the user command");
        command.pop();
        let mut command_tokens: Vec<&str> = command.split(' ').collect();
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

fn execute_command(command_tokens: Vec<&str>, is_background: bool) {
    let mut command_instance = Command::new(command_tokens[0]);
    if let Ok(mut child) = command_instance.args(&command_tokens[1..]).spawn() {
        if is_background == false {
            child.wait().expect("command wasn't running");
        }
        else {
            println!("{} started!", child.id());
        }
    } else {
        println!("Command didn't start");
    }
}

fn change_dir(new_path: &str) {
    let new_path = Path::new(new_path);
    match env::set_current_dir(&new_path) {
        Err(err) => println!("Failed to change the directory!\n{}", err),
        _ => (),
    }
}
