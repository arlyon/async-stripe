use std::collections::HashMap;

use anyhow::{Context, bail};
use heck::ToSnakeCase;
use openapiv3::{Parameter, ParameterData, ParameterSchemaOrContent, ReferenceOr, Schema};
use tracing::debug;

use crate::rust_object::{ObjectMetadata, RustObject, Struct, Visibility};
use crate::rust_type::{RustType, SimpleType};
use crate::spec::{Spec, get_ok_response_schema, get_request_form_parameters};
use crate::spec_inference::Inference;
use crate::stripe_object::{OperationType, PathParam, RequestParam, RequestSpec, StripeOperation};
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
        let method_name: String = match op_spec.base_method_name {
            "retrieve" => {
                if has_path_params {
                    "retrieve".into()
                } else {
                    "retrieve_for_my_account".into()
                }
            }
            name @ ("list" | "create") => {
                if let Some(param) = op_spec.path_params.first() {
                    format!("{name}_{}", param.name)
                } else {
                    name.into()
                }
            }
            // Explicit conversion to snake case handles a single method_name "upcomingLines" -
            // all other methods are already snake case
            other => other.to_snake_case(),
        };
        names.insert(RequestKey::from_op(op_spec), method_name);
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
    let req_ident = RustIdent::joined(method_name, parent_ident);
    let return_ident = RustIdent::joined(&req_ident, "returned");
    let return_type =
        Inference::new(&return_ident).required(true).infer_schema_or_ref_type(req.returned);

    let builder_ident = RustIdent::joined(&req_ident, "Builder");
    let param_inference = Inference::new(&req_ident).can_borrow(false).required(true);

    let param_typ = match &req.params {
        RequestParams::Form(schema) => {
            // We have a bit of a special case here - we're constructing a builder, not the object
            // itself. Modifying after the inference is complete is messy, but works for this one off
            // case.
            if let Some(mut inferred) = schema.map(|s| param_inference.infer_schema_or_ref_type(s))
            {
                // Make sure the struct is named "...Builder"
                let (obj, meta) = inferred
                    .as_object_mut()
                    .context("expected form or query parameter to be an object")?;
                meta.ident = builder_ident.clone();

                // Make sure the struct and its fields are private
                let RustObject::Struct(struct_) = obj else {
                    bail!("expected form or query parameter to be a struct");
                };
                struct_.vis = Visibility::Private;
                struct_.fields.iter_mut().for_each(|f| f.vis = Visibility::Private);
                Some(inferred)
            } else {
                None
            }
        }
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
                        .build_struct_field(&param.name, schema)
                        .vis(Visibility::Private);
                    struct_fields.push(struct_field);
                }
                Some(RustType::Object(
                    RustObject::Struct(Struct::new(struct_fields).vis(Visibility::Private)),
                    ObjectMetadata::new(builder_ident.clone()),
                ))
            }
        },
    };

    let req_path = req.path.trim_start_matches("/v1");

    let mut path_params = vec![];
    for param in &req.path_params {
        let ParameterSchemaOrContent::Schema(schema) = &param.format else {
            bail!("Expected path parameter to follow schema format");
        };
        let base_param_typ = Inference::new(&req_ident)
            .can_borrow(false)
            .required(param.required)
            .maybe_description(param.description.as_deref())
            .field_name(&param.name)
            .infer_schema_or_ref_type(schema);

        if base_param_typ != RustType::Simple(SimpleType::String) {
            bail!("Expected path parameter to be a string");
        }

        let rust_type = if let Some(id_typ) = infer_id_path(&param.name, req_path, path_id_map)? {
            RustType::object_id(id_typ, false)
        } else {
            // NB: Assuming this is safe since we check earlier that this matches the path type.
            RustType::Simple(SimpleType::String)
        };

        path_params.push(PathParam { name: param.name.clone(), rust_type })
    }
    Ok(RequestSpec {
        ident: req_ident,
        path_params,
        params: param_typ.map(|t| RequestParam { ident: builder_ident, typ: t }),
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
    req_path: &str,
    path_id_map: &HashMap<String, ComponentPath>,
) -> anyhow::Result<Option<ComponentPath>> {
    let pieces = req_path.trim_matches('/').split('/');

    // For now we only infer the first parametrized piece as an idea, so collect everything
    // that precedes it and return is this is not the first parameterized piece
    let mut preceding_sections = vec![];
    for piece in pieces {
        if piece.starts_with('{') {
            if piece.trim_start_matches('{').trim_end_matches('}') == param_name {
                break;
            } else {
                return Ok(None);
            }
        } else {
            preceding_sections.push(piece);
        }
    }

    // Based on the request path, infer what it belongs to. Note that parameter names
    // are used inconsistently so they are not used at all in inference

    // /accounts -> account
    let obj_name = if preceding_sections.len() == 1 {
        unpluralize(preceding_sections.first().unwrap())
        // /treasury/inbound_transfers/ -> treasury.inbound_transfer
    } else if preceding_sections.len() == 2 {
        let class = preceding_sections.first().unwrap();
        let item = preceding_sections.get(1).unwrap();
        format!("{class}.{}", unpluralize(item))
        // Some cases here, but mostly around test helpers, so not too urgent to handle
    } else {
        return Ok(None);
    };

    if let Some(comp) = path_id_map.get(&obj_name) {
        Ok(Some(comp.clone()))
    } else {
        debug!("No id type for {}", req_path);
        Ok(None)
    }
}

fn unpluralize(val: &str) -> String {
    // A simple transformation covers the two cases we see in stripe request paths:

    // treasury_entries -> treasury_entry
    if val.ends_with("ies") {
        val.replace("ies", "y")
    } else {
        // accounts -> account
        val.trim_end_matches('s').to_string()
    }
}
