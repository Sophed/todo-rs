use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
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

pub fn add_task(path: &PathBuf, task: &str) {
    let mut task_line = String::from("- [ ] ");
    task_line.push_str(task);
    task_line.push_str("\n");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    file.write(task_line.as_bytes())
        .expect("failed to write to file");
    println!("added \"{task}\"")
}

pub fn remove_task(path: &PathBuf, line: i32) {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut parts = content.split("\n").collect::<Vec<&str>>();

    let task = parts.remove(line.try_into().expect("invalid task"));
    let content = parts.join("\n");

    fs::write(path, content).expect("failed to write to file");
    print!("removed task \"{task}\"")
}

pub fn check_task(path: &PathBuf, line: i32) {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut parts = content.split("\n").collect::<Vec<&str>>();

    let task = parts.remove(line.try_into().expect("invalid task"));
    let task = task.replacen("- [ ] ", "- [x] ", 1);
    parts.push(&task);
    let content = parts.join("\n");

    fs::write(path, content).expect("failed to write to file");
    println!("completed task \"{}\"", &task[6..])
}

pub fn uncheck_task(path: &PathBuf, line: i32) {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut parts = content.split("\n").collect::<Vec<&str>>();

    let task = parts.remove(line.try_into().expect("invalid task"));
    let task = task.replacen("- [x] ", "- [ ] ", 1);
    parts.push(&task);
    let content = parts.join("\n");

    fs::write(path, content).expect("failed to write to file");
    println!("unchecked task \"{}\"", &task[6..])
}
