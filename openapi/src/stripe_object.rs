use anyhow::{anyhow, Context};
use heck::SnakeCase;
use openapiv3::Schema;
use serde::{Deserialize, Serialize};

use crate::crate_inference::Crate;
use crate::rust_object::RustObject;
use crate::rust_type::RustType;
use crate::spec_inference::Inference;
use crate::types::{ComponentPath, RustIdent};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct CrateInfo {
    krate: Crate,
    type_defs_in_stripe_types: bool,
}

impl CrateInfo {
    pub fn new(krate: Crate) -> Self {
        Self { krate, type_defs_in_stripe_types: false }
    }

    pub fn set_type_defs_in_types_crate(&mut self) {
        self.type_defs_in_stripe_types = true;
    }

    pub fn are_type_defs_types_crate(&self) -> bool {
        self.type_defs_in_stripe_types || self.krate == Crate::Types
    }

    pub fn for_types(&self) -> Crate {
        if self.type_defs_in_stripe_types {
            Crate::Types
        } else {
            self.krate
        }
    }

    pub fn base(&self) -> Crate {
        self.krate
    }
}

#[derive(Debug, Clone)]
pub struct StripeObject {
    pub requests: Vec<RequestSpec>,
    pub resource: StripeResource,
    pub data: StripeObjectData,
    pub krate: Option<CrateInfo>,
}

impl StripeObject {
    pub fn krate(&self) -> Option<CrateInfo> {
        self.krate
    }

    #[track_caller]
    pub fn krate_unwrapped(&self) -> CrateInfo {
        let Some(krate) = self.krate() else {
            panic!("Has no crate assigned: \n{:?}", self);
        };
        krate
    }

    #[track_caller]
    pub fn krate_unwrapped_mut(&mut self) -> &mut CrateInfo {
        self.krate.as_mut().expect("No crate assigned")
    }

    pub fn assign_crate(&mut self, new_krate: Crate) {
        self.krate = Some(CrateInfo::new(new_krate));
    }
}

#[derive(Debug, Clone)]
pub struct StripeObjectData {
    pub obj: RustObject,
    pub object_name: Option<String>,
    pub id_type: Option<RustType>,
}

impl StripeObject {
    pub fn mod_path(&self) -> String {
        self.resource.mod_path()
    }

    pub fn path(&self) -> &ComponentPath {
        &self.resource.path
    }

    pub fn ident(&self) -> &RustIdent {
        self.resource.ident()
    }

    pub fn id_type(&self) -> Option<&RustType> {
        self.data.id_type.as_ref()
    }

    pub fn rust_obj(&self) -> &RustObject {
        &self.data.obj
    }

    pub fn is_nested_resource_of(&self, other: &StripeObject) -> bool {
        if self.requests.is_empty() {
            return false;
        }
        self.requests.iter().all(|r| {
            let start = format!("/{}", other.path());
            r.req_path.starts_with(&start)
        })
    }
}

pub fn parse_stripe_schema_as_rust_object(
    schema: &Schema,
    path: &ComponentPath,
    ident: &RustIdent,
) -> StripeObjectData {
    let not_deleted_path = path.as_not_deleted();
    let infer_ctx = Inference::new(ident).id_path(&not_deleted_path);
    let typ = infer_ctx.infer_schema_type(schema);
    let (mut rust_obj, _) = typ.into_object().expect("Unexpected top level schema type");
    match &mut rust_obj {
        RustObject::Struct(fields) => {
            let mut id_type = None;
            let mut object_name = None;
            fields.retain(|field| {
                if field.field_name == "id" {
                    id_type = Some(field.rust_type.clone());
                }
                if field.field_name == "object" {
                    if let Some(RustObject::FieldlessEnum(variants)) =
                        field.rust_type.as_rust_object()
                    {
                        if variants.len() == 1 {
                            let first = variants.first().unwrap();
                            object_name = Some(first.wire_name.clone());
                            // The object name is used purely as a discriminant, so is
                            // unnecessary to generate 1-enum type for.
                            return false;
                        }
                    }
                }
                true
            });
            StripeObjectData { obj: rust_obj, object_name, id_type }
        }
        RustObject::Enum(_) => {
            // TODO: could validate that this enum holds what we expect from a top-level component,
            // namely a union of references to other components. We also are implicitly assuming
            // these components have ids
            StripeObjectData { obj: rust_obj, object_name: None, id_type: Some(RustType::string()) }
        }
        RustObject::FieldlessEnum(_) => panic!("Unexpected top level schema type"),
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StripeOperation {
    pub method_name: String,
    pub method_type: MethodType,
    pub method_on: String,
    #[serde(rename = "operation")]
    pub operation_type: OperationType,
    pub path: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MethodType {
    Retrieve,
    List,
    Create,
    Update,
    Delete,
    Custom,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum OperationType {
    Get,
    Post,
    Delete,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BaseResource {
    pub class_name: String,
    pub in_package: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StripeResource {
    pub path: ComponentPath,
    pub base_ident: RustIdent,
    pub in_package: Option<String>,
    pub requests: Vec<StripeOperation>,
}

impl StripeResource {
    pub fn mod_path(&self) -> String {
        self.ident().to_snake_case()
    }

    pub fn ident(&self) -> &RustIdent {
        &self.base_ident
    }
}

impl StripeResource {
    pub fn from_schema(schema: &Schema, path: ComponentPath) -> anyhow::Result<Self> {
        let resource: BaseResource = schema
            .schema_data
            .extensions
            .get("x-stripeResource")
            .context("No stripe resource")
            .and_then(|r| serde_json::from_value(r.clone()).map_err(|err| anyhow!(err)))?;

        let mut in_package = None;
        if let Some(package) = resource.in_package {
            if !package.is_empty() {
                in_package = Some(package.to_snake_case());
            }
        }

        let ident = infer_object_ident(&path, &schema.schema_data.title, &resource.class_name);
        let requests = if let Some(val) = schema.schema_data.extensions.get("x-stripeOperations") {
            serde_json::from_value(val.clone())?
        } else {
            vec![]
        };
        Ok(StripeResource { base_ident: ident, in_package, requests, path })
    }
}

fn infer_object_ident(path: &ComponentPath, title: &Option<String>, class: &str) -> RustIdent {
    let Some(title) = title else {
        return RustIdent::create(path);
    };
    if title == "Polymorphic" {
        return RustIdent::create(class);
    }
    RustIdent::create(title)
}

#[derive(Debug, Clone)]
pub struct RequestSpec {
    pub path_params: Vec<PathParam>,
    pub params: RustType,
    pub method_type: OperationType,
    pub returned: RustType,
    pub doc_comment: Option<String>,
    pub req_path: String,
    pub method_name: String,
}

#[derive(Debug, Clone)]
pub struct PathParam {
    pub name: String,
    pub rust_type: RustType,
}
