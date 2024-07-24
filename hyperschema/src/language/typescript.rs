use specta::ts::{self, ExportConfig};

use crate::service::{RouteType, Service};

use super::{ApplicationNode, RouteNode};

pub struct TypeScriptGenerator<'s, Ctx>
where
    Ctx: Send + Sync + 'static,
{
    conf: ExportConfig,
    service: &'s Service<Ctx>,
}

impl<'s, Ctx> TypeScriptGenerator<'s, Ctx>
where
    Ctx: Send + Sync + 'static,
{
    pub fn new(conf: ExportConfig, service: &'s Service<Ctx>) -> Self {
        TypeScriptGenerator { conf, service }
    }

    pub fn generate(&self) -> String {
        let app = ApplicationNode::from_service(self.service);
        let mut out = format!(
            "// This file has been generated by Hyperschema, using Specta. DO NOT EDIT.\n\n"
        );

        out += &"import { Transport } from '@hyperschema/client-ts';\n";
        out += &"\n";

        let type_map = &self.service.type_map;
        for (_, typ) in type_map.iter() {
            out += &specta::ts::export_named_datatype(&self.conf, typ, type_map).unwrap();
            out += "\n\n";
        }

        for service in app.services.iter() {
            out += &format!("export class {} {{\n", service.name);
            for (name, path) in service.subservices.iter() {
                out += &format!("  readonly {}: {};\n", name, path);
            }
            out += &"  constructor(protected transport: Transport) {\n";
            for (name, path) in service.subservices.iter() {
                out += &format!("    this.{} = new {}(this.transport);\n", name, path);
            }
            out += &"  }\n";
            service.routes.iter().for_each(|route| {
                out += &self.generate_route(&route);
                out += "\n";
            });
            out += "}\n\n"
        }

        out
    }

    pub fn generate_route(&self, route: &RouteNode) -> String {
        match &route.ty {
            RouteType::Query(arg, res) => {
                format!(
                    "  {}(arg: {}): Promise<{}> {{ return this.transport.query('{}', arg); }}",
                    route.name,
                    ts::datatype(&self.conf, &arg, &self.service.type_map).unwrap(),
                    ts::datatype(&self.conf, &res, &self.service.type_map).unwrap(),
                    route.path
                )
            }
            RouteType::Procedure(arg, res) => {
                format!(
                    "  {}(arg: {}): Promise<{}> {{ return this.transport.procedure('{}'); }}",
                    route.name,
                    ts::datatype(&self.conf, &arg, &self.service.type_map).unwrap(),
                    ts::datatype(&self.conf, &res, &self.service.type_map).unwrap(),
                    route.path
                )
            },
            RouteType::Event(res) => {
                format!(
                    "  {}(cb: (arg: {}) => void): () => void {{ return this.transport.event('{}', cb); }}",
                    route.name,
                    ts::datatype(&self.conf, &res, &self.service.type_map).unwrap(),
                    route.path
                )
            }
        }
    }
}
