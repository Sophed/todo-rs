use std::{env, fs, path::Path};

use anyhow::{Error, Ok, Result};
use microxdg::Xdg;

mod actions;
mod data;
mod ansi;

const FILE_NAME: &str = "todo.json";

fn main() -> Result<()> {
    let xdg = Xdg::new()?;
    let mut data_file = xdg.data()?;
    data_file.push(Path::new(FILE_NAME));

    if !fs::exists(&data_file)? {
        data::init_file(&data_file)?;
        println!("created {}", data_file.as_os_str().to_str().unwrap());
        return Ok(());
    }

    let args: Vec<String> = env::args().collect();
    let action = match args.get(1) {
        Some(s) => s,
        None => {
            actions::display::display(&data_file)?;
            return Ok(());
        }
    };

    match action.to_lowercase().as_str() {
        "add" | "create" | "new" | "+" => {
            if args.len() < 3 {
                return Err(Error::msg("no label provided, usage: todo add <task>"));
            }
            let label = args[2..].join(" ");
            actions::create::create(&data_file, label.as_str())?;
            println!("added task: {}", label);
            actions::display::display(&data_file)?
        }
        "remove" | "delete" | "del" | "-" => {
            let index: usize = match args.get(2) {
                Some(s) => s.parse::<usize>()?,
                None => {
                    return Err(Error::msg("invalid index provided, usage: todo remove <id>"));
                }
            };
            actions::remove::remove(&data_file, index)?;
            println!("removed task {}", index);
            actions::display::display(&data_file)?
        }
        "done" | "do" | "x" | "toggle" => {
            let index: usize = match args.get(2) {
                Some(s) => s.parse::<usize>()?,
                None => {
                    return Err(Error::msg("invalid index provided, usage: todo done <id>"));
                }
            };
            actions::toggle::toggle(&data_file, index)?;
            actions::display::display(&data_file)?
        }
        "help" | "?" => {
            println!("usage: todo [add|remove|do|help] <args>");
            return Ok(());
        }
        _ => {
            return Err(Error::msg("invalid subcommand"));
        }
    }

    Ok(())
}
