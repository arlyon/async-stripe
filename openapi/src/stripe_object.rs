use anyhow::{anyhow, Context};
use heck::SnakeCase;
use openapiv3::Schema;
use serde::{Deserialize, Serialize};

use crate::components::ModuleName;
use crate::rust_object::RustObject;
use crate::rust_type::RustType;
use crate::spec_inference::Inference;
use crate::types::{ComponentPath, RustIdent};

#[derive(Debug, Clone)]
pub struct StripeObject {
    pub requests: Vec<RequestSpec>,
    pub resource: StripeResource,
    pub data: StripeObjectData,
    pub module: ModuleName,
}

#[derive(Debug, Clone)]
pub struct StripeObjectData {
    pub obj: RustObject,
    pub object_name: Option<String>,
    pub id_type: Option<RustType>,
}

impl StripeObject {
    pub fn is_deleted_item(&self) -> bool {
        self.path().is_deleted()
    }

    pub fn mod_path(&self) -> String {
        self.resource.mod_path()
    }

    pub fn path(&self) -> &ComponentPath {
        &self.resource.path
    }

    pub fn ident(&self) -> &RustIdent {
        self.resource.ident()
    }

    pub fn object_name(&self) -> Option<&str> {
        self.data.object_name.as_deref()
    }

    pub fn id_type(&self) -> Option<&RustType> {
        self.data.id_type.as_ref()
    }

    pub fn inner_classes(&self) -> &[ComponentPath] {
        &self.resource.inner_classes
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
    let (rust_obj, _) = typ.into_object().expect("Unexpected top level schema type");
    match &rust_obj {
        RustObject::Struct(fields) => {
            let mut id_type = None;
            let mut object_name = None;
            for field in fields {
                if field.field_name == "id" {
                    id_type = Some(field.rust_type.clone());
                }
                if field.field_name == "object" {
                    if let Some(RustObject::Enum(variants)) = field.rust_type.as_rust_object() {
                        if variants.variants.len() == 1 {
                            let first = variants.variants.first().unwrap();
                            object_name = Some(first.wire_name.clone());
                        }
                    }
                }
            }
            StripeObjectData { obj: rust_obj, object_name, id_type }
        }
        RustObject::FieldedEnum(_) => {
            // TODO: could validate that this enum holds what we expect from a top-level component,
            // namely a union of references to other components. We also are implicitly assuming
            // these components have ids
            StripeObjectData { obj: rust_obj, object_name: None, id_type: Some(RustType::string()) }
        }
        RustObject::Enum(_) => panic!("Unexpected top level schema type"),
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
    #[serde(default)]
    pub inner_classes: Vec<String>,
    pub in_class: Option<String>,
    pub in_package: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StripeResource {
    pub path: ComponentPath,
    pub inner_classes: Vec<ComponentPath>,
    pub base_ident: RustIdent,
    pub in_class: Option<ComponentPath>,
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

fn maybe_force_ident(class_: &str) -> Option<RustIdent> {
    if class_ == "StripeError" {
        Some(RustIdent::create("APIErrors"))
    } else {
        None
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
        let mut in_class = None;
        let mut in_package = None;
        if let Some(class_) = resource.in_class {
            if !class_.is_empty() {
                in_class = Some(ComponentPath::new(class_));
            }
        }
        if let Some(package) = resource.in_package {
            if !package.is_empty() {
                in_package = Some(package.to_snake_case());
            }
        }

        let class_ = &resource.class_name;
        let ident = if let Some(forced) = maybe_force_ident(class_) {
            forced
        } else {
            RustIdent::create(class_)
        };
        let requests = if let Some(val) = schema.schema_data.extensions.get("x-stripeOperations") {
            serde_json::from_value(val.clone())?
        } else {
            vec![]
        };
        Ok(StripeResource {
            inner_classes: resource.inner_classes.into_iter().map(ComponentPath::new).collect(),
            in_class,
            base_ident: ident,
            in_package,
            requests,
            path,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RequestSpec {
    pub path_params: Vec<PathParam>,
    pub params: Option<RustType>,
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
