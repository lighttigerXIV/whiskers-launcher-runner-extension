use whiskers_launcher_rs::{
    actions,
    api::extensions::{send_extension_results, Context},
    results::{self, WhiskersResult},
};

use crate::{icons::get_icon_path, EXTENSION_ID};

pub fn handle_results(context: Context) {
    let search_text = context.search_text.unwrap();

    let mut results = Vec::<WhiskersResult>::new();

    results.push(WhiskersResult::Text(
        results::Text::new(
            format!("Run {}", &search_text),
            actions::Action::Extension(
                actions::Extension::new(EXTENSION_ID, "run").args(vec![search_text]),
            ),
        )
        .icon(get_icon_path("icon.svg"))
        .tint_icon(true),
    ));

    send_extension_results(results);
}
