use std::{env, fs::File, io::Read, path::PathBuf, process::ExitCode};

use actions::*;
mod actions;

const BOX_EMPTY: &str = "";
const BOX_CHECKED: &str = "";

fn main() -> ExitCode {
    let todo_file: PathBuf;

    match home::home_dir() {
        Some(path) => todo_file = path.join(".config/todofile"),
        _ => panic!("failed to get your home directory"),
    }

    let mut file: File;
    if !todo_file.is_file() {
        init_file(todo_file);
        return ExitCode::from(0);
    } else {
        file = match File::open(&todo_file) {
            Ok(file) => file,
            Err(e) => panic!("failed to open file: {e}"),
        }
    }

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(e) => panic!("failed to read file: {e}"),
    }

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let action = args.get(1).expect("this should not happen").to_lowercase();

        match action.as_str() {
            "add" | "create" | "new" | "+" => {
                if args.len() < 3 {
                    println!("usage: todo add <task>");
                    return ExitCode::from(1);
                }
                add_task(&todo_file, args[2..].join(" ").as_str()).expect("failed to write file");
                return ExitCode::from(0);
            }
            _ => {}
        }

        let selected_line = match args.get(2).expect("what").parse::<i32>() {
            Ok(n) => n,
            Err(_) => panic!("invalid line number"),
        };
        match action.as_str() {
            "remove" | "delete" | "del" | "-" => {
                remove_task(&todo_file, selected_line).expect("failed to write file")
            }
            "check" | "done" => match set_task_state(&todo_file, selected_line, true) {
                Ok(_) => {}
                Err(_) => {
                    print!("invalid task");
                    return ExitCode::from(1);
                }
            },
            "uncheck" | "undo" => match set_task_state(&todo_file, selected_line, false) {
                Ok(_) => {}
                Err(_) => {
                    print!("invalid task");
                    return ExitCode::from(1);
                }
            },
            _ => {}
        }
        return ExitCode::from(0);
    }

    let mut unfinished = 0;
    let mut incr = 0;
    for line in contents.split("\n") {
        if line == "" {
            continue;
        }
        let marker = match &line.to_lowercase()[0..6] {
            "- [ ] " => {
                unfinished += 1;
                BOX_EMPTY
            }
            "- [x] " => BOX_CHECKED,
            _ => continue,
        };
        println!("{incr} {marker}  {}", &line[6..]);
        incr += 1;
    }

    if unfinished == 0 {
        println!("no tasks left! ⸜( ´ ꒳ ` )⸝")
    } else {
        println!("task remaining: {unfinished}")
    }
    ExitCode::from(0)
}
