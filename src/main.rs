use clap::clap_app;

fn main() {
    let matches = clap_app!(t =>
        (version: "0.1.0")
        (author: "Simon Mikkelsen <simonbodall@runbox.com>")
        (about: "A Rust based port of t.py")
        (@arg list: -l --list [LIST] +required +takes_value "The list to work on")
        (@group action => 
            (@arg edit: -e --edit [TASK] +takes_value "Edit TASK to contain TEXT")
            (@arg finish: -f --finish [TASK] +takes_value "Mark TASK as finished")
            (@arg remove: -r --remove [TASK] +takes_value "Remove TASK from list")
        )
        (@arg text: [TEXT] ... +takes_value)
    ).get_matches();

    let (edit, finish, remove) = (
        matches.is_present("edit"),
        matches.is_present("finish"),
        matches.is_present("remove")
    );

    match (edit, finish, remove) {
        (true, _, _) => println!("Edit {}", matches.value_of("edit").unwrap()),
        (_, true, _) => println!("Finish {}", matches.value_of("finish").unwrap()),
        (_, _, true) => println!("Remove {}", matches.value_of("remove").unwrap()),
        _ => match matches.values_of("text") {
            Some(text) => println!("Add {}", text.collect::<Vec<_>>().join(" ")),
            None => println!("List")
        }
    };
}
