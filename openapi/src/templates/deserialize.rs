use std::fmt::Write as _;

use indoc::formatdoc;

use crate::types::RustIdent;

pub struct NamedObjectVariant<'a> {
    pub object_name: &'a str,
    pub variant_name: &'a RustIdent,
}

pub fn write_deserialize_by_object_name(
    variants: Vec<NamedObjectVariant>,
    ident: &RustIdent,
) -> String {
    let mut match_inner = String::new();
    for NamedObjectVariant { object_name, variant_name } in variants {
        let _ =
            writeln!(match_inner, r#""{object_name}" => Self::{variant_name}(from_str(str)?),"#);
    }
    let _ = writeln!(
        match_inner,
        r#"_ => return Err(crate::StripeError::JSONDeserialize("Unexpected object name".into())),"#
    );
    formatdoc! {
        r#"
            #[cfg(feature = "min-ser")]
            impl crate::deser::StripeDeserialize for {ident} {{
                fn deserialize(str: &str) -> Result<Self, crate::StripeError> {{
                    use miniserde::json::from_str;
                    let obj_name: crate::deser::ObjectName = from_str(str)?;
                    Ok(match obj_name.object.as_str() {{
            {match_inner}
                    }})
                }}
            }}
            "#
    }
}

pub struct DeletedOrNot<'a> {
    pub(crate) deleted_variant: &'a RustIdent,
    pub(crate) variant: &'a RustIdent,
}

pub fn write_deserialize_by_deleted_or_not(del_or_not: DeletedOrNot, ident: &RustIdent) -> String {
    let DeletedOrNot { deleted_variant, variant } = del_or_not;
    formatdoc! {
        r#"
            #[cfg(feature = "min-ser")]
            impl crate::deser::StripeDeserialize for {ident} {{
                fn deserialize(str: &str) -> Result<Self, crate::StripeError> {{
                    use crate::deser::StripeDeserialize;
                    use miniserde::json::from_str;
                    let maybe_deleted: crate::deser::MaybeDeleted = from_str(str)?;
                    let deleted = maybe_deleted.deleted.unwrap_or(false);
                    if deleted {{
                        Ok(Self::{deleted_variant}(StripeDeserialize::deserialize(str)?))
                    }} else {{
                        Ok(Self::{variant}(StripeDeserialize::deserialize(str)?))
                    }}
                }}
            }}
            "#
    }
}

pub fn write_deserialize_for_struct(ident: &RustIdent) -> String {
    formatdoc! {
        r#"
        #[cfg(feature = "min-ser")]
        impl miniserde::Deserialize for {ident} {{
            fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {{
                todo!()
            }}
        }}
        "#
    }
}
