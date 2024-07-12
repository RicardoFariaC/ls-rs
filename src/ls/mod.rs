use chrono::{DateTime, Local};
use clap::{arg, command, Parser};
use std::{fs, io::Error, path::PathBuf};

#[derive(Parser)]
#[command(version)]
pub struct DirArg {
    #[arg(default_value = std::env::current_dir().unwrap().into_os_string())]
    pub path: PathBuf,
}

impl DirArg {
    pub fn new() -> Self {
        let arg = DirArg::parse();
        arg
    }
}

pub fn execute(dir: &PathBuf) -> Result<(), Box<Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("Tried to read directory") {
            let entry = entry.unwrap();
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))
                .unwrap();
            if let Ok(time) = entry
                .metadata()
                .expect("Tried to read metadatas.")
                .modified()
            {
                let readable_time: DateTime<Local> = DateTime::from(time);
                println!(
                    "{} {}",
                    readable_time.format("%_d %b %H:%M").to_string(),
                    file_name
                );
            }
        }
    }
    Ok(())
}
