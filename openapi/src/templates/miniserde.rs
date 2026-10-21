use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::rust_object::{EnumOfObjects, StructField};
use crate::rust_type::{Container, PathToType, RustType};
use crate::templates::ObjectWriter;
use crate::types::RustIdent;

pub fn gen_enum_of_objects_miniserde(ident: &RustIdent, objects: &EnumOfObjects) -> String {
    let builder_name = RustIdent::joined(ident, "Builder");
    let inner_builder_type = match objects {
        EnumOfObjects::MaybeDeleted(_) => "MaybeDeletedBuilderInner",
        EnumOfObjects::ObjectUnion(_) => "ObjectBuilderInner",
    };
    let inner = build_inner(ident, &builder_name, objects);
    formatdoc! {
        r#"
        #[derive(Default)]
        pub struct {builder_name} {{
            inner: stripe_types::miniserde_helpers::{inner_builder_type},
        }}

        const _: () = {{
            use miniserde::de::{{Map, Visitor}};
            use miniserde::{{make_place, Deserialize, Result}};
            use stripe_types::MapBuilder;
            use miniserde::json::Value;
            use super::*;
            use stripe_types::miniserde_helpers::FromValueOpt;

            make_place!(Place);

            {inner}
        }};
        "#
    }
}

fn take_out_inner(ident: &RustIdent, objects: &EnumOfObjects) -> String {
    match objects {
        EnumOfObjects::MaybeDeleted(items) => {
            let deleted_name = &items.deleted.ident;
            let base_name = &items.base.ident;
            formatdoc! {r#"
            let (deleted, o) = self.inner.finish_inner()?;
            Some(if deleted {{
                {ident}::{deleted_name}(FromValueOpt::from_value(Value::Object(o))?)
            }} else {{
                {ident}::{base_name}(FromValueOpt::from_value(Value::Object(o))?)
            }})
            "#}
        }
        EnumOfObjects::ObjectUnion(_) => {
            formatdoc! {r#"
            let (k, o) = self.inner.finish_inner()?;
            {ident}::construct(&k, o)
            "#

            }
        }
    }
}

fn build_inner(ident: &RustIdent, builder_name: &RustIdent, objects: &EnumOfObjects) -> String {
    let take_out_func_inner = take_out_inner(ident, objects);
    let mut out = formatdoc! {r#"
    struct Builder<'a> {{
        out: &'a mut Option<{ident}>,
        builder: {builder_name},
    }}


    impl Deserialize for {ident} {{
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {{
           Place::new(out)
        }}
    }}

    impl Visitor for Place<{ident}> {{
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {{
            Ok(Box::new(Builder {{
                out: &mut self.out,
                builder: Default::default(),
            }}))
        }}
    }}

    impl Map for Builder<'_> {{
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {{
            self.builder.key(k)
        }}

        fn finish(&mut self) -> Result<()> {{
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }}
    }}

    impl MapBuilder for {builder_name} {{
        type Out = {ident};
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {{
            self.inner.key_inner(k)
        }}

        fn deser_default() -> Self {{
            Self::default()
        }}

        fn take_out(&mut self) -> Option<Self::Out> {{
            {take_out_func_inner}
        }}
    }}

    impl stripe_types::ObjectDeser for {ident} {{
        type Builder = {builder_name};
    }}
    "#
    };
    if let EnumOfObjects::ObjectUnion(objects) = objects {
        let mut cons_inner = String::new();
        for (obj_discr, obj) in objects {
            let name = &obj.ident;
            let _ = writeln!(
                cons_inner,
                r#""{obj_discr}" => Self::{name}(FromValueOpt::from_value(Value::Object(o))?),"#
            );
        }
        let _ = writedoc!(
            out,
            r#"
        impl {ident} {{
            fn construct(key: &str, o: miniserde::json::Object) -> Option<Self> {{
                Some(match key {{
                    {cons_inner}
                    _ => {{
                        tracing::warn!("Unknown object type '{{}}' for enum '{{}}'", key, "{ident}");
                        return None;
                    }}
                }})
            }}
        }}

        impl FromValueOpt for {ident} {{
           fn from_value(v: Value) -> Option<Self> {{
               let (typ, obj) = stripe_types::miniserde_helpers::extract_object_discr(v)?;
               Self::construct(&typ, obj)
           }}
        }}
        "#
        );
    }
    out
}

impl ObjectWriter<'_> {
    pub fn gen_miniserde_struct_deserialize(&self, out: &mut String, fields: &[StructField]) {
        let ident = self.ident;
        let builder_name = RustIdent::joined(ident, "Builder");
        let mut builder_inner = String::new();
        for field in fields {
            let f_name = &field.variable_name();
            let printable = self.components.construct_printable_type(slot(&field.rust_type).typ);
            let _ = writeln!(builder_inner, "{f_name}: Option<{printable}>,");
        }

        let inner = miniserde_struct_inner(ident, &builder_name, fields);
        let _ = writedoc! {
            out,
            r#"
        #[doc(hidden)]
        pub struct {builder_name} {{
            {builder_inner}
        }}

        #[allow(unused_variables, irrefutable_let_patterns, clippy::let_unit_value, clippy::match_single_binding, clippy::single_match)]
        const _: () = {{
            use miniserde::de::{{Map, Visitor}};
            use miniserde::json::Value;
            use miniserde::{{make_place, Deserialize, Result}};
            use stripe_types::{{MapBuilder, ObjectDeser}};
            use stripe_types::miniserde_helpers::FromValueOpt;

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
) -> String {
    let mut key_inner = String::new();
    let mut builder_new_inner = String::new();

    let mut take_out_pattern = String::new();
    let mut has_required = false;
    for field in fields {
        let f_name = &field.variable_name();
        let wire_name = field.wire_name();
        let slot = slot(&field.rust_type);

        let begin = if slot.nullable {
            "stripe_types::miniserde_helpers::begin_nullable"
        } else {
            "Deserialize::begin"
        };
        let _ = write!(
            key_inner,
            r#"
            "{wire_name}" => {begin}(&mut self.{f_name}),"#
        );

        if slot.nullable {
            let _ = writeln!(take_out_pattern, "{f_name},");
        } else {
            has_required = true;
            let _ = writeln!(take_out_pattern, "{f_name}: Some({f_name}),");
        }

        // NB: matches miniserde::Deserialize::default(), but generates smaller artifacts. `None` vs `Some(None)`
        // is how missing required values are detected
        let builder_default =
            if field.rust_type.is_option() && !slot.nullable { "Some(None)" } else { "None" };
        let _ = writeln!(builder_new_inner, "{f_name}: {builder_default},");
    }

    let comma_sep_fields =
        fields.iter().map(|f| f.variable_name().clone()).collect::<Vec<_>>().join(",");

    // Move the whole builder in the fallback arm, so a missing field costs one drop of the builder
    // instead of one drop per field
    let take_out_body = if has_required {
        format!(
            "match core::mem::replace(self, Self::deser_default()) {{
                Self {{ {take_out_pattern} }} => Some(Self::Out {{ {comma_sep_fields} }}),
                builder => {{
                    *self = builder;
                    None
                }}
            }}"
        )
    } else {
        format!(
            "let Self {{ {take_out_pattern} }} = core::mem::replace(self, Self::deser_default());
            Some(Self::Out {{ {comma_sep_fields} }})"
        )
    };

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
                builder: {builder_name}::deser_default(),
            }}))
        }}
    }}

    impl MapBuilder for {builder_name} {{
        type Out = {ident};
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {{
            Ok(match k {{{key_inner}
                _ => <dyn Visitor>::ignore(),
            }})
        }}

        fn deser_default() -> Self {{
            Self {{ {builder_new_inner} }}
        }}

        fn take_out(&mut self) -> Option<Self::Out> {{
            {take_out_body}
        }}
    }}

    impl Map for Builder<'_> {{
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {{
            self.builder.key(k)
        }}

        fn finish(&mut self) -> Result<()> {{
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }}
    }}

    impl ObjectDeser for {ident} {{
        type Builder = {builder_name};
    }}

    impl FromValueOpt for {ident} {{
        fn from_value(v: Value) -> Option<Self> {{
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }}
    }}
    "#
    }
}

/// How a builder stores a field.
struct Slot<'a> {
    /// The builder stores `Option<typ>`.
    typ: &'a RustType,
    /// The field is optional, and the builder starts it with `begin_nullable`. That skips
    /// miniserde's visitor for `Option<T>`, which is compiled again for every `T`.
    nullable: bool,
}

fn slot(typ: &RustType) -> Slot<'_> {
    match typ {
        RustType::Container(Container::Option(inner)) if is_string_or_object(inner) => {
            Slot { typ: inner, nullable: true }
        }
        _ => Slot { typ, nullable: false },
    }
}

/// Whether the type is generated, so its visitor accepts only strings and objects.
fn is_string_or_object(typ: &RustType) -> bool {
    match typ {
        RustType::Object(..) => true,
        RustType::Path { path, is_ref: false } => matches!(
            path,
            PathToType::Component(_) | PathToType::Deduplicated { .. } | PathToType::Shared(_)
        ),
        RustType::Container(Container::Expandable(_)) => true,
        _ => false,
    }
}
