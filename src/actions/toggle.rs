use std::path::PathBuf;

use anyhow::Error;

use crate::data::{self, write_tasks};

pub fn toggle(data_file: &PathBuf, index: u8) -> Result<(), Error> {
    let mut tasks = data::read_tasks(data_file)?;
    match tasks.get(index as usize) {
        Some(t) => tasks[index as usize].state = !t.state,
        None => {
            return Err(Error::msg("no task found at provided index"));
        }
    }
    write_tasks(data_file, tasks)?;
    Ok(())
}
