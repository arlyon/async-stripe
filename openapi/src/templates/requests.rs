use std::collections::HashMap;
use std::fmt::Display;

use indoc::formatdoc;

use crate::types::MethodTypes;
use crate::util::write_doc_comment;

/// Specification for implementing different request methods
pub enum RequestBase {
    List { params_name: String },
    Retrieve { id_type: String, has_expand_param: bool },
    Create { params_name: String, return_type: String },
    Update { id_type: String, params_name: String, return_type: String },
    Delete { id_type: String },
}

impl RequestBase {
    pub fn to_method_type(&self) -> MethodTypes {
        match self {
            RequestBase::List { .. } => MethodTypes::List,
            RequestBase::Retrieve { .. } => MethodTypes::Retrieve,
            RequestBase::Create { .. } => MethodTypes::Create,
            RequestBase::Update { .. } => MethodTypes::Update,
            RequestBase::Delete { .. } => MethodTypes::Delete,
        }
    }
}

pub struct RequestData {
    pub request: RequestBase,
    pub doc_comment: String,
    pub request_segments: Vec<String>,
}

impl RequestData {
    pub fn new<T: Display>(request: RequestBase, doc: T, segments: Vec<String>) -> Self {
        Self { request, doc_comment: doc.to_string(), request_segments: segments }
    }
}

pub fn gen_request(data: &RequestData, parent_struct: &str) -> String {
    let query_path = data.request_segments.join("/");
    let segment = &data.request_segments[0];

    let doc_comment = write_doc_comment(&data.doc_comment, 1);
    let req_str = match &data.request {
        RequestBase::List { params_name } => {
            formatdoc!(
                r#"
            pub fn list(client: &Client, params: &{params_name}<'_>) -> Response<List<{parent_struct}>> {{
                client.get_query("/{query_path}", &params)
            }}"#
            )
        }
        RequestBase::Retrieve { id_type, has_expand_param } => {
            if *has_expand_param {
                formatdoc!(
                    r#"
                    pub fn retrieve(client: &Client, id: &{id_type}, expand: &[&str]) -> Response<{parent_struct}> {{
                        client.get_query(&format!("/{segment}/{{}}", id), &Expand {{ expand }})
                    }}"#
                )
            } else {
                formatdoc!(
                    r#"
                    pub fn retrieve(client: &Client, id: &{id_type}) -> Response<{parent_struct}> {{
                        client.get_query(&format!("/{segment}/{{}}", id))
                    }}"#
                )
            }
        }
        RequestBase::Create { params_name, return_type } => {
            formatdoc!(
                r#"
                pub fn create(client: &Client, params: {params_name}<'_>) -> Response<{return_type}> {{
                   client.post_form("/{query_path}", &params)
                }}"#
            )
        }
        RequestBase::Update { id_type, params_name, return_type } => {
            formatdoc!(
                r#"
                pub fn update(client: &Client, id: &{id_type}, params: {params_name}<'_>) -> Response<{return_type}> {{
                    client.post_form(&format!("/{segment}/{{}}", id), &params)
                }}"#
            )
        }
        RequestBase::Delete { id_type } => {
            formatdoc!(
                r#"
                pub fn delete(client: &Client, id: &{id_type}) -> Response<Deleted<{id_type}>> {{
                    client.delete(&format!("/{segment}/{{}}", id))
                }}"#
            )
        }
    };
    format!("{doc_comment}{req_str}")
}

pub struct RequestsTemplateData {
    pub methods: HashMap<MethodTypes, RequestData>,
    pub parent_struct: String,
}

impl RequestsTemplateData {
    pub fn add(&mut self, req: RequestData) {
        self.methods.insert(req.request.to_method_type(), req);
    }

    pub fn gen_requests(&self) -> String {
        let parent = &self.parent_struct;
        // TODO: can simplify once no longer enforcing ordering to minimize git diff
        format!(
            "impl {} {{\n{}\n}}\n",
            parent,
            [
                MethodTypes::List,
                MethodTypes::Create,
                MethodTypes::Retrieve,
                MethodTypes::Update,
                MethodTypes::Delete
            ]
            .iter()
            .flat_map(|method| self.methods.get(method))
            .map(|m| gen_request(m, parent))
            .collect::<Vec<_>>()
            .join("\n\n")
        )
    }

    pub fn maybe_gen_impl_paginable(&self) -> Option<String> {
        let parent = &self.parent_struct;
        if let Some(RequestBase::List { params_name }) =
            self.methods.get(&MethodTypes::List).map(|m| &m.request)
        {
            Some(formatdoc!(
                r"
                impl Paginable for {params_name}<'_> {{
                    type O = {parent};
                    fn set_last(&mut self, item: Self::O) {{
                        self.starting_after = Some(item.id());
                    }}
                }}"
            ))
        } else {
            None
        }
    }
}

/// Implement the `Object` trait for the given struct
pub fn gen_impl_object(
    struct_name: &str,
    object_literal: &str,
    id_assoc_type: Option<&str>,
) -> String {
    let assoc_type = id_assoc_type.unwrap_or("()");
    let id_body = if id_assoc_type.is_some() { "self.id.clone()" } else { "" };
    formatdoc!(
        r#"
            impl Object for {struct_name} {{
                type Id = {assoc_type};
                fn id(&self) -> Self::Id {{
                    {id_body}
                }}
                fn object(&self) -> &'static str {{
                    "{object_literal}"
                }}
            }}
            "#
    )
}
