// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;
mod db_manager;
mod gui_func;
mod storage;
mod ui;

use crate::db_manager::{BDOperation, SerdePersons};

use std::path::PathBuf;
use std::sync::Mutex;
use storage::PersonStorage;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![open_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct AppState {
    pub file_path: Mutex<PathBuf>,
    pub person_storage: Mutex<PersonStorage>,
}

impl AppState {
    pub fn set_file_path(&self, file_path: String) {
        {
            let mut lock = self.file_path.lock().unwrap();
            *lock = PathBuf::from(file_path);
        }
    }
    pub fn get_file_path(&self) -> PathBuf {
        let result = {
            let lock = self.file_path.lock().unwrap();
            lock.to_owned()
        };
        result
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            file_path: Mutex::new(PathBuf::new()),
            person_storage: Mutex::new(PersonStorage::default()),
        }
    }
}

#[tauri::command]
fn open_db(state: tauri::State<AppState>, file_path: String) -> Result<SerdePersons, String> {
    //*Записали file_path в структуру AppState */
    state.set_file_path(file_path);
    let persons = match state.load() {
        Ok(person) => person,
        Err(err) => return Err(err.to_string()),
    };
    //*Проверяем заполняется ли persons */
    println!("{:#?}", persons);
    Ok(persons)
}

impl BDOperation for AppState {
    fn load(&self) -> Result<crate::db_manager::SerdePersons, Box<dyn std::error::Error>> {
        let data: SerdePersons = match &self.file_path.lock() {
            Ok(path) => {
                let handler = std::fs::File::open(path.as_path())?;
                serde_yaml::from_reader(&handler)?
            }
            Err(err) => {
                return Err(err.to_string())?;
            }
        };
        if !data.is_empty() {
            match self.person_storage.lock() {
                Ok(mut lock) => {
                    let persons: PersonStorage = data.clone().into();
                    *lock = persons;
                }
                Err(err) => {
                    return Err(err.to_string())?;
                }
            }
        }

        Ok(data)
    }
    fn save(
        &self,
        persons: &crate::db_manager::SerdePersons,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match &self.file_path.lock() {
            Ok(path) => {
                let handler = std::fs::File::create(path.as_path())?;
                serde_yaml::to_writer(&handler, &persons)?;
            }
            Err(err) => {
                return Err(err.to_string())?;
            }
        }

        Ok(())
    }
}
