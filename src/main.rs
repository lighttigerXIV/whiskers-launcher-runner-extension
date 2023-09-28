use std::env;
use std::process::{Command};

use simple_kl_rs::actions::{ExtensionAction, ResultAction};
use simple_kl_rs::extensions::Function::{GetResults, RunAction};
use simple_kl_rs::extensions::{emit_results, get_extension_setting, get_parameters};
use simple_kl_rs::paths::get_extension_directory;
use simple_kl_rs::results::{IconWithTextResult, SimpleKLResult};
const EXTENSION_ID: &str = "com-lighttigerxiv-terminal-runner";

fn main() {
    let parameters = get_parameters().unwrap();
    let function = parameters.function;

    let mut icon_path = get_extension_directory(EXTENSION_ID).unwrap();
    icon_path.push("src");
    icon_path.push("icons");
    icon_path.push("icon.svg");

    match function {
        GetResults => {
            let mut results: Vec<SimpleKLResult> = Vec::new();
            let search_text = parameters.search_text.unwrap();

            results.push(SimpleKLResult::IconWithText(
                IconWithTextResult::new_with_color(
                    icon_path,
                    &format!("Run {}", search_text.to_owned()),
                    ResultAction::ExtensionAction(ExtensionAction::new_with_args(
                        EXTENSION_ID,
                        "",
                        vec![search_text.to_owned()],
                    )),
                ),
            ));

            emit_results(&results);
        }
        RunAction => {
            let args = parameters.custom_args.unwrap();
            let command = args.get(0).unwrap().to_owned();

            let terminal = get_extension_setting(EXTENSION_ID, "terminal").unwrap();

            let execution_parameter =
                get_extension_setting(EXTENSION_ID, "exec").unwrap();

            let shell = get_extension_setting(EXTENSION_ID, "shell").unwrap();

            let environemnt_shell = env::var("SHELL").unwrap();

            if shell == "fish" {
                Command::new(terminal)
                    .arg(execution_parameter)
                    .arg(environemnt_shell.to_owned())
                    .arg("-c")
                    .arg(format!("{}; {}", command, environemnt_shell.to_owned()))
                    .spawn()
                    .expect("Error running command");
            } else {
                Command::new(terminal)
                    .arg(execution_parameter)
                    .arg(environemnt_shell.to_owned())
                    .arg("-c")
                    .arg("-i")
                    .arg(format!("{}; {}", command, environemnt_shell.to_owned()))
                    .spawn()
                    .expect("Error running command");
            }
        }
    }
}
