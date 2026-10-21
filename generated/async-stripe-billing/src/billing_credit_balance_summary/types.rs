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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: BillingCreditBalanceSummaryBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingCreditBalanceSummaryBuilder {
        type Out = BillingCreditBalanceSummary;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "balances" => Deserialize::begin(&mut self.balances),
                "customer" => Deserialize::begin(&mut self.customer),
                "customer_account" => Deserialize::begin(&mut self.customer_account),
                "livemode" => Deserialize::begin(&mut self.livemode),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { balances: None, customer: None, customer_account: Some(None), livemode: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    balances: Some(balances),
                    customer: Some(customer),
                    customer_account: Some(customer_account),
                    livemode: Some(livemode),
                } => Some(Self::Out { balances, customer, customer_account, livemode }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for BillingCreditBalanceSummary {
        type Builder = BillingCreditBalanceSummaryBuilder;
    }

    impl FromValueOpt for BillingCreditBalanceSummary {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
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
