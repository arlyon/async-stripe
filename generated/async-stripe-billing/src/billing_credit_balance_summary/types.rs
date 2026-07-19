/// Indicates the billing credit balance for billing credits granted to a customer.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditBalanceSummary {
    /// The billing credit balances.
    /// One entry per credit grant currency.
    /// If a customer only has credit grants in a single currency, then this will have a single balance entry.
    pub balances: Vec<stripe_billing::CreditBalance>,
    /// The customer the balance is for.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    /// The account the balance is for.
    pub customer_account: Option<String>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingCreditBalanceSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingCreditBalanceSummary").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingCreditBalanceSummaryBuilder {
    balances: Option<Vec<stripe_billing::CreditBalance>>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    customer_account: Option<Option<String>>,
    livemode: Option<bool>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for BillingCreditBalanceSummary {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingCreditBalanceSummary>,
        builder: BillingCreditBalanceSummaryBuilder,
    }

    impl Visitor for Place<BillingCreditBalanceSummary> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingCreditBalanceSummaryBuilder {
                    balances: Deserialize::default(),
                    customer: Deserialize::default(),
                    customer_account: Deserialize::default(),
                    livemode: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "balances" => Deserialize::begin(&mut self.builder.balances),
                "customer" => Deserialize::begin(&mut self.builder.customer),
                "customer_account" => Deserialize::begin(&mut self.builder.customer_account),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(balances), Some(customer), Some(customer_account), Some(livemode)) = (
                self.builder.balances.take(),
                self.builder.customer.take(),
                self.builder.customer_account.take(),
                self.builder.livemode,
            ) else {
                return Ok(());
            };
            *self.out = Some(BillingCreditBalanceSummary {
                balances,
                customer,
                customer_account,
                livemode,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingCreditBalanceSummary {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingCreditBalanceSummary", 5)?;
        s.serialize_field("balances", &self.balances)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_account", &self.customer_account)?;
        s.serialize_field("livemode", &self.livemode)?;

        s.serialize_field("object", "billing.credit_balance_summary")?;
        s.end()
    }
}
