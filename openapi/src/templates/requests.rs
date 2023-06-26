use std::fmt::Write;

use indoc::writedoc;

use crate::object_context::PrintableType;
use crate::stripe_object::OperationType;
use crate::templates::utils::write_doc_comment;

#[derive(Debug, Clone)]
pub struct PrintablePathParam<'a> {
    pub typ: PrintableType,
    pub name: &'a str,
}

#[derive(Debug, Clone)]
pub struct PrintableRequestSpec<'a> {
    pub path_params: Vec<PrintablePathParam<'a>>,
    pub doc_comment: Option<&'a str>,
    pub method_type: OperationType,
    pub param_type: Option<PrintableType>,
    pub method_name: &'a str,
    pub request_path: &'a str,
    pub returned: PrintableType,
}

impl<'a> PrintableRequestSpec<'a> {
    pub fn gen_code(&self, out: &mut String) {
        let method_name = self.method_name;
        let return_type = &self.returned;
        let mut params = vec![("client", "&stripe::Client".to_string())];
        for param in &self.path_params {
            params.push((param.name, param.typ.to_string()));
        }

        if let Some(param_typ) = &self.param_type {
            params.push(("params", param_typ.to_string()));
        }
        let params_body =
            params.into_iter().map(|p| format!("{}:{}", p.0, p.1)).collect::<Vec<_>>().join(",");

        let req_path = self.request_path;

        // Parameterized request path
        let path_arg = if !self.path_params.is_empty() {
            let fmt_args = self
                .path_params
                .iter()
                .map(|p| format!("{0}={0}", p.name))
                .collect::<Vec<_>>()
                .join(",");
            format!(r#"&format!("{req_path}", {fmt_args})"#)
        } else {
            // Plain request path
            format!(r#""{req_path}""#)
        };
        let method_enum = match self.method_type {
            OperationType::Get => "Get",
            OperationType::Post => "Post",
            OperationType::Delete => "Delete",
        };
        let body = match self.method_type {
            OperationType::Get => match self.param_type.is_some() {
                true => {
                    format!("client.get_query({path_arg}, params)")
                }
                false => {
                    format!("client.get({path_arg})")
                }
            },
            OperationType::Post | OperationType::Delete => {
                match self.param_type.is_some() {
                    true => {
                        format!("client.send_form({path_arg}, params, http_types::Method::{method_enum})")
                    }
                    false => {
                        format!("client.send({path_arg}, http_types::Method::{method_enum})")
                    }
                }
            }
        };
        if let Some(doc) = self.doc_comment {
            let comment = write_doc_comment(doc, 2);
            let _ = write!(out, "{comment}");
        }
        let _ = writedoc!(
            out,
            r#"
        pub fn {method_name}({params_body}) -> stripe::Response<{return_type}> {{
            {body}
        }}
        "#
        );
    }
}
