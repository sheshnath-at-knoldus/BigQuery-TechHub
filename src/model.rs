use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Person {
    pub(crate) name: String,
    pub(crate) age: u32,
    pub(crate) gender: String,
}
