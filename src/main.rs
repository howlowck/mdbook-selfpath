use clap::{Arg, Command};
use mdbook::preprocess::CmdPreprocessor;
use mdbook::preprocess::Preprocessor;
use mdbook_selfpath::SelfPathPreprocessor; // import our library Preprocessor
use std::io;
use std::process;

pub fn make_app() -> Command {
    Command::new("selfpath-preprocessor")
        .about("A preprocessor for mdBook that replaces {{ #selfpath }} with the self path.")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        let renderer = sub_args
            .get_one::<String>("renderer")
            .expect("Required argument");
        let preprocessor = SelfPathPreprocessor::new();
        let supported = preprocessor.supports_renderer(renderer);

        // Signal whether the renderer is supported by exiting with 1 or 0.
        if supported {
            process::exit(0);
        } else {
            process::exit(1);
        }
    } else {
        let (ctx, book) = CmdPreprocessor::parse_input(io::stdin()).expect("Failed to parse input");
        let preprocessor = SelfPathPreprocessor::new();
        let processed_book = preprocessor.run(&ctx, book).unwrap_or_else(|e| {
            eprintln!("Preprocessor error: {}", e);
            process::exit(1);
        });
        serde_json::to_writer(io::stdout(), &processed_book)
            .expect("Failed to write processed book to stdout");
    }
}
