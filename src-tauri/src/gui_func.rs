use std::str::FromStr;

use crate::{
    db_manager::SerdePersons,
    storage::{Person, PersonStorage},
};

pub trait UI {
    fn add_info(&self, data: &mut PersonStorage) -> Result<SerdePersons, String>;
    fn show_info(&self, data: &PersonStorage) -> Result<SerdePersons, String>;
    fn show_all_info(&self, data: &PersonStorage) -> Result<SerdePersons, String>;
    fn delet_param(&self, data: &mut PersonStorage) -> Result<SerdePersons, String>;
    fn find_param(&self, data: &PersonStorage, find_param: String) -> Result<SerdePersons, String>;
}
pub struct GUI {}

impl UI for GUI {
    fn show_all_info(&self, _data: &PersonStorage) -> Result<SerdePersons, String> {
        todo!()
    }

    fn show_info(&self, _data: &PersonStorage) -> Result<SerdePersons, String> {
        todo!()
    }

    fn add_info(&self, _data: &mut PersonStorage) -> Result<SerdePersons, String> {
        todo!()
    }

    fn delet_param(&self, _data: &mut PersonStorage) -> Result<SerdePersons, String> {
        todo!()
    }

    fn find_param(&self, data: &PersonStorage, find_param: String) -> Result<SerdePersons, String> {
        todo!()
    }
}

//* Создаем enum параметров для функции find и delet.
#[derive(Debug, Clone)]
pub enum PersonParam {
    Name,
    Surname,
    MiddleName,
    DateOfBirth,
    Gender,
}

impl FromStr for PersonParam {
    type Err = ();
    fn from_str(input_param: &str) -> Result<Self, Self::Err> {
        match input_param {
            "Name" => Ok(PersonParam::Name),
            "Surname" => Ok(PersonParam::Surname),
            "Middle name" => Ok(PersonParam::MiddleName),
            "Date of birth" => Ok(PersonParam::DateOfBirth),
            "Gender" => Ok(PersonParam::Gender),
            _ => Err(()),
        }
    }
}
