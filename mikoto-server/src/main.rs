use error::Error;
use hyperschema::{language::typescript::TypeScriptGenerator, service::Service};
use specta::ts::ExportConfig;

pub mod entities;
pub mod error;

fn humans() -> Service<()> {
    Service::<()>::new("HumanService")
        .query("get", |_, name: String| async move {
            Ok(entities::Human {
                name,
                age: 22,
                pet: None,
            })
        })
        .procedure("set", |_, name: String| async move {
            if name.is_empty() {
                return Err(Error::NotFound.into());
            }
            Ok(entities::Human {
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
