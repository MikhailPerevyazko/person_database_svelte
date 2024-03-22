use crate::{db_manager::SerdePersons, storage::PersonStorage};
use std::str::FromStr;

pub trait GUI {
    fn add_info(&self, data: &mut PersonStorage) -> Result<SerdePersons, String>;
    fn delete_by_param(&self, data: &mut PersonStorage) -> Result<SerdePersons, String>;
    fn show_by_param(
        &self,
        data: &PersonStorage,
        find_param: String,
    ) -> Result<SerdePersons, String>;
}
pub struct GUIStruct {}

impl GUI for GUIStruct {
    fn show_by_param(&self, data: &PersonStorage, find_id: String) -> Result<SerdePersons, String> {
        let mut id: Vec<i32> = Vec::new();
        for i in find_id.lines() {
            id.push(i.parse::<i32>().unwrap_or_default());
        }
        //*Получаем вектор персона по id */
        let info_from_id = data.get(Some(id));
        match info_from_id {
            None => Err("error")?,
            Some(info_from_id) => {
                let person_storage_info: PersonStorage = info_from_id.into();
                let serde_persons_info: SerdePersons = person_storage_info.into();
                Ok(serde_persons_info)
            }
        }
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
