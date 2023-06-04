use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use anyhow::Context;
use indexmap::IndexMap;

use crate::object_context::PrintableType;
use crate::requests::parse_requests;
use crate::rust_type::RustType;
use crate::spec::Spec;
use crate::spec_inference::infer_id_name;
use crate::stripe_object::{parse_stripe_data, StripeObject};
use crate::types::{ComponentPath, RustIdent};
use crate::url_finder::UrlFinder;

#[derive(Debug, Clone)]
pub enum Module {
    Package(String, Vec<ComponentPath>),
    Component(ComponentPath),
}

pub struct Components {
    pub modules: IndexMap<String, Module>,
    components: IndexMap<String, StripeObject>,
}

impl Components {
    #[track_caller]
    pub fn get(&self, path: &ComponentPath) -> &StripeObject {
        &self.components[path.as_ref()]
    }

    pub fn parent_for_deleted(&self, path: &ComponentPath) -> &StripeObject {
        assert!(path.starts_with("deleted_"), "Expected path for deleted object");
        let parent =
            self.components.get(path.trim_start_matches("deleted_")).expect("Component not found");
        parent
    }

    pub fn maybe_get(&self, path: &str) -> Option<&StripeObject> {
        self.components.get(path)
    }

    pub fn containing_module(&self, path: &ComponentPath) -> String {
        let obj = self.get(path);
        let mut parent = obj;
        while let Some(in_class) = &parent.resource.in_class {
            parent = self.get(in_class);
        }
        if let Some(package_name) = &parent.resource.in_package {
            package_name.to_string()
        } else {
            parent.mod_path()
        }
    }

    #[track_caller]
    pub fn resolve_path(&self, path: &ComponentPath) -> String {
        let component = self.get(path);
        let mut pieces = vec![];
        let mut parent =
            if component.is_deleted_item() { self.parent_for_deleted(path) } else { component };
        pieces.push(parent.mod_path());
        while let Some(in_class) = &parent.resource.in_class {
            parent = self.get(in_class);
            pieces.push(parent.mod_path());
        }
        if let Some(package) = &parent.resource.in_package {
            pieces.push(package.to_string());
        }
        let mut path: String = "crate".into();
        for path_piece in pieces.iter().rev() {
            let _ = write!(path, "::{path_piece}");
        }
        path
    }

    #[track_caller]
    pub fn resolve_path_with_ident(&self, path: &ComponentPath, ident: &RustIdent) -> String {
        let mut path = self.resolve_path(path);
        let _ = write!(path, "::{ident}");
        path
    }

    pub fn construct_printable_type(&self, typ: &RustType) -> PrintableType {
        match typ {
            RustType::Object(obj) => PrintableType::QualifiedPath {
                path: obj.ident.to_string(),
                has_borrow: obj.data.has_borrow(),
                is_borrowed: false,
            },

            RustType::Ref(path) => {
                let comp = self.get(path);
                PrintableType::QualifiedPath {
                    path: self.resolve_path_with_ident(path, comp.ident()),
                    is_borrowed: false,
                    has_borrow: false,
                }
            }
            RustType::Simple(typ) => PrintableType::Simple(*typ),
            RustType::Compound(kind, typ) => {
                let inner = self.construct_printable_type(typ);
                PrintableType::Compound(*kind, Box::new(inner))
            }
            RustType::ObjectId { path, borrowed } => {
                let ident = infer_id_name(path);
                PrintableType::QualifiedPath {
                    path: self.resolve_path_with_ident(path, &ident),
                    has_borrow: false,
                    is_borrowed: *borrowed,
                }
            }
        }
    }
}

pub fn get_components(spec: &Spec, url_finder: &UrlFinder) -> anyhow::Result<Components> {
    let mut components = IndexMap::with_capacity(spec.component_schemas().len());

    let mut data = vec![];
    let mut id_map = HashMap::new();
    for (path, schema) in spec.component_schemas() {
        let component_path = ComponentPath::new(path.clone());
        let schema = schema.as_item().expect("Expected top level component to be an item");
        let doc_url = url_finder.url_for_object(path);
        let stripe_data = parse_stripe_data(schema, component_path, doc_url.as_deref())
            .with_context(|| format!("Could not construct stripe object at path {}", path))?;
        let (_, _, obj_data, _) = &stripe_data;
        if let Some(obj_name) = &obj_data.object_name {
            if let Some(id_typ) = obj_data.id_type.as_ref().and_then(|t| t.as_id_path()) {
                id_map.insert(obj_name.clone(), id_typ.clone());
            }
        }
        data.push(stripe_data);
    }

    for (resource, operations, obj_data, path) in data {
        let reqs = parse_requests(operations, spec, &resource.base_ident, &id_map)?;
        components.insert(
            path.to_string(),
            StripeObject { requests: reqs, path, resource, data: obj_data },
        );
    }

    let mut modules = IndexMap::new();
    let mut packages = HashSet::new();
    for (path, obj) in &components {
        if obj.resource.in_package.is_none()
            && obj.resource.in_class.is_none()
            && !obj.is_deleted_item()
        {
            modules.insert(obj.mod_path(), Module::Component(ComponentPath::new(path.clone())));
        }
        if let Some(package) = &obj.resource.in_package {
            packages.insert(package.to_string());
        }
    }
    for package in packages {
        let mut members = vec![];
        for obj in components.values() {
            if obj.resource.in_package.as_deref() == Some(&package)
                && obj.resource.in_class.is_none()
                && !obj.is_deleted_item()
            {
                members.push(obj.path.clone());
            }
        }
        modules.insert(package.clone(), Module::Package(package, members));
    }
    Ok(Components { modules, components })
}
