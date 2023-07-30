use std::collections::HashMap;

use anyhow::{anyhow, Context};
use heck::SnakeCase;
use openapiv3::{Parameter, ParameterData, ParameterSchemaOrContent, ReferenceOr, Schema};
use tracing::warn;

use crate::rust_object::{ObjectMetadata, RustObject};
use crate::rust_type::{RustType, SimpleType};
use crate::spec::{get_ok_response_schema, get_request_form_parameters, Spec};
use crate::spec_inference::Inference;
use crate::stripe_object::{OperationType, PathParam, RequestSpec, StripeOperation};
use crate::types::{ComponentPath, RustIdent};

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
                    return Err(anyhow!("Expected path parameter to be required"));
                }
                path_params.push(parameter_data);
            }
            _ => return Err(anyhow!("Unexpected parameter type {param:?}")),
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
    path_id_map: &HashMap<String, ComponentPath>,
) -> anyhow::Result<Vec<RequestSpec>> {
    let mut req_details = Vec::with_capacity(operations.len());
    for op in &operations {
        if should_skip_request(op) {
            warn!("Skipping request at path {} with name {}", op.path, op.method_name);
            continue;
        }
        req_details.push(get_req_details(op, spec)?);
    }

    let method_names = infer_method_names(&req_details);

    let mut requests = Vec::with_capacity(operations.len());
    for req in &req_details {
        requests.push(
            build_request(req, ident, &method_names[&RequestKey::from_op(req)], path_id_map)
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
    path_id_map: &HashMap<String, ComponentPath>,
) -> anyhow::Result<RequestSpec> {
    let return_ident = RustIdent::joined(method_name, "returned");
    let return_type =
        Inference::new(&return_ident).required(true).infer_schema_or_ref_type(req.returned);

    let params_ident = RustIdent::joined(method_name, parent_ident);
    let param_inference = Inference::new(&params_ident).can_borrow(true).required(true);

    let param_typ = match &req.params {
        RequestParams::Form(schema) => schema.map(|s| param_inference.infer_schema_or_ref_type(s)),
        RequestParams::Query(params) => match params {
            None => None,
            Some(params) => {
                let mut struct_fields = Vec::with_capacity(params.len());
                for param in params {
                    let ParameterSchemaOrContent::Schema(schema) = &param.format else {
                        return Err(anyhow!("Expected query parameter to follow schema format"));
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
                    ObjectMetadata::new(params_ident.clone()),
                ))
            }
        },
    };
    let mut path_params = vec![];
    for param in &req.path_params {
        let ParameterSchemaOrContent::Schema(schema) = &param.format else {
            return Err(anyhow!("Expected path parameter to follow schema format"));
        };
        let mut rust_type = Inference::new(&params_ident)
            .can_borrow(true)
            .required(param.required)
            .maybe_description(param.description.as_deref())
            .field_name(&param.name)
            .infer_schema_or_ref_type(schema);

        if rust_type != RustType::Simple(SimpleType::Str) {
            return Err(anyhow!("Expected path parameter to be a string"));
        }
        if let Some(id_path) = path_id_map.get(param.name.as_str()) {
            rust_type = RustType::object_id(id_path.clone(), true);
        }

        path_params.push(PathParam { name: param.name.clone(), rust_type })
    }
    Ok(RequestSpec {
        path_params,
        params: param_typ,
        doc_comment: req.description.map(|d| d.to_string()),
        req_path: req.path.trim_start_matches("/v1").to_string(),
        returned: return_type,
        method_name: method_name.into(),
        method_type: req.operation_type,
    })
}
