use std::{io::Error, path::PathBuf};

use crate::{ansi, data};

use super::Task;

const MAX_VIEWS: u8 = 5;

pub fn display(data_file: &PathBuf) -> Result<(), Error> {
    let mut tasks = data::read_tasks(data_file)?;
    let mut uncomplete_tasks = 0;
    for (i, task) in tasks.clone().iter().enumerate() {
        if task.state {
            println!("{}{} {}[x] {}{}", ansi::CYAN, i, ansi::GREEN, ansi::RESET, task.label);
            tasks[i as usize].views += 1;
        } else {
            println!("{}{} [ ] {}{}", ansi::CYAN, i, ansi::RESET, task.label);
            uncomplete_tasks += 1;
        }
    }
    if uncomplete_tasks > 0 {
        println!("{}tasks left{}: {}", ansi::CYAN, ansi::RESET, uncomplete_tasks)
    } else {
        println!("no tasks left! ⸜( ´ ꒳ ` )⸝")
    }
    data::write_tasks(data_file, filter_views(tasks))
}

fn filter_views(list: Vec<Task>) -> Vec<Task> {
    let mut filtered_list: Vec<Task> = vec![];
    for task in list {
        if task.views < MAX_VIEWS {
            filtered_list.push(task);
        }
    }
    filtered_list
}