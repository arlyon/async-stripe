use std::path::Path;

use anyhow::{bail, Context};

use crate::components::{Components, RequestSource};
use crate::crate_inference::Crate;
use crate::rust_object::{EnumVariant, RustObject};
use crate::rust_type::RustType;
use crate::stripe_object::OperationType;
use crate::templates::derives::Derives;
use crate::templates::enums::{write_enum_variants, write_fieldless_enum_variants};
use crate::types::{ComponentPath, RustIdent};
use crate::utils::{append_to_file, as_object_names_for_deserialization};

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
    let mut variants = vec![];
    for path in WEBHOOK_OBJ_PATHS {
        let comp_path = ComponentPath::new(path.to_string());
        let component = components.get(&comp_path);
        let belonging_crate = component.krate_unwrapped().for_types();

        let typ = RustType::component_path(comp_path);
        let mut enum_variant = EnumVariant::new(component.ident().clone(), typ);
        if belonging_crate != Crate::Types {
            enum_variant = enum_variant.with_feature_gate(belonging_crate.name());
        }
        variants.push(enum_variant);
    }
    let mut out = String::new();
    let object_names = as_object_names_for_deserialization(components, &variants);
    let printable = variants.iter().map(|v| v.as_printable(components)).collect::<Vec<_>>();
    write_enum_variants(
        &mut out,
        &RustIdent::unchanged("EventObject".into()),
        &printable,
        Derives::new_deser(),
        None,
        object_names,
    );

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
    let mut event_enum_variants = match typ {
        RustObject::FieldlessEnum(enum_) => enum_.clone(),
        _ => bail!("Expected enum"),
    };
    event_enum_variants.retain(|v| v.wire_name != "*");
    let mut out = String::new();
    let ident = RustIdent::unchanged("EventType".into());
    write_fieldless_enum_variants(&mut out, &event_enum_variants, &ident, Derives::new_deser());
    append_to_file(out, out_path.join("mod.rs"))
}
