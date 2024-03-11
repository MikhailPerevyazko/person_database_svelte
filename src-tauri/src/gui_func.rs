use crate::{db_manager::SerdePersons, storage::PersonStorage};

pub trait UI {
    fn add_info(&self, data: &mut PersonStorage) -> Result<SerdePersons, String>;
    fn show_info(&self, data: &PersonStorage) -> Result<SerdePersons, String>;
    fn show_all_info(&self, data: &PersonStorage) -> Result<SerdePersons, String>;
    fn delet_param(&self, data: &mut PersonStorage) -> Result<SerdePersons, String>;
    fn find_param(&self, data: &PersonStorage) -> Result<SerdePersons, String>;
}
pub struct GUI {}

impl UI for GUI {
    fn show_all_info(&self, _data: &PersonStorage) -> Result<SerdePersons, String> {
        // let info = data.get(None);
        // match info {
        //     Some(info) => info,
        //     None => println!("Err"),
        // }
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

    fn find_param(&self, _data: &PersonStorage) -> Result<SerdePersons, String> {
        todo!()
    }
}
