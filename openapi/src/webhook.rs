use std::path::Path;

use indexmap::IndexMap;

use crate::components::Components;
use crate::crate_inference::Crate;
use crate::rust_object::ObjectRef;
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
                feature_gate: if belonging_crate != Crate::TYPES {
                    Some(belonging_crate.name())
                } else {
                    None
                },
            },
        );
    }
    let mut out = String::new();
    let ident = RustIdent::unchanged("EventObject");
    let mut writer = ObjectWriter::new(components, &ident);
    writer.provide_unknown_variant(true).derives_mut().deserialize(true);
    writer.write_enum_of_objects(&mut out, components, &objects);
    append_to_file(out, out_path.join("mod.rs"))?;
    Ok(())
}
