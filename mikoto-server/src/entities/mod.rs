use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type)]
pub struct Human {
    pub name: String,
    pub age: i32,
    pub pet: Option<Animal>,
}

#[derive(Serialize, Deserialize, Type)]
pub struct Animal {
    pub name: String,
    pub age: i32,
}
