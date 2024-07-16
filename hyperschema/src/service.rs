use std::{collections::HashMap, future::Future};

use serde::{de::DeserializeOwned, Serialize};
use specta::{DataType, Generics, NamedDataType, Type, TypeMap};

use crate::{
    error::Error,
    layer::{FnLayer, Layer, LayerResponse},
};

pub struct Service<Ctx = ()>
where
    Ctx: Send + Sync + 'static,
{
    pub queries: HashMap<String, Route<Ctx>>,
    pub procedures: HashMap<String, Route<Ctx>>,
    pub type_map: TypeMap,
}

impl<Ctx> Service<Ctx>
where
    Ctx: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Service {
            queries: HashMap::new(),
            procedures: HashMap::new(),
            type_map: TypeMap::default(),
        }
    }

    pub fn query<Fut, Arg, Res, F>(mut self, path: &'static str, f: F) -> Self
    where
        F: Fn(Ctx, Arg) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static,
        Arg: DeserializeOwned + Type,
        Res: Serialize + Type + Send + Sync + 'static,
    {
        self.queries
            .insert(path.to_string(), Route::from_fn(f, &mut self.type_map));
        self
    }

    pub fn procedure<Fut, Arg, Res, F>(mut self, path: &'static str, f: F) -> Self
    where
        F: Fn(Ctx, Arg) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static,
        Arg: DeserializeOwned + Type,
        Res: Serialize + Type + Send + Sync + 'static,
    {
        self.procedures
            .insert(path.to_string(), Route::from_fn(f, &mut self.type_map));
        self
    }
}

pub struct Route<Ctx> {
    pub layer: Box<dyn Layer<Ctx>>,
    pub ty: RouteType,
}

#[derive(Debug)]
pub enum RouteType {
    Query(DataType, DataType),
}

impl<Ctx> Route<Ctx>
where
    Ctx: Send + Sync + 'static,
{
    pub fn from_fn<Fut, Arg, Res, F>(f: F, type_map: &mut TypeMap) -> Self
    where
        F: Fn(Ctx, Arg) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static,
        Arg: DeserializeOwned + Type,
        Res: Serialize + Type + Send + Sync + 'static,
    {
        Route {
            layer: Box::new(FnLayer::new(move |ctx: Ctx, input: Vec<u8>| {
                let input: Arg = rmp_serde::from_slice(input.as_slice())
                    .map_err(|_| Error::DeserializationFailed)?;
                let fut = f(ctx, input);

                let ret = Ok(LayerResponse::Future(Box::pin(async move {
                    rmp_serde::to_vec_named(&fut.await).unwrap()
                })));
                ret
            })),
            ty: RouteType::Query(
                Arg::reference(type_map, &[]).inner,
                Res::reference(type_map, &[]).inner,
            ),
        }
    }
}
