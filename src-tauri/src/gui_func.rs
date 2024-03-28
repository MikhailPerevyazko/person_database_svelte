use crate::{
    db_manager::SerdePersons,
    storage::{Person, PersonStorage},
};
use chrono::NaiveDate;
use rand::Rng;
use std::str::FromStr;

pub trait GUI {
    fn add_info(
        &self,
        data: &mut PersonStorage,
        new_name: String,
        new_surname: String,
        new_middle_name: String,
        string_date: String,
        gender: String,
    );

    fn delete_by_param(&self, data: &mut PersonStorage) -> Result<SerdePersons, String>;

    fn find_by_param(
        &self,
        data: &PersonStorage,
        find_param: String,
    ) -> Result<SerdePersons, String>;
}

pub struct GUIStruct {}

impl GUI for GUIStruct {
    //*Функция, которая получает персону по заданному id. */
    fn find_by_param(&self, data: &PersonStorage, find_id: String) -> Result<SerdePersons, String> {
        let mut id: Vec<i32> = Vec::new();
        for i in find_id.lines() {
            id.push(i.parse::<i32>().unwrap_or_default());
        }
        //Получаем вектор персона по id
        let info_from_id = data.get(Some(id));
        match info_from_id {
            None => Err("This ID doesn't exists")?,
            Some(info_from_id) => {
                let person_storage_info: PersonStorage = info_from_id.into();
                let serde_persons_info: SerdePersons = person_storage_info.into();
                Ok(serde_persons_info)
            }
        }
    }

    //*Функция добавляет нового персона в базу данных. */
    fn add_info(
        &self,
        data: &mut PersonStorage,
        new_name: String,
        new_surname: String,
        new_middle_name: String,
        mut string_date: String,
        new_gender_string: String,
    ) {
        //* Пользователь получает сгенерированный id.

        let random_num: i32 = rand::thread_rng().gen_range(1000000, 9999999);
        let new_id = random_num;

        //*Создаем пустой вектор, в который программа будет записывать новые данные о персоне */
        let mut new_person: Vec<_> = Vec::new();

        //*Добавляем новый name в вектор */
        for line in new_name.lines() {
            new_person.push(line.to_string());
        }

        //*Добавляем новый surname в вектор */
        for line in new_surname.lines() {
            new_person.push(line.to_string());
        }

        //*Добавляем новый middle name в вектор */
        for line in new_middle_name.lines() {
            new_person.push(line.to_string());
        }

        //*Парсим новый date of birth */
        let get_len_from_date = &string_date.len();
        string_date.truncate(get_len_from_date - 1);
        let new_date_str: &str = &string_date.as_str();
        let new_date = NaiveDate::parse_from_str(&new_date_str, "%d-%m-%Y").unwrap();

        //*Парсим новый gender */
        let mut new_gender: bool = true;
        for line in new_gender_string.lines() {
            if line == "Male" {
                new_gender = true;
            } else if line == "Female" {
                new_gender = false;
            } else {
                println!("Wrong gender format.")
            }
        }

        let new_person: Person = Person {
            name: new_person[0].to_string(),
            surname: new_person[1].to_string(),
            middle_name: new_person[3].to_string(),
            date_of_birth: new_date,
            gender: new_gender,
        };

        data.add(new_id, new_person);
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
