use crate::Result;
use std::path::PathBuf;

pub fn root_of(pid: u32) -> Result<PathBuf> {
    Ok(std::fs::read_link(pid_path!(pid, "root"))?)
}