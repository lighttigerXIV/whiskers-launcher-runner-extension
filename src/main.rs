use std::env;
use std::process::{Command};

use simple_kl_rs::actions::{ExtensionAction, ResultAction};
use simple_kl_rs::extensions::Function::{GetResults, RunAction};
use simple_kl_rs::extensions::{emit_results, get_parameters};
use simple_kl_rs::paths::get_extension_directory;
use simple_kl_rs::results::{IconWithTextResult, SimpleKLResult};
use simple_kl_rs::settings::Settings;

fn main() {
    let parameters = get_parameters().unwrap();
    let function = parameters.function;
    let extension_id = "com-lighttigerxiv-terminal-runner";

    let mut icon_path = get_extension_directory(extension_id).unwrap();
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
                    "accent",
                    &format!("Run {}", search_text.to_owned()),
                    ResultAction::ExtensionAction(ExtensionAction::new_with_args(
                        extension_id,
                        "",
                        vec![search_text.to_owned()],
                    )),
                ),
            ));

            emit_results(results);
        }
        RunAction => {
            let args = parameters.custom_args.unwrap();
            let command = args.get(0).unwrap().to_owned();

            let terminal = Settings::get_extension_setting(extension_id, "terminal").unwrap();

            let execution_parameter =
                Settings::get_extension_setting(extension_id, "exec").unwrap();

            let shell = Settings::get_extension_setting(extension_id, "shell").unwrap();

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
