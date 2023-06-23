use std::collections::HashSet;

use indexmap::{IndexMap, IndexSet};

use crate::components::{Components, Module, ModuleName};

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
}

impl Crate {
    pub fn all() -> &'static [Crate] {
        use Crate::*;
        &[Core, Types, Misc, Connect, Terminal, Checkout, Treasury, Product]
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
        match self {
            Crate::Core => "core",
            Crate::Types => "types",
            Crate::Misc => "misc",
            Crate::Connect => "connect",
            Crate::Terminal => "terminal",
            Crate::Checkout => "checkout",
            Crate::Treasury => "treasury",
            Crate::Product => "product",
        }
    }
}

impl Crate {
    pub fn guaranteed_members(self) -> &'static [&'static str] {
        match self {
            Crate::Types => &["shipping"],
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
                "api_errors",
                "error",
                "connect_collection_transfer",
                "discounts_resource_discount_amount",
                "item",
                "source_mandate_notification",
                "setup_attempt",
                "invoices_line_items_proration_details",
                "payout",
                "event",
                "refund",
                "token",
                "application_fee",
                "review",
                "test_helpers",
                "issuing",
                "tax_code",
                "shipping_rate",
                "tax_id",
                "tax_rate",
                "promotion_code",
                "discount",
                "coupon",
                "credit_note",
                "invoice",
                "invoice_item",
                "plan",
                "billing_portal",
                "subscription_item",
                "subscription",
                "subscription_schedule",
                "quote",
                "card",
                "payment_link",
                "payment_method",
                "source",
                "product",
                "price",
                "account",
                "topup",
                "transfer",
                "bank_account",
                "radar",
            ],
            Crate::Product => &["sku"],
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
            ],
            Crate::Treasury => &["treasury"],
            Crate::Connect => &["apps", "country_spec"],
            Crate::Terminal => &["terminal"],
            Crate::Checkout => &["checkout"],
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
    pub fn make_crate_assignments(&self) -> IndexMap<ModuleName, Crate> {
        let mut crate_assignments = IndexMap::new();
        for (mod_name, module) in &self.modules {
            match module {
                Module::Package { name, .. } => {
                    let Some(krate) = Crate::map_guaranteed(name) else {
                        panic!("Expected package {} to have an enforced crate mapping", name);
                    };
                    crate_assignments.insert(mod_name.clone(), krate);
                }
                Module::Component { path } => {
                    if let Some(krate) = Crate::map_guaranteed(path) {
                        crate_assignments.insert(mod_name.clone(), krate);
                    }
                }
            }
        }
        for (mod_name, module) in &self.modules {
            match module {
                Module::Package { .. } => {}
                Module::Component { path } => {
                    if crate_assignments.contains_key(mod_name) {
                        continue;
                    }
                    let this_obj = self.get(path);
                    for (assigned_name, assigned_crate) in &crate_assignments {
                        let Some(obj) = self.maybe_get(assigned_name.as_ref()) else {
                            continue;
                        };
                        if this_obj.is_nested_resource_of(obj) {
                            crate_assignments.insert(mod_name.clone(), *assigned_crate);
                            break;
                        }
                    }
                }
            }
        }
        for mod_name in self.modules.keys() {
            if crate_assignments.contains_key(mod_name) {
                continue;
            }
            if let Some(assignment) = self.maybe_infer_crate_by_deps(mod_name, &crate_assignments) {
                crate_assignments.insert(mod_name.clone(), assignment);
            }
        }

        let mut missing = HashSet::new();
        for (mod_name, module) in &self.modules {
            if !crate_assignments.contains_key(mod_name) {
                missing.insert(module);
            }
        }
        if !missing.is_empty() {
            panic!("Some modules could not have their crate inferred: {:#?}", missing)
        }

        crate_assignments
    }

    fn maybe_infer_crate_by_deps(
        &self,
        mod_name: &ModuleName,
        curr_assignments: &IndexMap<ModuleName, Crate>,
    ) -> Option<Crate> {
        let mod_graph = self.gen_module_dep_graph();
        let known_dependents = mod_graph
            .neighbors(mod_name)
            .map(|n| curr_assignments.get(n))
            .collect::<Option<IndexSet<_>>>()?;

        // If all deps are the same crate, infer that crate
        if let Some(first) = known_dependents.first() {
            if known_dependents.iter().all(|d| d == first) {
                Some(**first)
            } else {
                None
            }
        } else {
            // Otherwise, place in the `types` crate
            Some(Crate::Types)
        }
    }
}
