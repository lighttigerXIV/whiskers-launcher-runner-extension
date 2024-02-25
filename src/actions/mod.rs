use std::{env, process::Command};

use whiskers_launcher_rs::api::extensions::{get_extension_setting, Context};

use crate::EXTENSION_ID;

pub fn handle_actions(context: Context) {
    let action = context.extension_action.unwrap();

    if action == "run" {
        let terminal = get_extension_setting(EXTENSION_ID, "terminal_path").unwrap();
        let terminal_args = get_extension_setting(EXTENSION_ID, "terminal_args").unwrap();
        let shell_setting = get_extension_setting(EXTENSION_ID, "shell").unwrap();
        let custom_shell_setting = get_extension_setting(EXTENSION_ID, "custom_shell").unwrap();
        let custom_shell_args = get_extension_setting(EXTENSION_ID, "custom_shell_args").unwrap();
        let command = context.custom_args[0].to_owned();

        let shell = match shell_setting.as_str() {
            "auto" => env::var("SHELL").unwrap(),
            "bash" => "/bin/bash".to_owned(),
            "zsh" => "/bin/zsh".to_owned(),
            "fish" => "/bin/fish".to_owned(),
            _ => custom_shell_setting,
        };

        if shell_setting == "fish" {
            Command::new(terminal)
                .arg(&terminal_args)
                .arg(&shell)
                .arg("-c")
                .arg(format!("{};{} {}", &command, &shell, &custom_shell_args))
                .spawn()
                .unwrap();
        } else {
            Command::new(terminal)
                .arg(&terminal_args)
                .arg(&shell)
                .arg("-c")
                .arg("-i") // Used to make alias work
                .arg(format!("{};{} {}", &command, &shell, &custom_shell_args)) // Needs the shell in the end to keep the terminal open
                .spawn()
                .unwrap();
        }
    }
}
