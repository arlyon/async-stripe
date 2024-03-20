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

        let lifetime_str =
            if self.params.has_reference(components) { Lifetime.as_param() } else { "" };
        let impl_for = components.construct_printable_type(&self.params);
        if let Some(pagination_kind) = as_pagination_kind(&self.returned) {
            self.gen_paginate(
                components.construct_printable_type(&self.returned),
                pagination_kind,
                &mut req_out,
                components,
            )
        }
        formatdoc!(
            r#"
            impl{lifetime_str} {impl_for}{lifetime_str} {{
                {req_out}
            }}
        "#
        )
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

    fn gen_paginate(
        &self,
        pagination_typ: PrintableType,
        kind: PaginationKind,
        out: &mut String,
        components: &Components,
    ) {
        let path = self.gen_path_arg();
        let path_params = self.gen_method_path_params(components);

        let paginate_method_name = match kind {
            PaginationKind::List => "from_list_params",
            PaginationKind::Search => "from_search_params",
        };
        let _ = writedoc!(
            out,
            r#"
        pub fn paginate(self, {path_params}) -> stripe::ListPaginator<{pagination_typ}> {{
            stripe::ListPaginator::{paginate_method_name}({path}, self)
        }}
        "#
        );
    }
}

#[derive(Copy, Clone)]
enum PaginationKind {
    /// List<T>
    List,
    /// Search<T>
    Search,
}

fn as_pagination_kind(typ: &RustType) -> Option<PaginationKind> {
    match typ {
        RustType::Container(Container::List(_)) => Some(PaginationKind::List),
        RustType::Container(Container::SearchList(_)) => Some(PaginationKind::Search),
        _ => None,
    }
}
