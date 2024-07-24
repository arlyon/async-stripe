use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::components::Components;
use crate::printable::{Lifetime, PrintableType, PrintableWithLifetime};
use crate::rust_object::{ObjectUsage, Struct, StructField, Visibility};
use crate::rust_type::{Container, RustType};
use crate::stripe_object::{OperationType, RequestSpec};
use crate::templates::utils::{write_default_impl, write_doc_comment};
use crate::templates::ObjectWriter;

impl RequestSpec {
    fn gen_path_arg(&self) -> String {
        let req_path = &self.req_path;

        // Parameterized request path
        if !self.path_params.is_empty() {
            format!(r#"format!("{req_path}")"#)
            // Plain request path
        } else {
            format!(r#""{req_path}""#)
        }
    }

    pub fn gen(&self, components: &Components) -> String {
        let build_inner = self.write_build_inner();

        let lifetime_str = if self.has_reference(components) { "<'_>" } else { "" };
        let impl_for = &self.ident;
        let impl_for_str = format!("{impl_for}{lifetime_str}");

        let return_type = components.construct_printable_type(&self.returned);
        let mut paginate_str = String::new();
        if let Some(pagination_kind) = as_pagination_kind(&self.returned) {
            self.gen_paginate(
                components.construct_printable_type(&self.returned),
                pagination_kind,
                &mut paginate_str,
            )
        }

        let mut out = self.write_builder_struct(components);
        let _ = writedoc!(
            out,
            r#"
            impl {impl_for_str} {{
                /// Send the request and return the deserialized response.
                pub async fn send<C: StripeClient>(&self, client: &C) -> Result<<Self as StripeRequest>::Output, C::Err> {{
                    self.customize().send(client).await
                }}

                /// Send the request and return the deserialized response, blocking until completion.
                pub fn send_blocking<C: StripeBlockingClient>(&self, client: &C) -> Result<<Self as StripeRequest>::Output, C::Err> {{
                    self.customize().send_blocking(client)
                }}

                {paginate_str}
            }}

            impl StripeRequest for {impl_for_str} {{
                type Output = {return_type};

                {build_inner}
            }}
        "#
        );

        out
    }

    fn gen_path_param_assignments(&self) -> String {
        let mut out = String::new();
        for param in &self.path_params {
            let name = &param.name;
            let _ = writeln!(out, r#"let {name} = self.{name};"#);
        }
        out
    }

    fn write_build_inner(&self) -> String {
        let path_arg = self.gen_path_arg();

        let method_enum = match self.method_type {
            OperationType::Get => "Get",
            OperationType::Post => "Post",
            OperationType::Delete => "Delete",
        };
        let mut build_inner = self.gen_path_param_assignments();
        let _ = write!(build_inner, "RequestBuilder::new(StripeMethod::{method_enum}, {path_arg})");

        if self.params.is_some() {
            if matches!(self.method_type, OperationType::Get) {
                let _ = write!(build_inner, ".query(&self.inner)");
            } else {
                let _ = write!(build_inner, ".form(&self.inner)");
            }
        }

        formatdoc!(
            r#"
        fn build(&self) -> RequestBuilder {{
            {build_inner}
        }}
        "#
        )
    }

    fn gen_paginate(&self, pagination_typ: PrintableType, kind: PaginationKind, out: &mut String) {
        let path = self.gen_path_arg();

        let paginate_method_name = match kind {
            PaginationKind::List => "new_list",
            PaginationKind::Search => "new_search_list",
        };

        let build_inner = self.gen_path_param_assignments();

        let _ = writedoc!(
            out,
            r#"
        pub fn paginate(&self) -> stripe_client_core::ListPaginator<{pagination_typ}> {{
            {build_inner}
            stripe_client_core::ListPaginator::{paginate_method_name}({path}, self.inner)
        }}
        "#
        );
    }

    fn write_builder_struct(&self, components: &Components) -> String {
        let name = &self.ident;
        let mut out = String::new();
        let (lifetime, lifetime_str) = if self.has_reference(components) {
            (Some(Lifetime), Lifetime.as_param())
        } else {
            (None, "")
        };
        if let Some(doc) = &self.doc_comment {
            let _ = writeln!(out, "{}", write_doc_comment(doc, 1).trim_end());
        }

        let mut fields = vec![];

        if let Some(req_param) = &self.params {
            fields.push(StructField::new("inner", req_param.typ.clone()).vis(Visibility::Private));
        }

        for arg in &self.path_params {
            fields
                .push(StructField::new(&arg.name, arg.rust_type.clone()).vis(Visibility::Private));
        }

        ObjectWriter::new(components, &self.ident, ObjectUsage::request_param())
            .lifetime(lifetime)
            .write_struct_definition(&mut out, &Struct::new(fields));

        let mut required_args = self
            .path_params
            .iter()
            .map(|p| {
                format!(
                    "{}:{}",
                    p.name,
                    PrintableWithLifetime::new(
                        &components.construct_printable_type(&p.rust_type),
                        lifetime
                    )
                )
            })
            .collect::<Vec<_>>();

        let mut required_struct_fields = vec![];
        let mut optional_struct_fields = vec![];
        if let Some(fields) = self.param_struct_fields() {
            for field in fields {
                if field.required {
                    required_struct_fields.push((&field.field_name, &field.rust_type));
                } else {
                    optional_struct_fields.push((
                        &field.field_name,
                        &field.rust_type,
                        field.doc_comment.as_ref(),
                    ))
                }
            }
        }
        for (f_name, typ) in &required_struct_fields {
            let typ = components.construct_printable_type(typ);
            let printable = PrintableWithLifetime::new(&typ, lifetime);
            required_args.push(format!("{f_name}:{printable}"))
        }
        let required_arg_str = required_args.join(",");

        let mut cons_inner = String::new();
        for path in &self.path_params {
            let _ = write!(cons_inner, "{},", path.name);
        }

        if let Some(req_param) = &self.params {
            let builder_name = components.construct_printable_type(&req_param.typ);
            let mut builder_params = String::new();
            for (f_name, _) in &required_struct_fields {
                let _ = write!(builder_params, "{f_name},");
            }
            let _ = write!(cons_inner, "inner: {builder_name}::new({builder_params})");
        }

        let mut cons_body = formatdoc! {
            r"
                /// Construct a new `{name}`.
                pub fn new({required_arg_str}) -> Self {{
                    Self {{
                        {cons_inner}
                    }}
                }}
                "
        };

        for (f_name, typ, doc_comment) in &optional_struct_fields {
            let typ = match typ {
                RustType::Container(Container::Option(inner)) => inner,
                _ => panic!("expected `Option`, found {typ:?}"),
            };
            let typ = components.construct_printable_type(typ);
            let typ = PrintableWithLifetime::new(&typ, lifetime);

            if let Some(doc_comment) = doc_comment {
                let _ = writeln!(cons_body, "{}", write_doc_comment(doc_comment, 1).trim_end());
            }
            let _ = writedoc!(
                cons_body,
                r#"
            pub fn {f_name}(mut self, {f_name}: {typ}) -> Self {{
                self.inner.{f_name} = Some({f_name});
                self
            }}
            "#
            );
        }

        let _ = writedoc!(
            out,
            r"
            impl{lifetime_str} {name}{lifetime_str} {{
                {cons_body}
            }}
                "
        );

        if required_args.is_empty() {
            write_default_impl(name, lifetime_str, &mut out);
        }
        out
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
