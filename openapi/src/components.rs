use std::collections::HashMap;
use std::fmt::Debug;

use indexmap::{IndexMap, IndexSet};
use petgraph::Direction;
use tracing::{debug, info};

use crate::crate_inference::{maybe_infer_crate, validate_crate_map, Crate};
use crate::object_writing::ObjectGenInfo;
use crate::overrides::Overrides;
use crate::printable::{PrintableContainer, PrintableType};
use crate::requests::parse_requests;
use crate::rust_object::RustObject;
use crate::rust_type::{Container, PathToType, RustType};
use crate::spec::Spec;
use crate::spec_inference::infer_id_name;
use crate::stripe_object::{
    parse_stripe_schema_as_rust_object, CrateInfo, OperationType, RequestSpec, StripeObject,
    StripeOperation, StripeResource,
};
use crate::types::{ComponentPath, RustIdent};

#[derive(Clone, Debug)]
pub struct TypeSpec {
    pub doc: Option<String>,
    pub gen_info: ObjectGenInfo,
    pub obj: RustObject,
    pub mod_path: String,
}

pub struct Components {
    pub components: IndexMap<ComponentPath, StripeObject>,
    pub extra_types: IndexMap<RustIdent, TypeSpec>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PathInfo {
    pub krate: Crate,
    pub path: Option<String>,
}

impl PathInfo {
    pub fn new(krate: Crate) -> Self {
        Self { krate, path: None }
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
        PathInfo { krate, path: Some(component.mod_path()) }
    }

    pub fn construct_printable_type_from_path(&self, path: &ComponentPath) -> PrintableType {
        let comp = self.get(path);
        PrintableType::QualifiedPath {
            path: Some(PathInfo::new(comp.krate_unwrapped().for_types())),
            ident: comp.ident().clone(),
            has_ref: false,
            is_ref: false,
        }
    }

    pub fn construct_printable_type(&self, typ: &RustType) -> PrintableType {
        match typ {
            RustType::Object(obj, metadata) => PrintableType::QualifiedPath {
                path: None,
                has_ref: obj.has_reference(self),
                is_ref: false,
                ident: metadata.ident.clone(),
            },
            RustType::Path { path: PathToType::Component(path), .. } => {
                self.construct_printable_type_from_path(path)
            }
            RustType::Path { path: PathToType::ObjectId(path), is_ref } => {
                let ident = infer_id_name(path);
                let path_info = self.resolve_path(path);
                PrintableType::QualifiedPath {
                    path: Some(path_info),
                    ident,
                    is_ref: *is_ref,
                    has_ref: false,
                }
            }
            RustType::Path { path: PathToType::Type(ident), is_ref } => {
                let referred_typ = self.get_extra_type(ident);
                let has_ref = referred_typ.obj.has_reference(self);
                PrintableType::QualifiedPath {
                    path: Some(PathInfo { krate: Crate::Types, path: None }),
                    has_ref,
                    is_ref: *is_ref,
                    ident: ident.clone(),
                }
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
                    Container::Map { key, is_ref, .. } => {
                        PrintableContainer::Map { key: *key, is_ref: *is_ref, value: inner }
                    }
                };
                PrintableType::Container(printable)
            }
        }
    }

    pub fn add_component_deps<'a>(
        &'a self,
        deps: &mut IndexSet<&'a ComponentPath>,
        path: &'a ComponentPath,
    ) {
        let component = self.get(path);
        let base_type = component.rust_obj();
        self.add_deps_from_obj(deps, base_type);
        let reqs = &component.requests;
        let mut typs = vec![];
        for req in reqs {
            typs.push(&req.params);
            typs.push(&req.returned);
            for path_param in &req.path_params {
                typs.push(&path_param.rust_type);
            }
        }
        for typ in typs {
            self.add_deps_from_typ(deps, typ);
        }
    }

    pub fn add_deps_from_obj<'a>(
        &'a self,
        deps: &mut IndexSet<&'a ComponentPath>,
        obj: &'a RustObject,
    ) {
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

    pub fn add_deps_from_typ<'a>(
        &'a self,
        deps: &mut IndexSet<&'a ComponentPath>,
        typ: &'a RustType,
    ) {
        match typ {
            RustType::Object(obj, _) => self.add_deps_from_obj(deps, obj),
            RustType::Path { path: PathToType::Component(path), .. } => {
                deps.insert(path);
            }
            RustType::Container(inner) => self.add_deps_from_typ(deps, inner.value_typ()),
            _ => {}
        }
    }

    pub fn deps_for_component<'a>(
        &'a self,
        path: &'a ComponentPath,
    ) -> IndexSet<&'a ComponentPath> {
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
                if component.requests.is_empty()
                    && graph.neighbors_directed(path, Direction::Incoming).count() == 0
                {
                    unused.push(path.clone());
                }
            }
            let done_filtering = unused.is_empty();
            if done_filtering {
                break;
            }

            debug!("Filtering unused components: {unused:#?}");
            for unused_mod in unused.drain(..) {
                self.components.remove(unused_mod.as_ref());
            }
        }
    }

    pub fn get_request_spec(&self, src: RequestSource) -> Option<&RequestSpec> {
        for comp in self.components.values() {
            for req in &comp.requests {
                if req.method_type == src.op && req.req_path == src.path {
                    return Some(req);
                }
            }
        }
        None
    }

    #[tracing::instrument(level = "debug", skip(self))]
    fn apply_overrides(&mut self) -> anyhow::Result<()> {
        let mut overrides = Overrides::new(self)?;
        for comp in self.components.values_mut() {
            comp.visit_mut(&mut overrides);
        }
        for (obj, override_meta) in overrides.overrides {
            self.extra_types.insert(
                override_meta.ident,
                TypeSpec {
                    doc: Some(override_meta.doc),
                    mod_path: override_meta.mod_path,
                    gen_info: ObjectGenInfo::new_deser(),
                    obj,
                },
            );
        }

        Ok(())
    }

    #[track_caller]
    pub fn get_extra_type(&self, ident: &RustIdent) -> &TypeSpec {
        &self.extra_types[ident]
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
        let stripe_reqs: Vec<StripeOperation> =
            if let Some(val) = schema.schema_data.extensions.get("x-stripeOperations") {
                serde_json::from_value(val.clone())?
            } else {
                vec![]
            };

        let inferred_krate = maybe_infer_crate(&path, resource.in_package.as_deref());
        let reqs = parse_requests(stripe_reqs, spec, &resource.base_ident, &path, &id_map)?;
        components.insert(
            path,
            StripeObject {
                requests: reqs,
                resource: resource.clone(),
                data,
                krate: inferred_krate.map(CrateInfo::new),
                extra_types: Default::default(),
            },
        );
    }

    let mut components = Components { components, extra_types: Default::default() };
    components.filter_unused_components();

    validate_crate_map(&components)?;
    components.infer_all_crate_assignments()?;
    info!("Finished inferring crates");

    components.apply_overrides()?;
    debug!("Finished applying overrides");

    Ok(components)
}

#[derive(Debug, Copy, Clone)]
pub struct RequestSource {
    pub path: &'static str,
    pub op: OperationType,
    pub field_name: &'static str,
}
