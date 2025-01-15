use std::{fs::File, io::{Error, Read, Write}, path::PathBuf};

use crate::actions::Task;

pub fn init_file(data_file: &PathBuf) -> Result<(), Error> {
    File::create(data_file)?;
    let list: Vec<Task> = vec![];
    write_tasks(data_file, list)?;
    Ok(())
}

pub fn read_tasks(data_file: &PathBuf) -> Result<Vec<Task>, Error> {
    let mut data = String::new();
    let mut file = File::open(data_file)?;
    file.read_to_string(&mut data)?;
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}

pub fn write_tasks(data_file: &PathBuf, list: Vec<Task>) -> Result<(), Error> {
    let data = serde_json::to_string(&list)?;
    let mut file = File::create(data_file)?;
    file.write_all(data.as_bytes())
}