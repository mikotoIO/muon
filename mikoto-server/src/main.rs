use hyperschema::{language::typescript::generate_typescript, service::Service};
use serde::{Deserialize, Serialize};
use specta::{ts::ExportConfig, NamedType, Type};

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
    let service = Service::<()>::new("PersonService")
        .query("getPerson", |ctx, name: String| async move {
            Person {
                name,
                age: 22,
                pet: None,
            }
        })
        .procedure("setPerson", |ctx, name: String| async move {
            Person {
                name,
                age: 22,
                pet: None,
            }
        });

    let ts = generate_typescript(&ExportConfig::default(), &service);
    println!("{}", ts);
}
