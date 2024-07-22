use hyperschema::{language::typescript::TypeScriptGenerator, service::Service};
use serde::{Deserialize, Serialize};
use specta::{ts::ExportConfig, Type};

pub enum Error {
    InternalServerError,
    NotFound,
}

impl From<hyperschema::error::Error> for Error {
    fn from(_: hyperschema::error::Error) -> Self {
        Error::InternalServerError
    }
}

#[derive(Serialize, Deserialize, Type)]
pub struct Person {
    pub name: String,
    pub age: i32,
    pub pet: Option<Animal>,
}

#[derive(Serialize, Deserialize, Type)]
pub struct Animal {
    pub name: String,
    pub age: i32,
}

fn main() {
    let person_service = Service::<()>::new("PersonService")
        .query("get", |_, name: String| async move {
            Ok(Person {
                name,
                age: 22,
                pet: None,
            })
        })
        .procedure("set", |_, name: String| async move {
            Ok(Person {
                name,
                age: 22,
                pet: None,
            })
        });

    let service = Service::<()>::new("Mikoto")
        .query("ping", |_, pong: String| async move { Ok(pong) })
        .mount("persons", person_service);

    let ts = TypeScriptGenerator::new(ExportConfig::default(), &service).generate();
    println!("{}", ts);
}
