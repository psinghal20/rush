extern crate libc;
extern crate rustyline;

use std::env;
use std::io;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::fs::File;
use rustyline::Editor;
pub mod colors;

fn main() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
        libc::signal(libc::SIGQUIT, libc::SIG_IGN);
    }
    let mut last_exit_status = true;
    let mut rl = Editor::<()>::new();
    let home = env::var("HOME").unwrap();
    if rl.load_history(&format!("{}/.rush_history", home)).is_err() {
        println!("No previous history.");
        File::create(format!("{}/.rush_history", home)).expect("Couldn't create history file");
    }
    loop {
        let prompt_string = print_prompt(last_exit_status);
        let mut command_string = rl.readline(&prompt_string).unwrap();
        rl.add_history_entry(command_string.as_ref());
        while command_string.chars().last() == Some('\\') {
            command_string.pop();
            let mut next_string = String::new();
            io::stdin()
                .read_line(&mut next_string)
                .expect("Failed to read the next line");
            next_string.pop();
            command_string.push_str(&next_string);
        }
        let commands = tokenize_commands(&mut command_string);

        for mut command in commands {
            last_exit_status = true;
            for mut dependent_command in command {
                let mut is_background = false;
                if let Some(&"&") = dependent_command.last() {
                    is_background = true;
                    dependent_command.pop();
                }
                match dependent_command[0] {
                    "exit" => {
                        rl.save_history(&format!("{}/.rush_history", home)).expect("Couldn't save history");
                        std::process::exit(0);
                    },
                    "cd" => {
                        last_exit_status = change_dir(dependent_command[1]);
                    }
                    _ => {
                        last_exit_status = execute_command(dependent_command, is_background);
                    }
                }
                if last_exit_status == false {
                    break;
                }
            }
        }
    }
}

fn print_prompt(last_exit_status: bool) -> String {
    let path = env::current_dir().unwrap();
    let prompt = format!(
        "{}RUSHING IN {}{}{}\n",
        colors::ANSI_BOLD,
        colors::ANSI_COLOR_CYAN,
        path.display(),
        colors::RESET
    );
    if last_exit_status {
        return format!(
            "{}{}{}\u{2ba1}{}  ",
            prompt,
            colors::ANSI_BOLD,
            colors::GREEN,
            colors::RESET
        );
    } else {
        return format!(
            "{}{}{}\u{2ba1}{}  ",
            prompt,
            colors::ANSI_BOLD,
            colors::RED,
            colors::RESET
        );
    }
}

fn tokenize_commands(command_string: &mut String) -> Vec<Vec<Vec<&str>>> {
    let commands: Vec<&str> = command_string.split(';').collect();
    let mut command_tokens: Vec<Vec<Vec<&str>>> = Vec::new();
    for command in commands.iter() {
        let mut dependent_commands: Vec<&str> = command.split("&&").collect();
        let mut temp_vec: Vec<Vec<&str>> = Vec::new();
        for dependent_command in dependent_commands.iter() {
            temp_vec.push(dependent_command.split_whitespace().collect());
        }
        command_tokens.push(temp_vec);
    }
    command_tokens
}

fn execute_command(command_tokens: Vec<&str>, is_background: bool) -> bool {
    let mut command_instance = Command::new(command_tokens[0]);
    if let Ok(mut child) = command_instance
        .args(&command_tokens[1..])
        .before_exec(|| {
            unsafe {
                libc::signal(libc::SIGINT, libc::SIG_DFL);
                libc::signal(libc::SIGQUIT, libc::SIG_DFL);
            }
            Result::Ok(())
        })
        .spawn()
    {
        if is_background == false {
            return child.wait().expect("command wasn't running").success();
        } else {
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
        }
        _ => (),
    }
    return true;
}
