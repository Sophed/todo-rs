use std::{io::Error, path::PathBuf};

use crate::data::{self, write_tasks};

pub fn create(data_file: &PathBuf, label: &str) -> Result<(), Error> {
    let mut tasks = data::read_tasks(data_file)?;
    tasks.push(super::Task {
        label: label.to_string(),
        state: false,
        views: 0,
    });
    write_tasks(data_file, tasks)?;
    Ok(())
}
