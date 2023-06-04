use std::fmt::Write;

use indoc::writedoc;

use crate::object_context::PrintableType;
use crate::rust_object::FieldedEnumVariant;
use crate::types::RustIdent;

pub fn write_object_trait(out: &mut String, ident: &RustIdent, id_type: &PrintableType) {
    let _ = writedoc!(
        out,
        r#"
            impl crate::Object for {ident} {{
                type Id = {id_type};
                fn id(&self) -> Self::Id {{
                    self.id.clone()
                }}
            }}
            "#
    );
}

pub fn write_object_trait_for_enum(
    out: &mut String,
    ident: &RustIdent,
    id_type: &PrintableType,
    variants: &[FieldedEnumVariant],
) {
    let mut match_inner = String::with_capacity(32);
    for variant in variants {
        let _ = writeln!(match_inner, "Self::{}(v) => v.id.to_string(),", variant.variant);
    }
    let _ = writedoc!(
        out,
        r#"
            impl crate::Object for {ident} {{
                type Id = {id_type};
                fn id(&self) -> Self::Id {{
                    match self {{
                    {match_inner}
                    }}
                }}
            }}
            "#
    );
}
