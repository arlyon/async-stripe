use std::collections::BTreeMap;

use crate::schema::{Parameter, Schema};

/// The top-level stripe OpenAPI spec
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Spec {
    components: Components,
    /// https://spec.openapis.org/oas/v3.1.0#paths-object
    paths: BTreeMap<String, BTreeMap<String, Operation>>,
}

impl Spec {
    pub fn get_schema(&self, name: &str) -> Option<&Schema> {
        self.components.schemas.get(name)
    }

    pub fn get_schema_unchecked(&self, name: &str) -> &Schema {
        self.components.schemas.get(name).as_ref().unwrap()
    }

    pub fn component_schemas(&self) -> &BTreeMap<String, Schema> {
        &self.components.schemas
    }

    /// Return an iterator over the paths to each endpoint
    pub fn paths(&self) -> impl Iterator<Item = &String> {
        self.paths.keys()
    }

    pub fn get_request_unchecked(&self, path: &str) -> &BTreeMap<String, Operation> {
        self.paths.get(path).unwrap()
    }
}

/// Holds a set of reusable objects for different aspects of the OAS.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct Components {
    /// An object to hold reusable Schema Objects.
    schemas: BTreeMap<String, Schema>,
}

/// Describes a single API operation on a path.
/// https://spec.openapis.org/oas/v3.1.0#operation-object
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    /// A verbose explanation of the operation behavior.
    description: String,
    /// The request body applicable for this operation.
    request_body: RequestBody,
    /// The list of possible responses as they are returned from executing this operation.
    responses: BTreeMap<String, Response>,
    /// A list of parameters that are applicable for this operation.
    #[serde(default)]
    parameters: Vec<Parameter>,
}

impl Operation {
    fn get_resp(&self, name: &str) -> Option<&Schema> {
        self.responses.get(name).and_then(|c| c.content.get("application/json").map(|r| &r.schema))
    }

    pub fn get_200_resp(&self) -> Option<&Schema> {
        self.get_resp("200")
    }

    pub fn err_schema_expected(&self) -> bool {
        self.get_resp("default")
            .map(|err_schema| {
                err_schema.path_ref().is_none()
                    || err_schema.path_ref() == Some("#/components/schemas/error")
            })
            .unwrap_or_default()
    }

    pub fn get_id_param(&self) -> Option<&Parameter> {
        self.parameters.iter().find(|p| p.location() == "path")
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    pub fn get_request_body_schema(&self) -> &Schema {
        self.request_body.get_schema()
    }
}

/// Describes a single request body
/// https://spec.openapis.org/oas/v3.1.0#request-body-object
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RequestBody {
    /// A map containing descriptions of potential response payloads.
    content: BTreeMap<String, MediaTypeObject>,
}

impl RequestBody {
    pub fn get_schema(&self) -> &Schema {
        self.content.get("application/x-www-form-urlencoded").unwrap().schema()
    }
}

/// Each Media Type Object provides schema and examples for the media type identified by its key.
/// https://spec.openapis.org/oas/v3.1.0#media-type-object
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MediaTypeObject {
    /// The schema defining the content of the request, response, or parameter.
    schema: Schema,
}

impl MediaTypeObject {
    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

/// Describes a single response from an API Operation.
/// https://spec.openapis.org/oas/v3.1.0#response-object
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Response {
    /// A map containing descriptions of potential response payloads.
    content: BTreeMap<String, MediaTypeObject>,
}
