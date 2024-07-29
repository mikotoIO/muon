use hyperschema::{language::typescript::TypeScriptGenerator, service::Service};
use serde::{Deserialize, Serialize};
use specta::{ts::ExportConfig, Type};

pub enum Error {
    InternalServerError,
    NotFound,
}

impl From<Error> for hyperschema::error::Error {
    fn from(_: Error) -> Self {
        hyperschema::error::Error::InternalServerError
    }
}

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

fn humans() -> Service<()> {
    Service::<()>::new("HumanService")
        .query("get", |_, name: String| async move {
            Ok(Human {
                name,
                age: 22,
                pet: None,
            })
        })
        .procedure("set", |_, name: String| async move {
            if name.is_empty() {
                return Err(Error::NotFound.into());
            }
            Ok(Human {
                name,
                age: 22,
                pet: None,
            })
        })
        .query("derp", |_, _: ()| async move { Ok(()) })
}

fn main() {
    let service = Service::<()>::new("MikotoClient")
        .query("ping", |_, pong: String| async move { Ok(pong) })
        .mount("humans", humans());

    let ts = TypeScriptGenerator::new(ExportConfig::default(), &service).generate();
    println!("{}", ts);
}
