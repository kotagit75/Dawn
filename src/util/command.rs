use std::{env::current_dir, process::Command};

pub fn run_command_and_get_output(command: &mut Command) -> Option<String> {
    match current_dir() {
        Ok(x) => {
            let output = command.current_dir(x).output();
            output.ok().and_then(|output| {
                if output.status.success() {
                    Some(String::from_utf8_lossy(&output.stdout).into_owned())
                } else {
                    None
                }
            })
        }
        Err(_) => None,
    }
}
