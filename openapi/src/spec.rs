use anyhow::Context;
use indexmap::IndexMap;
use openapiv3::{
    Components, ObjectType, OpenAPI, Operation, Parameter, ParameterData, ParameterSchemaOrContent,
    PathItem, QueryStyle, ReferenceOr, Response, Schema, SchemaKind, StatusCode, Type,
};

#[derive(Debug, Clone)]
pub struct Spec(OpenAPI);

impl Spec {
    pub fn new(spec: OpenAPI) -> Self {
        Self(spec)
    }

    pub fn version(&self) -> &str {
        self.0.info.version.as_str()
    }

    fn components(&self) -> &Components {
        self.0.components.as_ref().expect("Spec did not contain `components`!")
    }

    /// Return an iterator over the paths to each endpoint
    pub fn paths(&self) -> impl Iterator<Item = &String> {
        self.0.paths.paths.keys()
    }

    pub fn component_schemas(&self) -> &IndexMap<String, ReferenceOr<Schema>> {
        &self.components().schemas
    }

    pub fn get_schema_unwrapped(&self, name: &str) -> &ReferenceOr<Schema> {
        self.component_schemas()
            .get(name)
            .as_ref()
            .unwrap_or_else(|| panic!("Expected to find a schema with name = {}", name))
    }

    pub fn get_request_unwrapped(&self, path: &str) -> &ReferenceOr<PathItem> {
        self.0.paths.paths.get(path).expect("Path not found")
    }
}

pub fn as_object_type(schema: &Schema) -> Option<&ObjectType> {
    match &schema.schema_kind {
        SchemaKind::Type(Type::Object(obj)) => Some(obj),
        _ => None,
    }
}

pub fn as_object_properties(
    schema: &Schema,
) -> Option<&IndexMap<String, ReferenceOr<Box<Schema>>>> {
    Some(&as_object_type(schema)?.properties)
}

pub fn as_enum_strings(schema: &Schema) -> Option<Vec<String>> {
    match &schema.schema_kind {
        SchemaKind::Type(Type::String(typ)) => {
            let variants = typ.enumeration.clone().into_iter().flatten().collect::<Vec<_>>();
            if variants.is_empty() {
                None
            } else {
                Some(variants)
            }
        }
        _ => None,
    }
}

/// Untyped equivalent: `schema["enum"][0]`
pub fn as_first_enum_value(schema: &Schema) -> Option<String> {
    as_enum_strings(schema)?.first().cloned()
}

/// Untyped equivalent:`schema["properties"]["object"]["enum"][0].as_str()`
pub fn as_object_enum_name(schema: &Schema) -> Option<String> {
    as_first_enum_value(as_object_properties(schema)?.get("object")?.as_item()?)
}

/// Untyped equivalent:`obj["properties"]["data"]["items"]`
pub fn as_data_array_item(obj: &ObjectType) -> Option<&ReferenceOr<Box<Schema>>> {
    let schema = obj.properties.get("data")?.as_item()?;
    if let SchemaKind::Type(Type::Array(typ)) = &schema.schema_kind {
        typ.items.as_ref()
    } else {
        None
    }
}

pub fn get_ok_response(operation: &Operation) -> Option<&ReferenceOr<Response>> {
    operation.responses.responses.get(&StatusCode::Code(200))
}

pub fn get_ok_response_schema(operation: &Operation) -> Option<&ReferenceOr<Schema>> {
    let resp = get_ok_response(operation)?.as_item()?;
    resp.content.get("application/json")?.schema.as_ref()
}

/// Untyped equivalent: `schema["anyOf"][0]["title"]`
pub fn as_any_of_first_item_title(schema: &Schema) -> Option<&str> {
    let any_of = match &schema.schema_kind {
        SchemaKind::AnyOf { any_of } => any_of,
        _ => return None,
    };
    any_of.first()?.as_item()?.schema_data.title.as_deref()
}

pub fn err_schema_expected(operation: &Operation) -> bool {
    operation
        .responses
        .default
        .as_ref()
        .map(|err_schema| match err_schema {
            ReferenceOr::Reference { reference } => reference == "#/components/schemas/error",
            ReferenceOr::Item(_) => true,
        })
        .unwrap_or_default()
}

pub fn non_path_ref_params(operation: &Operation) -> Vec<Parameter> {
    operation.parameters.iter().flat_map(|p| p.as_item()).cloned().collect()
}

pub fn get_request_form_parameters(operation: &Operation) -> anyhow::Result<Vec<Parameter>> {
    let form_schema = operation
        .request_body
        .as_ref()
        .context("No request body")?
        .as_item()
        .context("Expected item")?
        .content
        .get("application/x-www-form-urlencoded")
        .context("No form content found")?
        .schema
        .as_ref()
        .context("No request schema")?
        .as_item()
        .context("Expected item")?;
    let obj_type = as_object_type(form_schema).context("Expected object type schema")?;
    let properties = &obj_type.properties;
    Ok(properties
        .iter()
        .map(|(key, value)| {
            let maybe_item = value.as_item();
            Parameter::Query {
                parameter_data: ParameterData {
                    name: key.clone(),
                    description: maybe_item.and_then(|i| i.schema_data.description.clone()),
                    required: obj_type.required.iter().any(|v| v.as_str() == key),
                    deprecated: None,
                    format: ParameterSchemaOrContent::Schema(value.clone().unbox()),
                    example: None,
                    examples: Default::default(),
                    explode: None,
                    extensions: Default::default(),
                },
                allow_reserved: false,
                style: QueryStyle::DeepObject,
                allow_empty_value: None,
            }
        })
        .collect())
}

pub fn find_param_by_name<'a>(operation: &'a Operation, name: &str) -> Option<&'a Parameter> {
    operation.parameters.iter().find_map(|p| {
        p.as_item().and_then(|p| if p.parameter_data_ref().name == name { Some(p) } else { None })
    })
}

/// Untyped equivalent:
/// `params["parameters"].as_array().and_then(|arr| arr.iter().find(|p| p["in"].as_str() == Some("path")))`
pub fn get_id_param(params: &[ReferenceOr<Parameter>]) -> Option<&Parameter> {
    params.iter().find_map(|p| match p {
        ReferenceOr::Item(param @ Parameter::Path { .. }) => Some(param),
        _ => None,
    })
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ExpansionResources {
    #[serde(rename = "oneOf")]
    pub one_of: Vec<PathRef>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PathRef {
    #[serde(rename = "$ref")]
    pub reference: String,
}

impl ExpansionResources {
    pub fn into_schema_kind(self) -> SchemaKind {
        SchemaKind::OneOf {
            one_of: self
                .one_of
                .into_iter()
                .filter(|r| !r.reference.starts_with("#/components/schemas/deleted_"))
                .map(|r| ReferenceOr::Reference { reference: r.reference })
                .collect(),
        }
    }
}
