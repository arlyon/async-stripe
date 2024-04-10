use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::components::Components;
use crate::rust_object::{EnumOfObjects, StructField};
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

    impl<'a> Map for Builder<'a> {{
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {{
            self.builder.key(k)
        }}

        fn finish(&mut self) -> Result<()> {{
            *self.out = self.builder.take_out();
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
                    _ => return None,
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

impl<'a> ObjectWriter<'a> {
    pub fn gen_miniserde_struct_deserialize(&self, out: &mut String, fields: &[StructField]) {
        let ident = self.ident;
        let builder_name = RustIdent::joined(ident, "Builder");
        let mut builder_inner = String::new();
        for field in fields {
            let f_name = &field.field_name;
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

        #[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
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
    components: &Components,
) -> String {
    let mut finish_inner = String::new();
    let mut key_inner = String::new();
    let mut builder_new_inner = String::new();
    let mut from_obj_inner = String::new();
    for field in fields {
        let f_name = &field.field_name;

        let _ = writeln!(
            key_inner,
            r#""{}" => Deserialize::begin(&mut self.{f_name}),"#,
            field.wire_name()
        );
        let _ = writeln!(
            from_obj_inner,
            r#""{}" => b.{f_name} = Some(FromValueOpt::from_value(v)?),"#,
            field.wire_name()
        );
        let is_copy = field.rust_type.is_copy(components);

        // For types which implement `Copy`, we don't need to call `.take()`. Does not affect
        // behavior, but helps a bit according to `llvm-lines` and binary size, so may as well since
        // unnecessary `take()` is not optimized out
        let take = if is_copy { "" } else { ".take()" };
        let _ = writeln!(finish_inner, "{f_name}: self.{f_name}{take}?,");

        // NB: using miniserde::Deserialize::default() instead of `None` is very important - this copies
        // the miniserde derives in defaulting `Option<Option<T>>` to `Ok(Some)` so that missing
        // values are allowed for option types
        let _ = writeln!(builder_new_inner, "{f_name}: Deserialize::default(),");
    }

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
            Ok(match k {{
                {key_inner}
                _ => <dyn Visitor>::ignore(),
            }})
        }}

        fn deser_default() -> Self {{
            Self {{
                {builder_new_inner}
            }}
        }}

        fn take_out(&mut self) -> Option<Self::Out> {{
            Some(Self::Out {{ {finish_inner} }})
        }}
    }}

    impl<'a> Map for Builder<'a> {{
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {{
            self.builder.key(k)
        }}

        fn finish(&mut self) -> Result<()> {{
            *self.out = self.builder.take_out();
            Ok(())
        }}
    }}

    impl ObjectDeser for {ident} {{
        type Builder = {builder_name};
    }}

    impl FromValueOpt for {ident} {{
        fn from_value(v: Value) -> Option<Self> {{
            let Value::Object(obj) = v else {{
                return None;
            }};
            let mut b = {builder_name}::deser_default();
            for (k, v) in obj {{
                match k.as_str() {{
                    {from_obj_inner}
                    _ => {{}}
                }}
            }}
            b.take_out()
        }}
    }}
    "#
    }
}
