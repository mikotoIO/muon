use std::collections::BTreeMap;

use crate::service::{Route, Service};

pub struct AssembledApplication<Ctx>
where
    Ctx: Send + Sync + 'static,
{
    pub queries: BTreeMap<String, Route<Ctx>>,
    pub procedures: BTreeMap<String, Route<Ctx>>,
}

impl<Ctx> AssembledApplication<Ctx>
where
    Ctx: Send + Sync + 'static,
{
    pub fn from_service(service: Service<Ctx>) -> Self {
        let mut assembled_service = Self {
            queries: BTreeMap::new(),
            procedures: BTreeMap::new(),
        };
        assembled_service.load_services_recursively("base".to_string(), service);
        assembled_service
    }

    pub fn load_services_recursively(&mut self, prefix: String, service: Service<Ctx>) {
        for (path, query) in service.queries {
            self.queries.insert(format!("{}.{}", prefix, path), query);
        }
        for (path, procedure) in service.procedures {
            self.procedures
                .insert(format!("{}.{}", prefix, path), procedure);
        }
        for (name, subservice) in service.subservices {
            self.load_services_recursively(format!("{}.{}", prefix, name), subservice);
        }
    }
}
