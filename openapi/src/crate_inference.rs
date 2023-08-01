use indexmap::{IndexMap, IndexSet};
use petgraph::prelude::DiGraphMap;
use tracing::debug;

use crate::components::{Components, Module, ModuleName};

pub const PATHS_IN_TYPES: &[&str] = &[
    "account",
    "file",
    "person",
    "external_account",
    "file_link",
    "bank_account",
    "card",
    "customer",
    "cash_balance",
    "payment_source",
    "discount",
    "subscription",
    "tax_id",
    "tax_code",
    "application_fee",
    "fee_refund",
    "payout",
    "topup",
    "invoice",
    "payment_method",
    "charge",
    "subscription_item",
    "tax_rate",
    "payment_intent",
    "invoiceitem",
    "quote",
    "setup_attempt",
    "setup_intent",
    "mandate",
    "coupon",
    "promotion_code",
    "line_item",
    "subscription_schedule",
    "review",
    "price",
    "plan",
    "transfer",
    "refund",
    "balance_transaction",
    "dispute",
    "product",
    "transfer_reversal",
    "balance_transaction_source",
    "customer_balance_transaction",
    "credit_note",
    "credit_note_line_item",
    "source",
    "test_helpers",
    "radar",
    "issuing",
    "discounts_resource_discount_amount",
    "connect_collection_transfer",
    "item",
    "shipping_rate",
    "external_account_requirements",
];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Crate {
    Core,
    Types,
    Misc,
    Connect,
    Terminal,
    Checkout,
    Treasury,
    Product,
    Billing,
    Payment,
}

impl Crate {
    pub fn all() -> &'static [Crate] {
        use Crate::*;
        &[Core, Types, Misc, Connect, Terminal, Checkout, Treasury, Product, Billing, Payment]
    }

    pub fn generate_to(self) -> String {
        let out_path = self.generated_out_path();
        if self == Self::Types {
            out_path
        } else {
            format!("{out_path}/src")
        }
    }

    pub fn generated_out_path(self) -> String {
        if self == Crate::Types {
            "stripe_types".into()
        } else {
            format!("crates/{}", self.name())
        }
    }

    pub fn name(self) -> String {
        format!("stripe_{}", self.suffix())
    }

    pub fn suffix(self) -> &'static str {
        use Crate::*;
        match self {
            Core => "core",
            Types => "types",
            Misc => "misc",
            Connect => "connect",
            Terminal => "terminal",
            Checkout => "checkout",
            Treasury => "treasury",
            Product => "product",
            Billing => "billing",
            Payment => "payment",
        }
    }
}

impl Crate {
    pub fn guaranteed_members(self) -> &'static [&'static str] {
        match self {
            Crate::Types => &[
                "api_errors",
                "shipping",
                "invoices_line_items_proration_details",
                "invoices_resource_line_items_proration_details",
            ],
            Crate::Core => &[
                "balance",
                "balance_transaction",
                "charge",
                "file",
                "dispute",
                "customer",
                "mandate",
                "payment_intent",
                "setup_intent",
                "fee_refund",
                "balance_transaction_source",
                "connect_collection_transfer",
                "source_mandate_notification",
                "setup_attempt",
                "payout",
                "event",
                "refund",
                "token",
                "review",
                "test_helpers",
                "issuing",
                "radar",
            ],
            Crate::Product => &[
                "sku",
                "product",
                "coupon",
                "discount",
                "price",
                "promotion_code",
                "shipping_rate",
                "tax_code",
                "tax_rate",
            ],
            Crate::Misc => &[
                "reporting",
                "identity",
                "sigma",
                "financial_connections",
                "apple_pay_domain",
                "ephemeral_key",
                "exchange_rate",
                "webhook_endpoint",
                "order",
                "tax",
            ],
            Crate::Treasury => &["treasury"],
            Crate::Connect => {
                &["apps", "country_spec", "account", "topup", "application_fee", "transfer"]
            }
            Crate::Terminal => &["terminal"],
            Crate::Checkout => &["checkout"],
            Crate::Billing => &[
                "invoice",
                "invoice_item",
                "item",
                "subscription",
                "subscription_item",
                "subscription_schedule",
                "credit_note",
                "quote",
                "plan",
                "billing_portal",
                "tax_id",
                "customer_balance_transaction",
            ],
            Crate::Payment => &["card", "payment_method", "source", "bank_account", "payment_link"],
        }
    }

    pub fn map_guaranteed(comp: &str) -> Option<Crate> {
        for krate in Crate::all() {
            if krate.guaranteed_members().contains(&comp) {
                return Some(*krate);
            }
        }
        None
    }
}

impl Components {
    pub fn infer_all_crate_assignments(&mut self) {
        // If a component includes requests that have URLs building off another component,
        // place with that component. This automates determinations like `external_account`
        // ending up in the same crate as `account` since all its requests start with `/account`.
        let mut new_assignments: IndexMap<ModuleName, Crate> = IndexMap::new();
        for (mod_name, module) in &self.modules {
            match module {
                Module::Package { .. } => {}
                Module::Component { path, krate } => {
                    if krate.is_some() {
                        continue;
                    }
                    let this_obj = self.get(path);
                    for (other_mod_name, other_module) in &self.modules {
                        let Some(krate) = other_module.krate() else {
                            continue;
                        };
                        let Some(obj) = self.maybe_get(other_mod_name.as_ref()) else {
                            continue;
                        };
                        if this_obj.is_nested_resource_of(obj) {
                            new_assignments.insert(mod_name.clone(), krate);
                            break;
                        }
                    }
                }
            }
        }
        debug!("Inferred {new_assignments:#?} based on nested request paths");
        for (mod_name, krate) in new_assignments {
            self.get_module_mut(&mod_name).assign_crate(krate);
        }

        let mod_graph = self.gen_module_dep_graph();
        let mut new_assignments: IndexMap<ModuleName, Crate> = IndexMap::new();
        for (mod_name, module) in &self.modules {
            if module.krate().is_some() {
                continue;
            }
            if let Some(assignment) = self.maybe_infer_crate_by_deps(mod_name, &mod_graph) {
                new_assignments.insert(mod_name.clone(), assignment);
            }
        }
        debug!("Inferred {new_assignments:#?} based on module dependencies");
        for (mod_name, krate) in new_assignments {
            self.get_module_mut(&mod_name).assign_crate(krate);
        }
    }

    fn maybe_infer_crate_by_deps(
        &self,
        mod_name: &ModuleName,
        mod_graph: &DiGraphMap<&ModuleName, ()>,
    ) -> Option<Crate> {
        let known_dependents = mod_graph
            .neighbors(mod_name)
            .map(|n| self.get_module(n).krate())
            .collect::<Option<IndexSet<_>>>()?;

        // If all deps are the same crate, infer that crate
        if let Some(first) = known_dependents.first() {
            if known_dependents.iter().all(|d| d == first) {
                Some(*first)
            } else {
                None
            }
        } else {
            // Otherwise, place in the `types` crate
            Some(Crate::Types)
        }
    }
}
