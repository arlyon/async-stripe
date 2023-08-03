use std::collections::HashMap;
use std::fmt::Debug;

use anyhow::Context;
use indexmap::{IndexMap, IndexSet};
use petgraph::Direction;
use tracing::info;

use crate::crate_inference::{maybe_infer_crate, validate_crate_map, Crate};
use crate::printable::{PrintableContainer, PrintableType};
use crate::requests::parse_requests;
use crate::rust_object::RustObject;
use crate::rust_type::{Container, PathToType, RustType};
use crate::spec::{as_object_properties, get_request_form_parameters, Spec};
use crate::spec_inference::{infer_id_name, Inference};
use crate::stripe_object::{parse_stripe_schema_as_rust_object, CrateInfo, OperationType, StripeObject, StripeOperation, StripeResource};
use crate::types::{ComponentPath, RustIdent};

pub struct Components {
    pub components: IndexMap<ComponentPath, StripeObject>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PathInfo {
    pub krate: Option<Crate>,
    pub path: Option<String>,
}

impl PathInfo {
    pub fn with_crate(krate: Crate) -> Self {
        Self { krate: Some(krate), path: None }
    }
}

impl Components {
    #[track_caller]
    pub fn get(&self, path: &ComponentPath) -> &StripeObject {
        &self.components[path.as_ref()]
    }

    #[track_caller]
    pub fn get_mut(&mut self, path: &ComponentPath) -> &mut StripeObject {
        &mut self.components[path.as_ref()]
    }

    pub fn get_crate_members(&self, krate: Crate) -> IndexSet<&ComponentPath> {
        let mut members = IndexSet::new();
        for (path, component) in &self.components {
            let Some(component_krate) = component.krate() else {
                continue;
            };
            if component_krate.base() == krate || component_krate.for_types() == krate {
                members.insert(path);
            }
        }
        members
    }

    #[track_caller]
    pub fn resolve_path(&self, path: &ComponentPath) -> PathInfo {
        let component = self.get(path);
        let krate = component.krate_unwrapped().for_types();
        PathInfo { krate: Some(krate), path: Some(component.mod_path()) }
    }

    pub fn construct_printable_type(&self, typ: &RustType) -> PrintableType {
        match typ {
            RustType::Object(obj, metadata) => PrintableType::QualifiedPath { path: None, has_ref: obj.has_reference(), is_ref: false, ident: metadata.ident.clone() },

            RustType::Path { path, has_reference, is_ref, .. } => {
                let (path, ident) = match path {
                    PathToType::Component(path) => {
                        let comp = self.get(path);
                        (Some(PathInfo::with_crate(comp.krate_unwrapped().for_types())), comp.ident().clone())
                    }
                    PathToType::IntraFile(ident) => (None, ident.clone()),
                    PathToType::ObjectId(path) => {
                        let ident = infer_id_name(path);
                        let path_info = self.resolve_path(path);
                        (Some(path_info), ident)
                    }
                    PathToType::Types(ident) => (Some(PathInfo { krate: Some(Crate::Types), path: None }), ident.clone()),
                };
                PrintableType::QualifiedPath { path, has_ref: *has_reference, is_ref: *is_ref, ident }
            }
            RustType::Simple(typ) => PrintableType::Simple(*typ),
            RustType::Container(typ) => {
                let inner = Box::new(self.construct_printable_type(typ.value_typ()));
                let printable = match typ {
                    Container::List(_) => PrintableContainer::List(inner),
                    Container::Vec(_) => PrintableContainer::Vec(inner),
                    Container::Slice(_) => PrintableContainer::Slice(inner),
                    Container::Expandable(_) => PrintableContainer::Expandable(inner),
                    Container::Option(_) => PrintableContainer::Option(inner),
                    Container::Box(_) => PrintableContainer::Box(inner),
                    Container::Map { key, is_ref, .. } => PrintableContainer::Map { key: *key, is_ref: *is_ref, value: inner },
                };
                PrintableType::Container(printable)
            }
        }
    }

    pub fn add_component_deps<'a>(&'a self, deps: &mut IndexSet<&'a ComponentPath>, path: &'a ComponentPath) {
        let component = self.get(path);
        let base_type = component.rust_obj();
        self.add_deps_from_obj(deps, base_type);
        let reqs = &component.requests;
        let mut typs = vec![];
        for req in reqs {
            if let Some(params) = &req.params {
                typs.push(params);
            }
            typs.push(&req.returned);
            for path_param in &req.path_params {
                typs.push(&path_param.rust_type);
            }
        }
        for typ in typs {
            self.add_deps_from_typ(deps, typ);
        }
    }

    pub fn add_deps_from_obj<'a>(&'a self, deps: &mut IndexSet<&'a ComponentPath>, obj: &'a RustObject) {
        match obj {
            RustObject::Struct(fields) => {
                for field in fields {
                    self.add_deps_from_typ(deps, &field.rust_type);
                }
            }
            RustObject::Enum(variants) => {
                for variant in variants {
                    if let Some(typ) = &variant.rust_type {
                        self.add_deps_from_typ(deps, typ);
                    }
                }
            }
            RustObject::FieldlessEnum(_) => {}
        }
    }

    pub fn add_deps_from_typ<'a>(&'a self, deps: &mut IndexSet<&'a ComponentPath>, typ: &'a RustType) {
        match typ {
            RustType::Object(obj, _) => self.add_deps_from_obj(deps, obj),
            RustType::Path { path: PathToType::Component(path), .. } => {
                deps.insert(path);
            }
            RustType::Container(inner) => self.add_deps_from_typ(deps, inner.value_typ()),
            _ => {}
        }
    }

    pub fn deps_for_component<'a>(&'a self, path: &'a ComponentPath) -> IndexSet<&'a ComponentPath> {
        let mut deps = IndexSet::new();
        self.add_component_deps(&mut deps, path);
        deps
    }

    pub fn filter_unused_components(&mut self) {
        let mut unused = vec![];
        loop {
            let graph = self.gen_component_dep_graph();
            for (path, component) in &self.components {
                // This will be a false positive here since we don't include `ApiError` in
                // dependency calculation even though it is really the error type in all
                // requests.
                if path.as_ref() == "error" {
                    continue;
                }
                if component.requests.is_empty() && graph.neighbors_directed(path, Direction::Incoming).count() == 0 {
                    unused.push(path.clone());
                }
            }
            let done_filtering = unused.is_empty();
            if done_filtering {
                break;
            }

            info!("Filtering unused components: {unused:#?}");
            for unused_mod in unused.drain(..) {
                self.components.remove(unused_mod.as_ref());
            }
        }
    }
}

pub fn get_components(spec: &Spec) -> anyhow::Result<Components> {
    let mut components = IndexMap::with_capacity(spec.component_schemas().len());

    let mut resource_map = HashMap::new();
    let mut obj_map = IndexMap::new();
    let mut id_map = HashMap::new();
    for path in spec.component_schemas().keys() {
        let path = ComponentPath::new(path.clone());
        let schema = spec.get_component_schema(&path);
        let stripe_resource = StripeResource::from_schema(schema, path.clone())?;
        let data = parse_stripe_schema_as_rust_object(schema, &path, &stripe_resource.base_ident);
        if let Some(obj_name) = &data.object_name {
            if let Some(id_typ) = data.id_type.as_ref().and_then(|t| t.as_id_path()) {
                id_map.insert(obj_name.clone(), id_typ.clone());
            }
        }
        resource_map.insert(path.clone(), stripe_resource);
        obj_map.insert(path, data);
    }

    for (path, data) in obj_map {
        let resource = resource_map.get(&path).unwrap();
        let schema = spec.get_component_schema(&path);
        let stripe_reqs: Vec<StripeOperation> = if let Some(val) = schema.schema_data.extensions.get("x-stripeOperations") { serde_json::from_value(val.clone())? } else { vec![] };

        let inferred_krate = maybe_infer_crate(&path, resource.in_package.as_deref());
        let reqs = parse_requests(stripe_reqs, spec, &resource.base_ident, &path, &id_map)?;
        components.insert(path, StripeObject { requests: reqs, resource: resource.clone(), data, krate: inferred_krate.map(CrateInfo::new) });
    }

    let mut components = Components { components };
    components.filter_unused_components();

    validate_crate_map(&components)?;
    components.infer_all_crate_assignments()?;
    Ok(components)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OverrideMetadata {
    pub ident: RustIdent,
    pub doc: String,
    pub mod_path: String,
}

pub struct Overrides {
    pub overrides: IndexMap<RustObject, OverrideMetadata>,
}

#[derive(Debug, Copy, Clone)]
pub struct OverrideData {
    pub doc: &'static str,
    pub mod_path: &'static str,
    pub ident: &'static str,
    pub source: OverrideSource,
}

#[derive(Debug, Copy, Clone)]
pub struct RequestOverrideSource {
    path: &'static str,
    op: OperationType,
    field_name: &'static str,
}

#[derive(Debug, Copy, Clone)]
pub enum OverrideSource {
    Request(RequestOverrideSource),
}

const OVERRIDES: &[OverrideData] = &[OverrideData { doc: "", mod_path: "api_version", ident: "ApiVersion", source: OverrideSource::Request(RequestOverrideSource { path: "/v1/webhook_endpoints", op: OperationType::Post, field_name: "api_version" }) }];

fn get_override_object(spec: &Spec, data: &OverrideData) -> anyhow::Result<(RustObject, OverrideMetadata)> {
    match data.source {
        OverrideSource::Request(req_src) => {
            let op = spec.get_request_operation(req_src.path, req_src.op).context("Request not found")?;
            let form_params = get_request_form_parameters(op).context("No form params")?.as_item().context("Was a ref")?;
            let typ = as_object_properties(form_params).context("Not an object")?;
            let schema = typ.get(req_src.field_name).context("Field not found")?;
            let ident = RustIdent::create(data.ident);
            let (obj, _) = Inference::new(&ident).infer_schema_or_ref_type(schema).into_object().context("Expected object type to be inferred")?;
            Ok((obj, OverrideMetadata { ident, doc: data.doc.to_string(), mod_path: data.mod_path.to_string() }))
        }
    }
}

pub fn build_field_overrides(spec: &Spec) -> anyhow::Result<Overrides> {
    let mut overrides = IndexMap::new();
    for override_ in OVERRIDES {
        let (obj, meta) = get_override_object(spec, override_).with_context(|| format!("Failed to construct override for source {override_:?}"))?;
        overrides.insert(obj, meta);
    }
    Ok(Overrides { overrides })
}

impl Overrides {
    pub fn replace_typ(&self, typ: &mut RustType) {
        match typ {
            RustType::Object(obj, _) => {
                if let Some(meta) = self.overrides.get(obj) {
                    *typ = RustType::Path { path: PathToType::Types(meta.ident.clone()), is_ref: false, has_reference: obj.has_reference(), is_copy: obj.is_copy() }
                } else {
                    self.replace_obj(obj);
                }
            }
            RustType::Simple(_) => {}
            RustType::Path { .. } => {}
            RustType::Container(inner) => {
                self.replace_typ(inner.value_typ_mut());
            }
        }
    }

    fn replace_obj(&self, obj: &mut RustObject) {
        match obj {
            RustObject::Struct(fields) => {
                for field in fields {
                    self.replace_typ(&mut field.rust_type);
                }
            }
            RustObject::FieldlessEnum(_) => {}
            RustObject::Enum(variants) => {
                for var in variants {
                    if let Some(typ) = &mut var.rust_type {
                        self.replace_typ(typ);
                    }
                }
            }
        }
    }
}

impl Components {
    pub fn apply_overrides(&mut self, overrides: &Overrides) {
        for obj in self.components.values_mut() {
            let obj_typ = &mut obj.data.obj;
            overrides.replace_obj(obj_typ);

            for req in &mut obj.requests {
                if let Some(typ) = &mut req.params {
                    overrides.replace_typ(typ);
                }
                overrides.replace_typ(&mut req.returned);
            }
        }
    }
}
