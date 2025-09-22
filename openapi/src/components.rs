use std::collections::HashMap;
use std::fmt::Debug;

use anyhow::bail;
use indexmap::{IndexMap, IndexSet};
use petgraph::Direction;
use tracing::{debug, info};

use crate::crate_inference::validate_crate_info;
use crate::crates::{Crate, get_crate_for_package, maybe_use_hardcoded_crate_assignment};
use crate::deduplication::{DeduppedObject, deduplicate_types};
use crate::overrides::Overrides;
use crate::printable::{PrintableContainer, PrintableType};
use crate::requests::parse_requests;
use crate::rust_object::{ObjectKind, ObjectUsage, RustObject};
use crate::rust_type::{Container, PathToType, RustType};
use crate::spec::Spec;
use crate::stripe_object::{
    CrateInfo, OperationType, RequestSpec, StripeObject, StripeOperation, StripeResource,
    parse_stripe_schema_as_rust_object,
};
use crate::types::{ComponentPath, RustIdent};
use crate::visitor::Visit;
use crate::webhook::WebhookObject;

#[derive(Clone, Debug)]
pub struct TypeSpec {
    pub obj: RustObject,
    pub doc: Option<String>,
    pub ident: RustIdent,
    pub mod_path: String,
    pub usage: ObjectUsage,
}

pub struct Components {
    pub components: IndexMap<ComponentPath, StripeObject>,
    pub webhook_objs: Vec<WebhookObject>,
    pub extra_types: IndexMap<RustIdent, TypeSpec>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
        &self.components[path]
    }

    #[track_caller]
    pub fn get_mut(&mut self, path: &ComponentPath) -> &mut StripeObject {
        &mut self.components[path]
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
                let obj = self.get(path);
                let krate = obj.krate_unwrapped().for_types();
                PrintableType::QualifiedPath {
                    path: Some(PathInfo::new(krate)),
                    ident: obj.id_type_ident(),
                    is_ref: *is_ref,
                    has_ref: false,
                }
            }
            RustType::Path { path: PathToType::Deduplicated { path, ident }, is_ref } => {
                let referred_typ = self.get_dedupped_type(ident, path);
                let has_ref = referred_typ.object.has_reference(self);
                let path = if referred_typ.info.usage.kind == ObjectKind::Type {
                    Some(PathInfo {
                        krate: self.get(path).krate_unwrapped().for_types(),
                        path: None,
                    })
                } else {
                    // For now, deduplicated request types always can just reference
                    // the same file
                    None
                };
                PrintableType::QualifiedPath {
                    path,
                    ident: ident.clone(),
                    is_ref: *is_ref,
                    has_ref,
                }
            }
            RustType::Path { path: PathToType::Shared(ident), is_ref } => {
                let referred_typ = self.get_extra_type(ident);
                let has_ref = referred_typ.obj.has_reference(self);
                PrintableType::QualifiedPath {
                    path: Some(PathInfo { krate: Crate::SHARED, path: None }),
                    ident: ident.clone(),
                    is_ref: *is_ref,
                    has_ref,
                }
            }

            RustType::Simple(typ) => PrintableType::Simple(*typ),
            RustType::Container(typ) => {
                let inner = Box::new(self.construct_printable_type(typ.value_typ()));
                let printable = match typ {
                    Container::List(_) => PrintableContainer::List(inner),
                    Container::SearchList(_) => PrintableContainer::SearchList(inner),
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

    pub fn deps_for_webhooks(&self) -> IndexSet<&ComponentPath> {
        let mut dep_collector = DependencyCollector::new();
        for obj in &self.webhook_objs {
            dep_collector.visit_typ(&obj.typ, ObjectUsage::type_def());
        }
        dep_collector.deps
    }

    pub fn deps_for_component<'a>(
        &'a self,
        path: &'a ComponentPath,
    ) -> IndexSet<&'a ComponentPath> {
        let mut dep_collector = DependencyCollector::new();
        let comp = self.get(path);
        dep_collector.visit_stripe_object(comp);
        dep_collector.deps
    }

    pub fn filter_unused_components(&mut self) {
        loop {
            let mut unused = vec![];
            let webhook_deps = self.deps_for_webhooks();
            let graph = self.gen_component_dep_graph();
            for (path, component) in &self.components {
                // The `error` component is a false positive since we don't include the error
                // type in dependency calculations, but every requests depends on `error`
                if path.as_ref() == "error" || webhook_deps.contains(path) {
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
            for unused_mod in unused {
                self.components.shift_remove(unused_mod.as_ref());
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

    fn run_deduplication_pass(&mut self) {
        for comp in self.components.values_mut() {
            let extra_typs = deduplicate_types(comp);
            for (ident, typ) in extra_typs {
                comp.deduplicated_objects.insert(ident, typ);
            }
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    fn apply_overrides(&mut self) -> anyhow::Result<()> {
        let mut overrides = Overrides::new(self)?;
        for comp in self.components.values_mut() {
            comp.visit_mut(&mut overrides);
        }
        for (obj, override_meta) in overrides.overrides {
            self.extra_types.insert(
                override_meta.metadata.ident.clone(),
                TypeSpec {
                    obj,
                    doc: override_meta.metadata.doc,
                    ident: override_meta.metadata.ident,
                    mod_path: override_meta.mod_path,
                    usage: ObjectUsage { kind: ObjectKind::Type, used_as_request_param: true },
                },
            );
        }

        Ok(())
    }

    #[track_caller]
    pub fn get_extra_type(&self, ident: &RustIdent) -> &TypeSpec {
        &self.extra_types[ident]
    }

    #[track_caller]
    pub fn get_dedupped_type(&self, ident: &RustIdent, comp: &ComponentPath) -> &DeduppedObject {
        &self.get(comp).deduplicated_objects[ident]
    }
}

#[tracing::instrument(skip_all)]
pub fn get_components(spec: &Spec) -> anyhow::Result<Components> {
    let mut webhook_objs = vec![];
    let mut components = IndexMap::with_capacity(spec.component_schemas().len());

    let mut resource_map = HashMap::new();
    let mut obj_map = IndexMap::new();
    let mut id_map = HashMap::new();
    for path in spec.component_schemas().keys() {
        let path = ComponentPath::new(path.clone());
        let schema = spec.get_component_schema(&path);

        if let Some(webhook_object) = WebhookObject::from_schema(schema)? {
            webhook_objs.push(webhook_object);
            continue;
        };

        let stripe_resource = StripeResource::from_schema(schema, path.clone())?;
        let data = parse_stripe_schema_as_rust_object(schema, &path, stripe_resource.ident());
        if let Some(obj_name) = &data.object_name {
            if let Some(id_typ) = data.id_type.as_ref().and_then(|t| t.as_id_or_opt_id_path()) {
                // We will hit duplicate object names from e.g. `account` and `deleted_account`, but sanity check nothing
                // worse can happen
                if let Some(orig) = id_map.insert(obj_name.clone(), id_typ.clone()) {
                    if &orig != id_typ {
                        bail!("Mismatched object name and id type for {obj_name}");
                    }
                }
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

        if resource.in_package.is_none() {
            tracing::debug!("No package for {path}. Using path name for inference instead");
        }

        let inferred_crate = resource
            .in_package
            .as_ref()
            .map(|package| get_crate_for_package(package))
            .transpose()?
            .or_else(|| maybe_use_hardcoded_crate_assignment(&path))
            .map(CrateInfo::new);

        let reqs = parse_requests(stripe_reqs, spec, resource.ident(), &id_map)?;
        components.insert(
            path,
            StripeObject {
                requests: reqs,
                resource: resource.clone(),
                data,
                krate: inferred_crate,
                stripe_doc_url: None,
                deduplicated_objects: IndexMap::default(),
            },
        );
    }

    let mut components = Components { components, webhook_objs, extra_types: Default::default() };
    components.filter_unused_components();

    validate_crate_info(&components)?;
    components.infer_all_crate_assignments()?;
    info!("Finished inferring crates");

    components.apply_overrides()?;
    debug!("Finished applying overrides");

    components.run_deduplication_pass();
    info!("Finished deduplication pass");

    Ok(components)
}

#[derive(Debug, Copy, Clone)]
pub struct RequestSource {
    pub path: &'static str,
    pub op: OperationType,
    pub field_name: &'static str,
}

#[derive(Default)]
struct DependencyCollector<'a> {
    deps: IndexSet<&'a ComponentPath>,
}

impl DependencyCollector<'_> {
    fn new() -> Self {
        Self::default()
    }
}

impl<'a> Visit<'a> for DependencyCollector<'a> {
    fn visit_typ(&mut self, typ: &'a RustType, object_usage: ObjectUsage)
    where
        Self: Sized,
    {
        match typ {
            RustType::Path { path: PathToType::Component(path), .. } => {
                self.deps.insert(path);
            }
            _ => typ.visit(self, object_usage),
        }
    }
}
