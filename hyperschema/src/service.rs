use std::{collections::HashMap, future::Future};

use serde::{de::DeserializeOwned, Serialize};
use specta::{DataType, Type, TypeMap};

use crate::{
    error::Error,
    layer::{FnLayer, Layer, LayerResponse},
};

pub struct Service<Ctx = ()>
where
    Ctx: Send + Sync + 'static,
{
    pub name: &'static str,
    pub subservices: HashMap<String, Service<Ctx>>,
    pub type_map: TypeMap,

    pub queries: HashMap<String, Route<Ctx>>,
    pub procedures: HashMap<String, Route<Ctx>>,
}

impl<Ctx> Service<Ctx>
where
    Ctx: Send + Sync + 'static,
{
    pub fn new(name: &'static str) -> Self {
        Service {
            name,
            queries: HashMap::new(),
            procedures: HashMap::new(),
            subservices: HashMap::new(),
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
        self.queries.insert(
            path.to_string(),
            Route::from_fn(
                f,
                RouteType::Query(
                    Arg::reference(&mut self.type_map, &[]).inner,
                    Res::reference(&mut self.type_map, &[]).inner,
                ),
            ),
        );
        self
    }

    pub fn procedure<Fut, Arg, Res, F>(mut self, path: &'static str, f: F) -> Self
    where
        F: Fn(Ctx, Arg) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static,
        Arg: DeserializeOwned + Type,
        Res: Serialize + Type + Send + Sync + 'static,
    {
        self.procedures.insert(
            path.to_string(),
            Route::from_fn(
                f,
                RouteType::Procedure(
                    Arg::reference(&mut self.type_map, &[]).inner,
                    Res::reference(&mut self.type_map, &[]).inner,
                ),
            ),
        );
        self
    }

    pub fn mount(mut self, path: &'static str, sub: Service<Ctx>) -> Self {
        for (sid, ty) in sub.type_map.iter() {
            self.type_map.insert(sid.clone(), ty.clone());
        }
        self.subservices.insert(path.to_string(), sub);
        self
    }
}

pub struct Route<Ctx> {
    pub layer: Box<dyn Layer<Ctx>>,
    pub ty: RouteType,
}

#[derive(Debug, Clone)]
pub enum RouteType {
    Query(DataType, DataType),
    Procedure(DataType, DataType),
}

impl<Ctx> Route<Ctx>
where
    Ctx: Send + Sync + 'static,
{
    pub fn from_fn<Fut, Arg, Res, F>(f: F, ty: RouteType) -> Self
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
            ty,
        }
    }
}
