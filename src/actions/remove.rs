use std::path::PathBuf;

use anyhow::{Error, Ok};

use crate::data::{self, write_tasks};

pub fn remove(data_file: &PathBuf, index: u8) -> Result<(), Error> {
    let mut tasks = data::read_tasks(data_file)?;
    match tasks.get(index as usize) {
        Some(_) => {
            tasks.remove(index as usize);
        }
        None => {
            return Err(Error::msg("no task found at provided index"));
        }
    }
    write_tasks(data_file, tasks)?;
    Ok(())
}
