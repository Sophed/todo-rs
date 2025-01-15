use std::path::PathBuf;

use anyhow::Error;

use crate::data::{self, write_tasks};

pub fn toggle(data_file: &PathBuf, index: usize) -> Result<(), Error> {
    let mut tasks = data::read_tasks(data_file)?;
    match tasks.get(index) {
        Some(t) => tasks[index].state = !t.state,
        None => {
            return Err(Error::msg("no task found at provided index"));
        }
    }
    write_tasks(data_file, tasks)?;
    Ok(())
}
