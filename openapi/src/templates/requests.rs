use std::fmt::Write;

use indoc::writedoc;

use crate::printable::PrintableType;
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
    pub param_type: PrintableType,
    pub request_path: &'a str,
    pub returned: PrintableType,
}

impl<'a> PrintableRequestSpec<'a> {
    pub fn gen_code(&self, out: &mut String) {
        let return_type = &self.returned;
        let mut params = vec![("client", "&stripe::Client".to_string())];
        for param in &self.path_params {
            params.push((param.name, param.typ.to_string()));
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
            OperationType::Get => {
                format!("client.get_query({path_arg}, self)")
            }
            OperationType::Post | OperationType::Delete => {
                format!("client.send_form({path_arg}, self, http_types::Method::{method_enum})")
            }
        };
        if let Some(doc) = self.doc_comment {
            let comment = write_doc_comment(doc, 2);
            let _ = write!(out, "{comment}");
        }
        let _ = writedoc!(
            out,
            r#"
        pub fn send(&self, {params_body}) -> stripe::Response<{return_type}> {{
            {body}
        }}
        "#
        );
    }
}
