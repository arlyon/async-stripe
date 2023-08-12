use std::path::Path;

use anyhow::{bail, Context};
use indexmap::IndexMap;

use crate::components::{Components, RequestSource};
use crate::crate_inference::Crate;
use crate::rust_object::{ObjectRef, RustObject};
use crate::stripe_object::OperationType;
use crate::templates::derives::Derives;
use crate::templates::ObjectWriter;
use crate::types::{ComponentPath, RustIdent};
use crate::utils::append_to_file;

/// Paths to components which will be options in the `EventObject` union.
const WEBHOOK_OBJ_PATHS: &[&str] = &[
    "account",
    "capability",
    "application",
    "application_fee",
    "fee_refund",
    "balance",
    "bank_account",
    "billing_portal.configuration",
    "card",
    "checkout.session",
    "coupon",
    "customer",
    "discount",
    "dispute",
    "file",
    "invoice",
    "invoiceitem",
    "issuing.authorization",
    "issuing.card",
    "issuing.cardholder",
    "issuing.dispute",
    "issuing.transaction",
    "mandate",
    "payment_intent",
    "payment_link",
    "payment_method",
    "payout",
    "person",
    "plan",
    "price",
    "product",
    "promotion_code",
    "quote",
    "refund",
    "review",
    "setup_intent",
    "subscription",
    "subscription_schedule",
    "tax_id",
    "tax_rate",
    "test_helpers.test_clock",
    "topup",
    "transfer",
];

pub fn write_generated_for_webhooks(components: &Components) -> anyhow::Result<()> {
    let base = Path::new("stripe_webhook");
    write_event_type(components, base)?;
    write_event_object(components, base)?;
    Ok(())
}

fn write_event_object(components: &Components, out_path: &Path) -> anyhow::Result<()> {
    let mut objects = IndexMap::new();
    for path in WEBHOOK_OBJ_PATHS {
        let comp_path = ComponentPath::new(path.to_string());
        let component = components.get(&comp_path);
        let belonging_crate = component.krate_unwrapped().for_types();
        let obj_name = component.data.object_name.as_deref().expect("Component has no object name");

        objects.insert(
            obj_name,
            ObjectRef {
                path: comp_path,
                feature_gate: if belonging_crate != Crate::Types {
                    Some(belonging_crate.name())
                } else {
                    None
                },
            },
        );
    }
    let mut out = String::new();
    let ident = RustIdent::unchanged("EventObject".into());
    let mut writer = ObjectWriter::new(components, &ident);
    writer.provide_unknown_variant(true).derives_mut().deserialize(true);
    writer.write_enum_of_objects(&mut out, components, &objects);
    append_to_file(out, out_path.join("mod.rs"))?;
    Ok(())
}

fn write_event_type(components: &Components, out_path: &Path) -> anyhow::Result<()> {
    let event_type_src = RequestSource {
        path: "/webhook_endpoints",
        op: OperationType::Post,
        field_name: "enabled_events",
    };
    let spec = components.get_request_spec(event_type_src).context("Webhook endpoint not found")?;
    let typ = spec
        .params
        .as_rust_object()
        .context("Not an object")?
        .get_struct_field(event_type_src.field_name)
        .context("Field not found")?
        .as_rust_object()
        .context("Not an object")?;
    let RustObject::FieldlessEnum(enum_) = typ else {
        bail!("Expected enum");
    };
    let event_enum_variants =
        enum_.iter().filter(|v| v.wire_name != "*").cloned().collect::<Vec<_>>();
    let mut out = String::new();
    let ident = RustIdent::unchanged("EventType".into());
    ObjectWriter::new(components, &ident)
        .derives(Derives::new_deser())
        .provide_unknown_variant(true)
        .write_fieldless_enum_variants(&mut out, &event_enum_variants);
    append_to_file(out, out_path.join("mod.rs"))
}
