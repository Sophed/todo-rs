use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    num::TryFromIntError,
    path::PathBuf,
};

pub fn init_file(path: PathBuf) -> File {
    match File::create(&path) {
        Ok(file) => {
            println!("initialized file at {}", path.to_str().unwrap());
            return file;
        }
        Err(e) => panic!("failed to create file: {e}"),
    };
}

pub fn add_task(path: &PathBuf, task: &str) -> Result<(), std::io::Error> {
    let mut task_line = String::from("- [ ] ");
    task_line.push_str(task);
    task_line.push_str("\n");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("failed to open file");
    match file.write(task_line.as_bytes()) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }
    println!("added \"{task}\"");
    Ok(())
}

pub fn remove_task(path: &PathBuf, line: i32) -> Result<(), TryFromIntError> {
    let mut file = File::open(path).expect("failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file");
    let mut parts = content.split("\n").collect::<Vec<&str>>();

    let task = parts.remove(match line.try_into() {
        Ok(i) => i,
        Err(e) => return Err(e),
    });
    let content = parts.join("\n");

    fs::write(path, content).expect("failed to write to file");
    println!("removed task \"{}\"", &task[6..]);
    Ok(())
}

pub fn set_task_state(path: &PathBuf, line: i32, state: bool) -> Result<(), TryFromIntError> {
    let mut file = File::open(path).expect("failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file");
    let mut parts = content.split("\n").collect::<Vec<&str>>();

    let mut task = parts
        .remove(match line.try_into() {
            Ok(i) => i,
            Err(e) => return Err(e),
        })
        .to_string();

    if state {
        task.replace_range(4..5, "x")
    } else {
        task.replace_range(4..5, " ")
    }

    parts.push(&task);
    let content = parts.join("\n");

    fs::write(path, content).expect("failed to write to file");
    println!("completed task \"{}\"", &task[6..]);
    Ok(())
}
