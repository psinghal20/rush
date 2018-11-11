pub fn tokenize_commands(command_string: &str) -> Vec<Vec<Vec<&str>>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_command() {
        let commands = "ls";
        let tokens = tokenize_commands(commands);

        assert_eq!(vec![vec![vec!["ls"]]], tokens);
    }

    #[test]
    fn single_command_with_args() {
        let commands = "ls -a";
        let tokens = tokenize_commands(commands);

        assert_eq!(vec![vec![vec!["ls", "-a"]]], tokens);
    }

    #[test]
    fn background_process() {
        let commands = "long-running-process &";
        let tokens = tokenize_commands(commands);

        assert_eq!(vec![vec![vec!["long-running-process", "&"]]], tokens);
    }

    #[test]
    fn background_process_with_other() {
        let commands = "long-running-process & date";
        let tokens = tokenize_commands(commands);

        assert_eq!(vec![vec![vec!["long-running-process", "&", "date"]]], tokens);
    }

    #[test]
    fn semicolon() {
        let commands = "date ; ls";
        let tokens = tokenize_commands(commands);

        assert_eq!(vec![vec![vec!["date"]], vec![vec!["ls"]]], tokens);
    }

    #[test]
    fn and() {
        let commands = "date && ls";
        let tokens = tokenize_commands(commands);

        assert_eq!(vec![vec![vec!["date"], vec!["ls"]]], tokens);
    }
}
