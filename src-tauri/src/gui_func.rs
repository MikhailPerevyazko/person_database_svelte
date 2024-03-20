use std::str::FromStr;

use crate::{db_manager::SerdePersons, storage::PersonStorage};

pub trait GUI {
    fn add_info(&self, data: &mut PersonStorage) -> Result<SerdePersons, String>;
    fn delete_by_param(&self, data: &mut PersonStorage) -> Result<SerdePersons, String>;
    fn show_by_param(
        &self,
        data: &PersonStorage,
        find_param: String,
    ) -> Result<SerdePersons, String>;
}
pub struct GUI_struct {}

impl GUI for GUI_struct {
    fn show_by_param(&self, data: &PersonStorage, find_id: String) -> Result<SerdePersons, String> {
        // let info_from_id = data.get(Some(find_id));
        // match info_from_id {
        //     Some(info_from_id) => Ok(info_from_id),
        //     None => Err("error".to_string()),
        // }
        todo!()
    }

    fn add_info(&self, _data: &mut PersonStorage) -> Result<SerdePersons, String> {
        todo!()
    }

    fn delete_by_param(&self, _data: &mut PersonStorage) -> Result<SerdePersons, String> {
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
