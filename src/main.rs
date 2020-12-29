use clap::clap_app;
use hex;
use sha1::{Sha1, Digest};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader, Write};

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

    let path = matches.value_of("list").unwrap();
    let mut list = match load_list(path) {
        Ok(list) => list,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => TaskList { tasks: HashSet::new() },
            _ => todo!()
        }
    };

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
            Some(text) => {
                list.add(text.collect::<Vec<_>>().join(" ").trim());
                save_list(path, list).expect("Failed to save list");
            },
            None => println!("List")
        }
    };
}

type Id = String;

#[derive(Eq)]
struct Task {
    text: String,
    id: Id
}

impl PartialEq for Task {
    fn eq(&self, other: &Task) -> bool {
        self.id == other.id
    }
}

impl Hash for Task {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Borrow<Id> for Task {
    fn borrow(&self) -> &Id {
        &self.id
    }
}

struct TaskList {
    tasks: HashSet<Task>
}

impl TaskList {
    fn add(&mut self, text: &str) {
        let task = Task {
            text: text.trim().to_owned(),
            id: generate_id(text)
        };

        self.tasks.insert(task);
    }
}

fn load_list(path: &str) -> io::Result<TaskList> {
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let mut tasks = HashSet::new();

    for line in file.lines() {
        match line {
            Ok(line) => if let Some(task) = task_from_taskline(&line) {
                tasks.insert(task);
            },
            Err(_) => unimplemented!()
        }
    }

    Ok(TaskList { tasks })
}

fn save_list(path: &str, list: TaskList) -> io::Result<()> {
    let mut file = File::create(path)?;

    for task in list.tasks.iter() {
        writeln!(file, "{} | id:{}", task.text, task.id)?;
    }

    Ok(())
}

fn generate_id(text: &str) -> Id {
    hex::encode(Sha1::digest(text.as_bytes()))
}

fn task_from_taskline(line: &str) -> Option<Task> {
    let line = line.trim();

    match line.chars().nth(0) {
        Some('#') => None,
        None => None,
        _ => Some(match line.rfind('|') {
            Some(index) => {
                let (text, metadata) = line.split_at(index);

                let text = text.trim();
                let mut metadata: HashMap<String, String> = metadata
                    .trim()
                    .split(',')
                    .filter_map(|s| {
                        match s.find(':') {
                            Some(i) => {
                                let (label, data) = s.split_at(i);

                                Some((label.to_owned(), data.to_owned()))
                            },
                            None => None
                        }
                    })
                    .collect();

                Task {
                    text: text.to_owned(),
                    id: metadata
                        .remove("id")
                        .unwrap_or(generate_id(text))
                        .to_owned()
                }
            },
            None => Task {
                text: String::from(line),
                id: generate_id(line)
            }
        })
    }
}
