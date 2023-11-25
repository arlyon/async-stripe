use std::collections::HashMap;
use std::fs::File;

use anyhow::{bail, Context};
use heck::ToSnakeCase;
use lazy_static::lazy_static;
use openapiv3::{Parameter, ParameterData, ParameterSchemaOrContent, ReferenceOr, Schema};
use tracing::debug;

use crate::object_writing::ObjectGenInfo;
use crate::rust_object::{ObjectMetadata, RustObject};
use crate::rust_type::{RustType, SimpleType};
use crate::spec::{get_ok_response_schema, get_request_form_parameters, Spec};
use crate::spec_inference::Inference;
use crate::stripe_object::{OperationType, PathParam, RequestSpec, StripeOperation};
use crate::types::{ComponentPath, RustIdent};

/// Load overrides that hardcode the id types we should use for path parameters.
/// Map request path start to id path
fn load_request_path_param_id_overrides() -> anyhow::Result<HashMap<String, String>> {
    Ok(serde_json::from_reader(File::open("request_path_params.json")?)?)
}

lazy_static! {
    pub static ref PATH_PARAM_OVERRIDE: HashMap<String, String> =
        load_request_path_param_id_overrides().expect("Unable to load path params");
}

fn find_single_path_param_override(req_path: &str) -> Option<&'static str> {
    for (k, v) in &*PATH_PARAM_OVERRIDE {
        if k.starts_with(req_path) {
            return Some(v);
        }
    }
    None
}

/// Should we skip a currently unsupported request?
fn should_skip_request(op: &StripeOperation) -> bool {
    // TODO: what is the relevance of the `method_on` field? A small number of requests
    // use "collection" instead of "service", but the OpenAPI schema does not differentiate
    // so we just end up with duplicate requests if we don't skip like this
    op.method_on != "service"
        // Skip PDF download (binary format response not supported by client yet)
        || op.method_name == "pdf"
        // Skip file upload (form/multipart not supported by client yet)
        || (op.method_name == "create" && op.path == "/v1/files")
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct RequestKey<'a> {
    path: &'a str,
    method_type: OperationType,
}

impl<'a> RequestKey<'a> {
    pub fn from_op(op: &'a RequestDetails) -> Self {
        Self { path: op.path, method_type: op.operation_type }
    }
}

fn infer_method_names<'a>(operations: &'a [RequestDetails]) -> HashMap<RequestKey<'a>, String> {
    let mut names = HashMap::new();
    for op_spec in operations {
        let has_path_params = !op_spec.path_params.is_empty();
        let method_name = match op_spec.base_method_name {
            "retrieve" => {
                if has_path_params {
                    "retrieve"
                } else {
                    "retrieve_for_my_account"
                }
            }
            other => other,
        };
        // Explicit conversion to snake case handles a single method_name "upcomingLines" -
        // all other methods are already snake case
        names.insert(RequestKey::from_op(op_spec), method_name.to_snake_case());
    }

    deduplicate_method_names(&names, operations)
}

/// `BankAccount` and `Card` require deduplication of the update and delete methods, so
/// handle polymorphic cases like those here
fn deduplicate_method_names<'a>(
    base_names: &HashMap<RequestKey<'a>, String>,
    reqs: &'a [RequestDetails],
) -> HashMap<RequestKey<'a>, String> {
    let mut dedupped = HashMap::new();
    for req in reqs {
        let req_key = RequestKey::from_op(req);
        let curr_name = &base_names[&req_key];
        let has_dup_name =
            base_names.iter().any(|(&key, name)| key != req_key && name == curr_name);
        if has_dup_name {
            let path_param =
                req.path_params.first().expect("Expected path parameter on duplicate method");
            dedupped.insert(req_key, format!("{curr_name}_{}", path_param.name));
        } else {
            dedupped.insert(req_key, curr_name.to_string());
        }
    }
    dedupped
}

fn get_req_details<'a>(
    op: &'a StripeOperation,
    spec: &'a Spec,
) -> anyhow::Result<RequestDetails<'a>> {
    let operation = spec
        .get_request_operation(&op.path, op.operation_type)
        .context("Request path not found")?;
    let mut path_params = vec![];
    let mut query_params = vec![];
    for param in &operation.parameters {
        let param = param.as_item().context("Unexpected reference in parameter")?;
        match param {
            Parameter::Query { parameter_data, .. } => {
                query_params.push(parameter_data);
            }
            Parameter::Path { parameter_data, .. } => {
                if !parameter_data.required {
                    bail!("Expected path parameter to be required");
                }
                path_params.push(parameter_data);
            }
            _ => bail!("Unexpected parameter type {param:?}"),
        }
    }
    let form_params = get_request_form_parameters(operation);
    let params = match op.operation_type {
        OperationType::Get => {
            assert!(form_params.is_none(), "Body parameters not supported for GET");
            RequestParams::Query(if query_params.is_empty() { None } else { Some(query_params) })
        }
        OperationType::Post => {
            assert!(query_params.is_empty(), "Unexpected query parameters for POST");
            RequestParams::Form(form_params)
        }
        OperationType::Delete => {
            assert!(query_params.is_empty(), "Unexpected query parameters for DELETE");
            RequestParams::Form(form_params)
        }
    };

    let return_schema = get_ok_response_schema(operation).context("Expected schema")?;
    Ok(RequestDetails {
        returned: return_schema,
        path_params,
        params,
        description: operation.description.as_deref(),
        operation_type: op.operation_type,
        path: &op.path,
        base_method_name: &op.method_name,
    })
}

pub fn parse_requests(
    operations: Vec<StripeOperation>,
    spec: &Spec,
    ident: &RustIdent,
    component: &ComponentPath,
    path_id_map: &HashMap<String, ComponentPath>,
) -> anyhow::Result<Vec<RequestSpec>> {
    let mut req_details = Vec::with_capacity(operations.len());
    for op in &operations {
        if should_skip_request(op) {
            debug!("Skipping request at path {} with name {}", op.path, op.method_name);
            continue;
        }
        req_details.push(get_req_details(op, spec)?);
    }

    let method_names = infer_method_names(&req_details);

    let mut requests = Vec::with_capacity(operations.len());
    for req in &req_details {
        requests.push(
            build_request(
                req,
                ident,
                &method_names[&RequestKey::from_op(req)],
                component,
                path_id_map,
            )
            .with_context(|| format!("Error generating request operation {req:?}"))?,
        );
    }
    Ok(requests)
}

#[derive(Clone, Debug)]
enum RequestParams<'a> {
    Form(Option<&'a ReferenceOr<Schema>>),
    Query(Option<Vec<&'a ParameterData>>),
}

#[derive(Clone, Debug)]
struct RequestDetails<'a> {
    returned: &'a ReferenceOr<Schema>,
    path_params: Vec<&'a ParameterData>,
    params: RequestParams<'a>,
    description: Option<&'a str>,
    operation_type: OperationType,
    path: &'a str,
    base_method_name: &'a str,
}

fn build_request(
    req: &RequestDetails,
    parent_ident: &RustIdent,
    method_name: &str,
    component: &ComponentPath,
    path_id_map: &HashMap<String, ComponentPath>,
) -> anyhow::Result<RequestSpec> {
    let return_ident = RustIdent::joined(method_name, "returned");
    let return_type = Inference::new(&return_ident, ObjectGenInfo::new_deser())
        .required(true)
        .infer_schema_or_ref_type(req.returned);

    let params_ident = RustIdent::joined(method_name, parent_ident);
    let params_gen_info = ObjectGenInfo::new().include_constructor().serialize(true);
    let param_inference =
        Inference::new(&params_ident, params_gen_info).can_borrow(true).required(true);

    let param_typ = match &req.params {
        RequestParams::Form(schema) => schema.map(|s| param_inference.infer_schema_or_ref_type(s)),
        RequestParams::Query(params) => match params {
            None => None,
            Some(params) => {
                let mut struct_fields = Vec::with_capacity(params.len());
                for param in params {
                    let ParameterSchemaOrContent::Schema(schema) = &param.format else {
                        bail!("Expected query parameter to follow schema format");
                    };

                    let struct_field = param_inference
                        .field_name(&param.name)
                        .maybe_description(param.description.as_deref())
                        .required(param.required)
                        .build_struct_field(&param.name, schema);
                    struct_fields.push(struct_field);
                }
                Some(RustType::Object(
                    RustObject::Struct(struct_fields),
                    ObjectMetadata::new(params_ident.clone(), params_gen_info),
                ))
            }
        },
    }
    .unwrap_or_else(|| {
        RustType::Object(
            RustObject::Struct(vec![]),
            ObjectMetadata::new(params_ident.clone(), params_gen_info),
        )
    });

    let req_path = req.path.trim_start_matches("/v1");

    let mut path_params = vec![];
    for param in &req.path_params {
        let ParameterSchemaOrContent::Schema(schema) = &param.format else {
            bail!("Expected path parameter to follow schema format");
        };
        let base_param_typ = Inference::new(&params_ident, ObjectGenInfo::new())
            .can_borrow(true)
            .required(param.required)
            .maybe_description(param.description.as_deref())
            .field_name(&param.name)
            .infer_schema_or_ref_type(schema);

        if base_param_typ != RustType::Simple(SimpleType::Str) {
            bail!("Expected path parameter to be a string");
        }

        let rust_type = if let Some(id_typ) =
            infer_id_path(&param.name, &req.path_params, req_path, component, path_id_map)?
        {
            RustType::object_id(id_typ, true)
        } else {
            // NB: Assuming this is safe since we check earlier that this matches the path type.
            RustType::Simple(SimpleType::Str)
        };

        path_params.push(PathParam { name: param.name.clone(), rust_type })
    }
    Ok(RequestSpec {
        path_params,
        params: param_typ,
        doc_comment: req.description.map(|d| d.to_string()),
        req_path: req_path.to_string(),
        returned: return_type,
        method_name: method_name.into(),
        method_type: req.operation_type,
    })
}

/// Try our best to determine a specialized id type for this parameter, e.g. `AccountId`.
fn infer_id_path(
    param_name: &str,
    params: &[&ParameterData],
    req_path: &str,
    component: &ComponentPath,
    path_id_map: &HashMap<String, ComponentPath>,
) -> anyhow::Result<Option<ComponentPath>> {
    let has_single_path_param = params.len() == 1;

    // First, check if there is a hardcoded object name we should use. These are overrides might not be necessary
    // with cleverer inference, but this seems useful regardless since id handling is quite finicky, so having
    // an easy mechanism to hardcode parameter ideas is helpful
    if has_single_path_param {
        if let Some(obj_name) = find_single_path_param_override(req_path) {
            let id_path = path_id_map.get(obj_name).with_context(|| {
                format!("Overriden request path {req_path} has object name with no id")
            })?;
            return Ok(Some(id_path.clone()));
        }
    }

    // Try to use the name of the path parameter to infer the id type.
    if let Some(id_path) = path_id_map.get(param_name) {
        return Ok(Some(id_path.clone()));
    }

    // If there's just a single path parameter with the name `id`, try assuming the id corresponds
    // to the parent component (assuming such an id actually exists). We check for `id` explicitly
    // because some requests like `GetApplePayDomainsDomain` has a path parameter which is a _domain_,
    // not an id
    if has_single_path_param && param_name == "id" && path_id_map.contains_key(component.as_ref()) {
        return Ok(Some(component.clone()));
    }
    Ok(None)
}
