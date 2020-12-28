use clap::clap_app;
use std::io::{self, Write};
use std::fs::{self, OpenOptions};

fn main() {
    let matches = clap_app!(tart =>
        (version: "0.1")
        (author: "Simon Mikkelsen <simonbodall@runbox.com>")
        (about: "An umambitious todo application")
        (@arg FILE: -f --file +required +takes_value "The todo file")
        (@subcommand list =>
            (about: "Lists all items")
        )
        (@subcommand add =>
            (about: "Adds a new todo item")
            (@arg TEXT: +required "The text related to the item")
        )
    ).get_matches();

    let file = matches.value_of("FILE").unwrap();

    match matches.subcommand_name() {
        Some("list") => list(file),
        Some("add") => {
            if let Some(ref matches) = matches.subcommand_matches("add") {
                let text = matches.value_of("TEXT").unwrap();

                add(file, text);
            }
        },
        Some(_) => panic!("Unable to parse subcommand"),
        None => println!("default")
    };
}

fn list(path: &str) {
    io::stdout()
        .write_all(fs::read_to_string(path).expect("Failed to read file").as_bytes())
        .expect("");
}

fn add(path: &str, text: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    writeln!(file, "{}", text).expect("Failed to write to file");
}
