use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::components::Components;
use crate::rust_object::{EnumOfObjects, StructField};
use crate::templates::ObjectWriter;
use crate::types::RustIdent;

pub fn gen_enum_of_objects_miniserde(ident: &RustIdent, objects: &EnumOfObjects) -> String {
    let (peek_helper, miniserde_imports) = match objects {
        EnumOfObjects::MaybeDeleted(_) => ("peek_deleted_flag", "make_place, Deserialize, Result"),
        EnumOfObjects::ObjectUnion(_) => {
            ("peek_object_tag", "make_place, Deserialize, Error, Result")
        }
    };
    let raw_func_inner = raw_inner(ident, objects);
    formatdoc! {
        r#"
        const _: () = {{
            use stripe_miniserde::de::Visitor;
            use stripe_miniserde::{{{miniserde_imports}}};
            use stripe_miniserde::json::{peek_helper};
            use super::*;

            make_place!(Place);

            impl Deserialize for {ident} {{
                const WANTS_RAW: bool = true;

                fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {{
                   Place::new(out)
                }}
            }}

            impl Visitor for Place<{ident}> {{
                fn wants_raw(&self) -> bool {{
                    true
                }}

                fn raw(&mut self, bytes: &str) -> Result<()> {{
                    {raw_func_inner}
                }}
            }}
        }};
        "#
    }
}

fn raw_inner(ident: &RustIdent, objects: &EnumOfObjects) -> String {
    match objects {
        EnumOfObjects::MaybeDeleted(items) => {
            let deleted_name = &items.deleted.ident;
            let base_name = &items.base.ident;
            formatdoc! {r#"
            self.out = Some(if peek_deleted_flag(bytes) {{
                {ident}::{deleted_name}(stripe_miniserde::json::from_str(bytes)?)
            }} else {{
                {ident}::{base_name}(stripe_miniserde::json::from_str(bytes)?)
            }});
            Ok(())
            "#}
        }
        EnumOfObjects::ObjectUnion(objects) => {
            let mut arms = String::new();
            for (obj_discr, obj) in objects {
                let name = &obj.ident;
                let _ = writeln!(
                    arms,
                    r#""{obj_discr}" => {ident}::{name}(stripe_miniserde::json::from_str(bytes)?),"#
                );
            }
            formatdoc! {r#"
            let tag = peek_object_tag(bytes).ok_or(Error)?;
            self.out = Some(match tag.as_str() {{
                {arms}
                _ => return Err(Error),
            }});
            Ok(())
            "#}
        }
    }
}

impl ObjectWriter<'_> {
    pub fn gen_miniserde_struct_deserialize(&self, out: &mut String, fields: &[StructField]) {
        let ident = self.ident;
        let builder_name = RustIdent::joined(ident, "Builder");
        let mut builder_inner = String::new();
        for field in fields {
            let f_name = &field.variable_name();
            let printable = self.components.construct_printable_type(&field.rust_type);
            let _ = writeln!(builder_inner, "{f_name}: Option<{printable}>,");
        }

        let inner = miniserde_struct_inner(ident, &builder_name, fields, self.components);
        let _ = writedoc! {
            out,
            r#"
        #[doc(hidden)]
        pub struct {builder_name} {{
            {builder_inner}
        }}

        #[allow(unused_variables, irrefutable_let_patterns, dead_code, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
        const _: () = {{
            use stripe_miniserde::de::{{Map, Visitor}};
            use stripe_miniserde::{{make_place, Deserialize, Result}};

            make_place!(Place);

            {inner}
        }};
        "#
        };
    }
}

fn miniserde_struct_inner(
    ident: &RustIdent,
    builder_name: &RustIdent,
    fields: &[StructField],
    components: &Components,
) -> String {
    let mut key_inner = String::new();
    let mut builder_new_inner = String::new();

    let mut take_out_left = String::new();
    let mut take_out_right = String::new();
    for field in fields {
        let f_name = &field.variable_name();
        let wire_name = field.wire_name();

        let _ = write!(
            key_inner,
            r#"
            "{wire_name}" => Deserialize::begin(&mut self.builder.{f_name}),"#
        );
        let is_copy = field.rust_type.is_copy(components);

        let _ = writeln!(take_out_left, "Some({f_name}),");
        // For types which implement `Copy`, we don't need to call `.take()`. Does not affect
        // behavior, but helps a bit according to `llvm-lines` and binary size, so may as well since
        // unnecessary `take()` is not optimized out
        let take = if is_copy { "" } else { ".take()" };
        let _ = writeln!(take_out_right, "self.builder.{f_name}{take},");

        // NB: using stripe_miniserde::Deserialize::default() instead of `None` is very important - this copies
        // the miniserde derives in defaulting `Option<Option<T>>` to `Ok(Some)` so that missing
        // values are allowed for option types
        let _ = writeln!(builder_new_inner, "{f_name}: Deserialize::default(),");
    }

    let comma_sep_fields =
        fields.iter().map(|f| f.variable_name().clone()).collect::<Vec<_>>().join(",");

    formatdoc! {r#"
    impl Deserialize for {ident} {{
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {{
           Place::new(out)
        }}
    }}

    struct Builder<'a> {{
        out: &'a mut Option<{ident}>,
        builder: {builder_name},
    }}

    impl Visitor for Place<{ident}> {{
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {{
            Ok(Box::new(Builder {{
                out: &mut self.out,
                builder: {builder_name} {{ {builder_new_inner} }},
            }}))
        }}
    }}

    impl Map for Builder<'_> {{
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {{
            Ok(match k {{{key_inner}
                _ => <dyn Visitor>::ignore(),
            }})
        }}

        fn finish(&mut self) -> Result<()> {{
            let ({take_out_left}) = ({take_out_right}) else {{
                return Ok(());
            }};
            *self.out = Some({ident} {{ {comma_sep_fields} }});
            Ok(())
        }}
    }}
    "#
    }
}
