use std::collections::HashMap;
use std::path::PathBuf;

use heck::ToSnakeCase;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use openapiv3::Schema;
use serde::{Deserialize, Serialize};

use crate::components::Components;
use crate::crates::Crate;
use crate::deduplication::DeduppedObject;
use crate::rust_object::{ObjectUsage, RustObject, StructField};
use crate::rust_type::RustType;
use crate::spec_inference::Inference;
use crate::types::{ComponentPath, RustIdent};
use crate::visitor::{Visit, VisitMut};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct CrateInfo {
    krate: Crate,
    type_defs_are_shared: bool,
}

impl CrateInfo {
    pub fn new(krate: Crate) -> Self {
        Self { krate, type_defs_are_shared: false }
    }

    pub fn set_share_type_defs(&mut self) {
        self.type_defs_are_shared = true;
    }

    pub fn are_type_defs_shared(&self) -> bool {
        self.type_defs_are_shared || self.krate == Crate::SHARED
    }

    pub fn for_types(&self) -> Crate {
        if self.type_defs_are_shared {
            Crate::SHARED
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
    pub stripe_doc_url: Option<String>,
    pub deduplicated_objects: IndexMap<RustIdent, DeduppedObject>,
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

    pub fn mod_path(&self) -> String {
        self.resource.mod_path()
    }

    pub fn object_name(&self) -> Option<&str> {
        self.data.object_name.as_deref()
    }

    pub fn types_split_from_requests(&self) -> bool {
        self.types_are_shared() && self.krate_unwrapped().base() != Crate::SHARED
    }

    /// Do schema definitions live in `async-stripe-shared`?
    pub fn types_are_shared(&self) -> bool {
        let krate = self.krate_unwrapped();
        krate.for_types() == Crate::SHARED
    }

    /// The crate we write schema definitions to
    pub fn types_crate(&self) -> Crate {
        let krate = self.krate_unwrapped();
        krate.for_types()
    }

    /// The crate we write request content to
    pub fn req_crate(&self) -> Crate {
        let krate = self.krate_unwrapped();
        krate.base()
    }

    fn get_requests_folder_path(&self) -> PathBuf {
        self.req_crate().get_path().join(self.mod_path())
    }

    pub fn get_requests_module_path(&self) -> PathBuf {
        self.get_requests_folder_path().join("mod.rs")
    }

    pub fn get_requests_content_path(&self) -> PathBuf {
        self.get_requests_folder_path().join("requests.rs")
    }

    pub fn get_types_content_path(&self) -> PathBuf {
        if !self.has_requests() || self.types_split_from_requests() {
            self.types_crate().get_path().join(format!("{}.rs", self.mod_path()))
        } else {
            self.get_requests_folder_path().join("types.rs")
        }
    }

    pub fn has_requests(&self) -> bool {
        !self.requests.is_empty()
    }

    pub fn path(&self) -> &ComponentPath {
        &self.resource.path
    }

    pub fn ident(&self) -> &RustIdent {
        self.resource.ident()
    }

    pub fn id_type_ident(&self) -> RustIdent {
        RustIdent::create(format!("{}Id", self.ident()))
    }

    pub fn object_name_ident(&self) -> RustIdent {
        RustIdent::create(format!("{}ObjectName", self.ident()))
    }

    pub fn id_type(&self) -> Option<&RustType> {
        self.data.id_type.as_ref()
    }

    pub fn rust_obj(&self) -> &RustObject {
        &self.data.obj
    }

    pub fn is_nested_resource_of(&self, other: &Self) -> bool {
        if self.requests.is_empty() {
            return false;
        }
        self.requests.iter().all(|r| {
            let start = format!("/{}", other.path());
            r.req_path.starts_with(&start)
        })
    }

    pub fn visit<'a, V: Visit<'a>>(&'a self, visitor: &mut V) {
        visitor.visit_obj(&self.data.obj, None, ObjectUsage::type_def());
        for req in &self.requests {
            visitor.visit_req(req);
        }
    }

    pub fn visit_mut<V: VisitMut>(&mut self, visitor: &mut V) {
        visitor.visit_obj_mut(&mut self.data.obj, None, ObjectUsage::type_def());
        for req in &mut self.requests {
            visitor.visit_req_mut(req);
        }
    }
}

#[derive(Debug, Clone)]
pub struct StripeObjectData {
    pub obj: RustObject,
    pub object_name: Option<String>,
    pub id_type: Option<RustType>,
}

pub fn parse_stripe_schema_as_rust_object(
    schema: &Schema,
    path: &ComponentPath,
    ident: &RustIdent,
) -> StripeObjectData {
    let not_deleted_path = path.as_not_deleted();
    let infer_ctx = Inference::new(ident).id_path(&not_deleted_path).required(true);
    let typ = infer_ctx.infer_schema_type(schema);
    let Some((mut rust_obj, _)) = typ.into_object() else {
        panic!("Unexpected top level schema type for {}", path);
    };
    match &mut rust_obj {
        RustObject::Struct(struct_) => {
            let mut id_type = None;
            let mut object_name = None;
            struct_.fields.retain(|field| {
                if field.field_name == "id" && field.rust_type.as_id_or_opt_id_path().is_some() {
                    id_type = Some(field.rust_type.clone());
                }
                if field.field_name == "object" {
                    if let Some(RustObject::FieldlessEnum(variants)) =
                        field.rust_type.as_rust_object()
                    {
                        if variants.len() == 1 {
                            let first = variants.first().unwrap();
                            object_name = Some(first.wire_name.clone());

                            // This constant field just helps serialize the constant "object"
                            // key - we don't want it as part of the public API needed to
                            // construct these types
                            return false;
                        }
                    }
                }
                true
            });
            if let Some(obj_name) = &object_name {
                struct_.object_field = Some(obj_name.clone());
            }
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

#[derive(Debug, Clone, Deserialize, Default)]
struct BaseResource {
    pub in_package: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StripeResource {
    pub path: ComponentPath,
    base_ident: RustIdent,
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
        let resource =
            if let Some(stripe_resource) = schema.schema_data.extensions.get("x-stripeResource") {
                serde_json::from_value(stripe_resource.clone())?
            } else {
                BaseResource::default()
            };
        let mut in_package = None;
        if let Some(package) = resource.in_package {
            if !package.is_empty() {
                in_package = Some(package.to_snake_case());
            }
        }

        let ident = infer_object_ident(&path);
        let requests = if let Some(val) = schema.schema_data.extensions.get("x-stripeOperations") {
            serde_json::from_value(val.clone())?
        } else {
            vec![]
        };
        Ok(Self { base_ident: ident, in_package, requests, path })
    }
}

fn infer_object_ident(path: &ComponentPath) -> RustIdent {
    lazy_static! {
        static ref OBJECT_RENAMES: HashMap<&'static str, &'static str> = HashMap::from([
            ("invoiceitem", "invoice_item"),
            ("item", "checkout_session_item"),
            ("line_item", "invoice_line_item"),
            ("fee_refund", "application_fee_refund"),
        ]);
    }
    if let Some(renamed) = OBJECT_RENAMES.get(path.as_ref()) {
        RustIdent::create(renamed)
    } else {
        RustIdent::create(path)
    }
}

#[derive(Debug, Clone)]
pub struct RequestSpec {
    pub ident: RustIdent,
    pub path_params: Vec<PathParam>,
    pub params: Option<RequestParam>,
    pub method_type: OperationType,
    pub returned: RustType,
    pub doc_comment: Option<String>,
    pub req_path: String,
    pub method_name: String,
}

#[derive(Debug, Clone)]
pub struct RequestParam {
    pub ident: RustIdent,
    pub typ: RustType,
}

impl RequestSpec {
    pub fn visit<'a, V: Visit<'a>>(&'a self, visitor: &mut V) {
        visitor.visit_typ(&self.returned, ObjectUsage::return_type());
        if let Some(params) = &self.params {
            visitor.visit_typ(&params.typ, ObjectUsage::request_param());
        }
    }

    pub fn visit_mut<V: VisitMut>(&mut self, visitor: &mut V) {
        visitor.visit_typ_mut(&mut self.returned, ObjectUsage::return_type());
        if let Some(params) = &mut self.params {
            visitor.visit_typ_mut(&mut params.typ, ObjectUsage::request_param());
        }
    }

    pub fn has_reference(&self, components: &Components) -> bool {
        if self.path_params.iter().any(|p| p.rust_type.has_reference(components)) {
            return true;
        }
        let Some(params) = &self.params else {
            return false;
        };
        params.typ.has_reference(components)
    }

    pub fn param_struct_fields(&self) -> Option<&[StructField]> {
        let params = self.params.as_ref()?;
        match &params.typ.as_rust_object()? {
            RustObject::Struct(struct_) => Some(&struct_.fields),
            _ => None,
        }
    }

    pub fn get_param_field(&self, field: &str) -> Option<&RustType> {
        self.params.as_ref()?.typ.as_rust_object()?.get_struct_field(field)
    }
}

#[derive(Debug, Clone)]
pub struct PathParam {
    pub name: String,
    pub rust_type: RustType,
}
