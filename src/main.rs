use clap::clap_app;
use hex;
use sha1::{Sha1, Digest};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader, Write};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

fn main() {
    let matches = clap_app!(t =>
        (version: "0.1.0")
        (author: "Simon Mikkelsen <simonbodall@runbox.com>")
        (about: "A Rust based port of t.py")
        (@arg ("task-dir"): -t --("task-dir") [DIR] +required +takes_value "The directory of lists")
        (@arg list: -l --list [LIST] +required +takes_value "The list to work on")
        (@group action => 
            (@arg edit: -e --edit [TASK] +takes_value "Edit TASK to contain TEXT")
            (@arg finish: -f --finish [TASK] +takes_value "Mark TASK as finished")
            (@arg remove: -r --remove [TASK] +takes_value "Remove TASK from list")
        )
        (@arg text: [TEXT] ... +takes_value)
    ).get_matches();

    let mut path = PathBuf::from(matches.value_of("task-dir").unwrap());

    if !path.is_dir() {
        todo!();
    }

    path.push(matches.value_of("list").unwrap());

    let mut list = match load_list(&path) {
        Ok(list) => list,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => TaskList::new(),
            _ => todo!()
        }
    };

    let (edit, finish, remove) = (
        matches.is_present("edit"),
        matches.is_present("finish"),
        matches.is_present("remove")
    );

    match (edit, finish, remove) {
        (true, _, _) => {
            // TODO Fix hacky ownership
            let text = match matches.values_of("text") {
                Some(t) => t.collect::<Vec<_>>().join(" ").trim().to_owned(),
                None => String::from("")
            };

            let id = matches.value_of("edit").unwrap();
            
            list.edit(id.to_owned(), text.as_str());

            save_list(&path, list).expect("Failed to save list");
        },
        (_, true, _) => println!("Finish {}", matches.value_of("finish").unwrap()),
        (_, _, true) => {
            let id = matches.value_of("remove").unwrap().to_owned();
            
            list.remove(id);
            save_list(&path, list).expect("Failed to save list");
        },
        _ => match matches.values_of("text") {
            Some(text) => {
                list.add(text.collect::<Vec<_>>().join(" ").trim());
                save_list(&path, list).expect("Failed to save list");
            },
            None => print_list(list)
        }
    };
}

type Id = String;

#[derive(Eq)]
struct Task {
    text: String,
    id: Id
}

impl Task {
    fn new(text: &str) -> Self {
        let text = text.trim();

        Self {
            text: text.to_owned(),
            id: generate_id(text)
        }
    }
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
    fn new() -> Self {
        Self {
            tasks: HashSet::new()
        }
    }

    fn from_tasks<T: IntoIterator<Item = Task>>(tasks: T) -> Self {
        Self {
            tasks: HashSet::from_iter(tasks)
        }
    }

    fn add(&mut self, text: &str) {
        self.tasks.insert(Task::new(text));
    }

    fn remove(&mut self, id: Id) {
        self.tasks.retain(|t| t.id != id);
    }

    fn edit(&mut self, id: Id, text: &str) {
        self.remove(id);
        self.add(text);
    }
}

fn load_list(path: &Path) -> io::Result<TaskList> {
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

    Ok(TaskList::from_tasks(tasks))
}

fn save_list(path: &Path, list: TaskList) -> io::Result<()> {
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
            None => Task::new(line)
        })
    }
}

fn print_list(list: TaskList) {
    let mut stdout = io::stdout();

    for task in list.tasks {
        writeln!(&mut stdout, "{} - {}", task.id, task.text).expect("Failed to write to stdout");
    }
}
