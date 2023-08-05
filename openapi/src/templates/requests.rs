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
    fn gen_path_arg(&self) -> String {
        let req_path = self.request_path;

        // Parameterized request path
        if !self.path_params.is_empty() {
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
        }
    }

    fn gen_method_path_params(&self) -> String {
        self.path_params
            .iter()
            .map(|p| format!("{}:{}", p.name, p.typ))
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn gen_code(&self, out: &mut String) {
        let return_type = &self.returned;
        let path_arg = self.gen_path_arg();
        let path_params = self.gen_method_path_params();

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
        pub fn send(&self, client: &stripe::Client, {path_params}) -> stripe::Response<{return_type}> {{
            {body}
        }}
        "#
        );
    }

    pub fn gen_paginate(&self, inner_pagination_typ: PrintableType, out: &mut String) {
        let path = self.gen_path_arg();
        let path_params = self.gen_method_path_params();
        let _ = writedoc!(
            out,
            r#"
        pub fn paginate(self, {path_params}) -> stripe::ListPaginator<{inner_pagination_typ}> {{
            stripe::ListPaginator::from_params({path}, self)
        }}
        "#
        );
    }
}
