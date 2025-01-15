use std::path::PathBuf;

use anyhow::{Error, Ok};

use crate::data::{self, write_tasks};

pub fn remove(data_file: &PathBuf, index: usize) -> Result<(), Error> {
    let mut tasks = data::read_tasks(data_file)?;
    match tasks.get(index) {
        Some(_) => {
            tasks.remove(index);
        }
        None => {
            return Err(Error::msg("no task found at provided index"));
        }
    }
    write_tasks(data_file, tasks)?;
    Ok(())
}
