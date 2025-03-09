use std::{fs::read, path::PathBuf};

#[derive(Debug, Clone, Copy)]
pub struct Resources {}

impl Resources {
    pub async fn load_binary(file_path: &PathBuf) -> anyhow::Result<Vec<u8>> {
        Ok(read(file_path)?)
    }
}