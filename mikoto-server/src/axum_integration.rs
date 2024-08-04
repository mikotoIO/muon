use std::sync::Arc;

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use hyperschema::{layer::LayerResponse, service::Service};

struct HSAxumResponse {
    status: u16,
    body: Vec<u8>,
}

impl IntoResponse for HSAxumResponse {
    fn into_response(self) -> Response {
        Response::builder()
            .status(self.status)
            .header("Content-Type", "application/msgpack")
            .body(self.body.into())
            .unwrap()
    }
}

async fn query(Extension(service): Extension<Arc<Service<()>>>, path: Path<String>) -> Response {
    let mut path: Vec<_> = path.split('.').collect();
    let method = path.pop().unwrap();

    let mut service: &Service = &service;
    for p in path {
        service = service.subservices.get(p).unwrap();
    }
    let f = service.queries.get(method).unwrap();
    let res = f.layer.call((), vec![]).unwrap();
    if let LayerResponse::Future(f) = res {
        let res = f.await.unwrap();
        HSAxumResponse {
            status: 200,
            body: res,
        }
        .into_response()
    } else {
        panic!("Unimplemented branch")
    }
}

pub fn start_axum_server(service: Service<()>) -> Router {
    Router::new().route("/h/*path", get(query))
}
