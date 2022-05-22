#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    fmt::Display,
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
};

use rand::Rng;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![echo, get_mod_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn echo(string_to_echo: String) -> Result<String, ()> {
    println!(
        "Command echo got invoked with argument string_to_echo=\"{}\"",
        string_to_echo
    );

    if rand::thread_rng().gen::<i32>() % 2 == 0 {
        Ok(format!("Echoing \"{:?}\"", string_to_echo))
    } else {
        Err(())
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ApplicationError {
    Error(String),
    IOError(String),
    Utf8Error(String),
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ApplicationError::Error(err) => err,
                ApplicationError::IOError(err) => err,
                ApplicationError::Utf8Error(err) => err,
            }
        )
    }
}
impl std::error::Error for ApplicationError {}
impl From<std::io::Error> for ApplicationError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(String::from(err.to_string()))
    }
}

impl From<std::string::FromUtf8Error> for ApplicationError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::Utf8Error(String::from(err.to_string()))
    }
}

#[tauri::command]
fn get_mod_list(raw_path: &str) -> Result<String, ApplicationError> {
    let path = Path::new(raw_path);

    //read
    let dir_reader = fs::read_dir(&path)?;
    let mut data: Vec<String> = vec![];
    for dir in dir_reader {
        let dir = dir?;
        let subdir_reader = fs::read_dir(dir.path())?;
        for file in subdir_reader {
            let file = file?;
            if file.file_name() == "ServerData.json" {
                let f = File::open(file.path())?;
                let mut reader = BufReader::new(f);
                let mut content = vec![];

                let _ = reader.read_to_end(&mut content);

                let s = String::from_utf8(content)?;
                let t = s.replace(|c: char| !c.is_ascii(), "");

                data.push(t);
            }
        }
    }

    let res = format!("[{}]", data.join(","));

    Ok(res)
}
