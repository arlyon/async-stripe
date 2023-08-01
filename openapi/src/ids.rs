use std::collections::HashMap;
use std::fmt::Write;

use indexmap::IndexSet;
use lazy_static::lazy_static;

use crate::spec_inference::infer_id_name;
use crate::types::ComponentPath;

pub fn id_prefixes() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("account", "acct"),
        ("application_fee", "fee"),
        ("sku", "sku"),
        ("webhook_endpoint", "we"),
        ("tax_id", "txi"),
        ("person", "person"),
        ("quote", "qt"),
        ("file", "file"),
        ("file_link", "link"),
        ("customer", "cus"),
        ("discount", "di"),
        ("card", "card"),
        ("topup", "tu"),
        ("transfer", "tr"),
        ("transfer_reversal", "trr"),
        ("usage_record", "mbur"),
        ("billing_portal.session", "bps"),
        ("billing_portal.configuration", "bpc"),
        ("payment_intent", "pi"),
        ("test_helpers.test_clock", "clock"),
        ("price", "price"),
        ("invoice", "in"),
        ("invoiceitem", "ii"),
        ("review", "prv"),
        ("application", "ca"),
        ("order", "or"),
        ("mandate", "mandate"),
        ("ephemeral_key", "ephkey"),
        ("shipping_rate", "shr"),
        ("payout", "po"),
        ("payment_link", "plink"),
        ("source", "src"),
        ("tax_rate", "txr"),
        ("setup_attempt", "setatt"),
        ("balance_transaction", "txn"),
        ("subscription", "sub"),
        ("credit_note", "cn"),
        ("credit_note_line_item", "cnli"),
        ("customer_balance_transaction", "cbtxn"),
        ("subscription_item", "si"),
        ("event", "evt"),
        ("setup_intent", "seti"),
        ("tax_code", "txcd"),
        ("checkout.session", "cs"),
        ("promotion_code", "promo"),
        ("platform_tax_fee", "ptf"),
        ("connect_collection_transfer", "connct"),
        ("scheduled_query_run", "sqr"),
        ("tax_deducted_at_source", "itds"),
        ("terminal.reader", "tmr"),
        ("terminal.location", "tml"),
        ("terminal.configuration", "tmc"),
        ("issuing.dispute", "idp"),
        ("issuing.cardholder", "ich"),
        ("issuing.card", "ic"),
        ("issuing.transaction", "ipi"),
        ("issuing.authorization", "iauth"),
        ("reserve_transaction", "rtx"),
        ("subscription_schedule", "sub_sched"),
    ])
}

pub fn complex_id_prefixes() -> HashMap<&'static str, &'static [&'static str]> {
    // NB: done differently with temporary here because of type inference. Having to specify
    // length is also annoying but `from` doesn't accept slices
    let prefixes: [(&'static str, &'static [&'static str]); 6] = [
        ("refund", &["re", "pyr"]),
        ("usage_record_summary", &["urs", "sis"]),
        ("payment_method", &["pm", "card", "src", "ba"]),
        ("dispute", &["dp", "du"]),
        ("bank_account", &["ba", "card"]),
        ("charge", &["ch", "py"]), // TODO: Understand (and then document) why "py_" is a valid charge id
    ];
    HashMap::from(prefixes)
}

lazy_static! {
    pub static ref ID_PREFIXES: HashMap<&'static str, &'static str> = id_prefixes();
    pub static ref COMPLEX_ID_PREFIXES: HashMap<&'static str, &'static [&'static str]> =
        complex_id_prefixes();
    pub static ref IDS_IN_STRIPE: IndexSet<ComponentPath> = ids_in_stripe();
}

pub fn write_object_id(out: &mut String, path: &ComponentPath) {
    let crate_name = "stripe_types";
    let ident = infer_id_name(path);
    if let Some(prefix) = ID_PREFIXES.get(path.as_ref()) {
        let _ = writeln!(out, r#"{crate_name}::def_id!({ident}, "{prefix}_");"#);
    } else if let Some(multi_prefix) = COMPLEX_ID_PREFIXES.get(path.as_ref()) {
        let prefix_arg =
            multi_prefix.iter().map(|p| format!(r#""{p}_""#)).collect::<Vec<_>>().join("|");
        let _ = writeln!(out, "{crate_name}::def_id!({ident}, {prefix_arg});");
    } else {
        let _ = writeln!(out, "{crate_name}::def_id!({ident});");
    }
}

fn ids_in_stripe() -> IndexSet<ComponentPath> {
    let mut res = IndexSet::new();
    for comp in ["account", "application"] {
        res.insert(ComponentPath::new(comp.to_string()));
    }
    res
}
