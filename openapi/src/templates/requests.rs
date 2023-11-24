use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::components::Components;
use crate::printable::{Lifetime, PrintableType};
use crate::rust_type::{Container, RustType};
use crate::stripe_object::{OperationType, RequestSpec};
use crate::templates::utils::write_doc_comment;

impl RequestSpec {
    fn gen_path_arg(&self) -> String {
        let req_path = &self.req_path;

        // Parameterized request path
        if !self.path_params.is_empty() {
            format!(r#"&format!("{req_path}")"#)
            // Plain request path
        } else {
            format!(r#""{req_path}""#)
        }
    }

    fn gen_method_path_params(&self, components: &Components) -> String {
        self.path_params
            .iter()
            .map(|p| format!("{}:{}", p.name, components.construct_printable_type(&p.rust_type)))
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn gen(&self, components: &Components) -> String {
        let mut req_out = String::with_capacity(128);
        self.write_req_body(&mut req_out, components);

        let lifetime_str = if self.params.has_reference(components) {
            Lifetime::new().as_param()
        } else {
            "".into()
        };
        let impl_for = components.construct_printable_type(&self.params);
        let maybe_inner_list_typ = as_list_inner_typ(&self.returned);
        if let Some(inner_list_typ) = maybe_inner_list_typ {
            self.gen_paginate(
                components.construct_printable_type(inner_list_typ),
                &mut req_out,
                components,
            );
        }
        let mut out = formatdoc!(
            r#"
            impl{lifetime_str} {impl_for}{lifetime_str} {{
                {req_out}
            }}
        "#
        );

        if maybe_inner_list_typ.is_some() {
            let _ = writeln!(
                out,
                r"impl{lifetime_str} stripe::PaginationParams for {impl_for}{lifetime_str} {{}}"
            );
        }
        out
    }

    fn write_req_body(&self, out: &mut String, components: &Components) {
        let return_type = components.construct_printable_type(&self.returned);
        let path_arg = self.gen_path_arg();
        let path_params = self.gen_method_path_params(components);

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
        if let Some(doc) = &self.doc_comment {
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

    pub fn gen_paginate(
        &self,
        inner_pagination_typ: PrintableType,
        out: &mut String,
        components: &Components,
    ) {
        let path = self.gen_path_arg();
        let path_params = self.gen_method_path_params(components);
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

fn as_list_inner_typ(typ: &RustType) -> Option<&RustType> {
    match typ {
        RustType::Container(Container::List(typ)) => Some(typ),
        _ => None,
    }
}
