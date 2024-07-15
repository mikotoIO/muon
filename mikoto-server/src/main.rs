use specta::{collect_functions, export, function, ts::export, Type};

#[derive(Type)]
pub struct Foo {
    pub bar: String,
    pub baz: Baz,
}

#[derive(Type)]
pub struct Baz {
    pub quux: String,
}

#[specta::specta]
fn the_fn(arg1: i32, arg2: bool) -> Baz {
    todo!()
}

fn main() {
    let types: Vec<_> = export::get_types().collect();
    // dbg!(types);
    // export::ts("./src/lib.ts").unwrap();

    let res = function::collect_functions![the_fn];
    dbg!(res);

    println!("Hello, world!!!");
}
