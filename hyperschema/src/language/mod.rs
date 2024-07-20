use std::vec;

use crate::service::{RouteType, Service};

pub mod typescript;

#[derive(Debug)]
pub struct ApplicationNode {
    pub services: Vec<ServiceNode>,
}

#[derive(Debug)]
pub struct ServiceNode {
    pub name: String,
    pub path: String,
    pub routes: Vec<RouteNode>,
    pub subservices: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct RouteNode {
    pub name: String,
    pub path: String,
    pub ty: RouteType,
}

fn combine_path(path: &str, name: &str) -> String {
    if path.is_empty() {
        name.to_string()
    } else {
        format!("{}.{}", path, name)
    }
}

impl ApplicationNode {
    pub fn from_service<T: Send + Sync + 'static>(service: &Service<T>) -> Self {
        let mut app = ApplicationNode { services: vec![] };
        app.flatten_services_recursively(service, "".to_string());
        app
    }

    fn flatten_services_recursively<T: Send + Sync + 'static>(
        &mut self,
        service: &Service<T>,
        path: String,
    ) {
        let mut routes = vec![];
        let subservices = service
            .subservices
            .iter()
            .map(|(k, v)| (k.to_string(), v.name.to_string()))
            .collect();

        for (name, route) in &service.queries {
            routes.push(RouteNode {
                name: name.to_string(),
                path: combine_path(&path, name),
                ty: route.ty.clone(),
            });
        }
        for (name, route) in &service.procedures {
            routes.push(RouteNode {
                name: name.to_string(),
                path: combine_path(&path, name),
                ty: route.ty.clone(),
            });
        }

        self.services.push(ServiceNode {
            name: service.name.to_string(),
            path: path.clone(),
            routes,
            subservices,
        });

        for (name, subservice) in &service.subservices {
            self.flatten_services_recursively(subservice, combine_path(&path, name));
        }
    }
}
