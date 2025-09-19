use std::fmt::Write;
use std::path::Path;

use anyhow::{Context, bail, ensure};
use indoc::writedoc;
use openapiv3::Schema;

use crate::components::Components;
use crate::crates::Crate;
use crate::printable::PrintableType;
use crate::rust_object::{ObjectUsage, RustObject, as_enum_of_objects};
use crate::rust_type::RustType;
use crate::spec_inference::Inference;
use crate::templates::ObjectWriter;
use crate::templates::object_writer::write_derives_line;
use crate::templates::utils::write_doc_comment;
use crate::types::RustIdent;
use crate::utils::append_to_file;

pub fn write_generated_for_webhooks(components: &Components) -> anyhow::Result<()> {
    let base = Path::new("async-stripe-webhook");
    write_event_object(components, base)?;
    Ok(())
}

fn write_event_object(components: &Components, out_path: &Path) -> anyhow::Result<()> {
    let mut out = String::new();
    let mut enum_body = String::new();
    let mut match_inner = String::new();
    for webhook_obj in &components.webhook_objs {
        let ident = RustIdent::create(&webhook_obj.event_type);

        let (printable, feature_gate) =
            if let Some(enum_objs) = webhook_obj.typ.as_rust_object().and_then(|o| match o {
                RustObject::Enum(variants) => as_enum_of_objects(components, variants),
                _ => None,
            }) {
                ObjectWriter::new(components, &ident, ObjectUsage::type_def())
                    .write_enum_of_objects(&mut out, &enum_objs);
                (
                    PrintableType::QualifiedPath {
                        path: None,
                        ident: ident.clone(),
                        has_ref: false,
                        is_ref: false,
                    },
                    None,
                )
            } else {
                let path = webhook_obj.typ.as_component_path().context(
                    "expected webhook data type to be enum of objects or component path",
                )?;
                let belonging_crate = components.get(path).krate_unwrapped().for_types();
                (
                    components.construct_printable_type_from_path(path),
                    if belonging_crate != Crate::SHARED {
                        Some(belonging_crate.crate_name())
                    } else {
                        None
                    },
                )
            };

        let comment = write_doc_comment(&webhook_obj.doc, 1);
        let _ = write!(enum_body, "{comment}");
        if let Some(gate) = &feature_gate {
            let _ = writeln!(enum_body, r#"#[cfg(feature = "{gate}")]"#);
        }
        let _ = writeln!(enum_body, "{ident}({printable}),");

        if let Some(gate) = &feature_gate {
            let _ = writeln!(match_inner, r#"#[cfg(feature = "{gate}")]"#);
        }
        let evt_type = &webhook_obj.event_type;
        let _ = writeln!(
            match_inner,
            r#""{evt_type}" => Self::{ident}(FromValueOpt::from_value(data)?),"#
        );
    }
    let _ = writedoc! {enum_body, r#"
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(with = "stripe_types::with_serde_json")
    )]
    Unknown(miniserde::json::Value),
    "#};

    write_derives_line(&mut out, Default::default());
    let _ = writedoc! {out, r#"
    #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
    #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    #[non_exhaustive]
    /// The event data for a webhook event.
    pub enum EventObject {{
        {enum_body}
    }} 
    "#};

    let _ = writedoc! {out, r#"
    impl EventObject {{
        pub(crate) fn from_raw_data(typ: &str, data: miniserde::json::Value) -> Option<Self> {{
            use stripe_types::miniserde_helpers::FromValueOpt;
            Some(match typ {{
                {match_inner}
                _ => Self::Unknown(data),
            }})
        }}
    }}
    "#};

    append_to_file(out, out_path.join("mod.rs"))?;
    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WebhookObject {
    pub event_type: String,
    pub doc: String,
    pub typ: RustType,
}

impl WebhookObject {
    pub fn from_schema(schema: &Schema) -> anyhow::Result<Option<Self>> {
        let Some(event_info) = schema.schema_data.extensions.get("x-stripeEvent") else {
            return Ok(None);
        };
        let event_type =
            event_info.get("type").context("no type")?.as_str().context("expected string")?;
        let desc = schema.schema_data.description.as_ref().context("expected description")?;

        let ident = RustIdent::create(event_type);
        let infer_ctx = Inference::new(&ident);
        let typ = infer_ctx.required(true).infer_schema_type(schema);

        Ok(Some(Self {
            event_type: event_type.to_string(),
            doc: desc.to_string(),
            typ: extract_object_type(typ)?,
        }))
    }
}

/// Extract and validate the type of the object field
fn extract_object_type(typ: RustType) -> anyhow::Result<RustType> {
    let (obj, _) = typ.into_object().context("expected object")?;

    let RustObject::Struct(struct_) = obj else {
        bail!("expected struct");
    };
    ensure!(struct_.fields.len() == 1);

    let object_field =
        struct_.fields.into_iter().find(|f| f.field_name == "object").context("no object field")?;
    Ok(object_field.rust_type)
}
