use indexmap::IndexMap;
use openapiv3::{
    Components, ObjectType, OpenAPI, Operation, PathItem, ReferenceOr, Response, Schema,
    SchemaKind, StatusCode, Type,
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

    pub fn component_schemas(&self) -> &IndexMap<String, ReferenceOr<Schema>> {
        &self.components().schemas
    }

    pub fn get_request(&self, path: &str) -> Option<&ReferenceOr<PathItem>> {
        self.0.paths.paths.get(path)
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

pub fn is_enum_with_just_empty_string(item: &ReferenceOr<Schema>) -> bool {
    let ReferenceOr::Item(item) = item else {return false};
    let Some(enum_strings) = as_enum_strings(item) else {return false};
    enum_strings.len() == 1 && enum_strings.first().unwrap().is_empty()
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

pub fn get_request_form_parameters(operation: &Operation) -> Option<&ReferenceOr<Schema>> {
    let schema = operation
        .request_body
        .as_ref()?
        .as_item()?
        .content
        .get("application/x-www-form-urlencoded")?
        .schema
        .as_ref();

    // Treat empty object as `None`
    if let Some(obj_type) = schema.and_then(|s| s.as_item().and_then(as_object_type)) {
        if obj_type.properties.is_empty() {
            return None;
        }
    }
    schema
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
