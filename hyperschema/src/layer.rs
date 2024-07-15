use std::{future::Future, marker::PhantomData, pin::Pin};

use crate::error::Error;

pub trait Layer<Ctx: 'static>: Send + Sync + 'static {
    fn call(&self, ctx: Ctx, input: Vec<u8>) -> Result<LayerResponse<Vec<u8>>, Error>;
}

pub enum LayerType {}

pub enum LayerResponse<T> {
    Future(Pin<Box<dyn Future<Output = T> + Send>>),
}

pub struct FnLayer<Ctx, F>
where
    Ctx: Send + Sync + 'static,
    F: Fn(Ctx, Vec<u8>) -> Result<LayerResponse<Vec<u8>>, Error> + Send + Sync + 'static,
{
    pub func: F,
    pub _ctx: PhantomData<Ctx>,
}

impl<Ctx, F> FnLayer<Ctx, F>
where
    Ctx: Send + Sync + 'static,
    F: Fn(Ctx, Vec<u8>) -> Result<LayerResponse<Vec<u8>>, Error> + Send + Sync + 'static,
{
    pub fn new(func: F) -> Self {
        FnLayer {
            func,
            _ctx: PhantomData,
        }
    }
}

impl<Ctx, F> Layer<Ctx> for FnLayer<Ctx, F>
where
    Ctx: Send + Sync + 'static,
    F: Fn(Ctx, Vec<u8>) -> Result<LayerResponse<Vec<u8>>, Error> + Send + Sync + 'static,
{
    fn call(&self, ctx: Ctx, input: Vec<u8>) -> Result<LayerResponse<Vec<u8>>, Error> {
        (self.func)(ctx, input)
    }
}
