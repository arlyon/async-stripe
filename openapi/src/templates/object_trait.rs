use std::fmt::Write;

use indexmap::IndexMap;
use indoc::writedoc;

use crate::components::Components;
use crate::printable::PrintableType;
use crate::rust_object::ObjectRef;
use crate::types::RustIdent;

pub fn write_object_trait(
    out: &mut String,
    ident: &RustIdent,
    id_type: &PrintableType,
    is_optional: bool,
) {
    let body =
        if is_optional { "self.id.as_ref().map(|i| i.as_str())" } else { "Some(self.id.as_str())" };
    let _ = writedoc!(
        out,
        r#"
            impl stripe_types::Object for {ident} {{
                type Id = {id_type};
                fn id(&self) -> Option<&str> {{
                    {body}
                }}
            }}
            "#
    );
}

pub fn write_object_trait_for_enum(
    components: &Components,
    out: &mut String,
    ident: &RustIdent,
    id_type: &PrintableType,
    variants: &IndexMap<&str, ObjectRef>,
) {
    let mut match_inner = String::with_capacity(32);
    for obj in variants.values() {
        let comp = components.get(&obj.path);
        let ident = comp.ident();
        let _ = writeln!(match_inner, "Self::{ident}(v) => Some(v.id.as_str()),");
    }
    let _ = writedoc!(
        out,
        r#"
            impl stripe_types::Object for {ident} {{
                type Id = {id_type};
                fn id(&self) -> Option<&str> {{
                    match self {{
                    {match_inner}
                    }}
                }}
            }}
            "#
    );
}
