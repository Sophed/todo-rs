use serde::{Deserialize, Serialize};

pub mod create;
pub mod display;
pub mod remove;
pub mod toggle;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    label: String,
    state: bool,
    views: u8,
}
