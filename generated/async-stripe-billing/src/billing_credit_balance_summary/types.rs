/// Indicates the billing credit balance for billing credits granted to a customer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditBalanceSummary {
    /// The billing credit balances.
    /// One entry per credit grant currency.
    /// If a customer only has credit grants in a single currency, then this will have a single balance entry.
    pub balances: Vec<stripe_billing::CreditBalance>,
    /// The customer the balance is for.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
#[doc(hidden)]
pub struct BillingCreditBalanceSummaryBuilder {
    balances: Option<Vec<stripe_billing::CreditBalance>>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
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
                "livemode" => Deserialize::begin(&mut self.livemode),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                balances: Deserialize::default(),
                customer: Deserialize::default(),
                livemode: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(balances), Some(customer), Some(livemode)) =
                (self.balances.take(), self.customer.take(), self.livemode)
            else {
                return None;
            };
            Some(Self::Out { balances, customer, livemode })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for BillingCreditBalanceSummary {
        type Builder = BillingCreditBalanceSummaryBuilder;
    }

    impl FromValueOpt for BillingCreditBalanceSummary {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingCreditBalanceSummaryBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "balances" => b.balances = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingCreditBalanceSummary {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingCreditBalanceSummary", 4)?;
        s.serialize_field("balances", &self.balances)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("livemode", &self.livemode)?;

        s.serialize_field("object", "billing.credit_balance_summary")?;
        s.end()
    }
}
