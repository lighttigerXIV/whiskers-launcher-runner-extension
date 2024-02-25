use actions::handle_actions;
use results::handle_results;
use whiskers_launcher_rs::api::extensions::get_extension_context;

mod actions;
mod icons;
mod results;

const EXTENSION_ID: &str = "lighttigerxiv/runner";

fn main() {
    let context = get_extension_context().unwrap();

    match context.action {
        whiskers_launcher_rs::api::extensions::Action::GetResults => {
            handle_results(context.to_owned())
        }
        whiskers_launcher_rs::api::extensions::Action::RunAction => {
            handle_actions(context.to_owned())
        }
    }
}
