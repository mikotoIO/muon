use hyperschema::{language::typescript::TypeScriptGenerator, service::Service};
use serde::{Deserialize, Serialize};
use specta::{ts::ExportConfig, Type};

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
            Person {
                name,
                age: 22,
                pet: None,
            }
        })
        .procedure("set", |_, name: String| async move {
            Person {
                name,
                age: 22,
                pet: None,
            }
        });

    let service = Service::<()>::new("Mikoto")
        .query("ping", |_, pong: String| async move { pong })
        .mount("persons", person_service);

    let ts = TypeScriptGenerator::new(ExportConfig::default(), &service).generate();
    println!("{}", ts);
}
