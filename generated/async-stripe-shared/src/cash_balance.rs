/// A customer's `Cash balance` represents real funds.
/// Customers can add funds to their cash balance by sending a bank transfer.
/// These funds can be used for payment and can eventually be paid out to your bank account.
///
/// For more details see <<https://stripe.com/docs/api/cash_balance/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CashBalance {
    /// A hash of all cash balances available to this customer.
    /// You cannot delete a customer with any cash balances, even if the balance is 0.
    /// Amounts are represented in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub available: Option<std::collections::HashMap<String, i64>>,
    /// The ID of the customer whose cash balance this object represents.
    pub customer: String,
    /// The ID of an Account representing a customer whose cash balance this object represents.
    pub customer_account: Option<String>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    pub settings: stripe_shared::CustomerBalanceCustomerBalanceSettings,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CashBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CashBalance").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CashBalanceBuilder {
    available: Option<Option<std::collections::HashMap<String, i64>>>,
    customer: Option<String>,
    customer_account: Option<Option<String>>,
    livemode: Option<bool>,
    settings: Option<stripe_shared::CustomerBalanceCustomerBalanceSettings>,
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

    impl Deserialize for CashBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CashBalance>,
        builder: CashBalanceBuilder,
    }

    impl Visitor for Place<CashBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CashBalanceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CashBalanceBuilder {
        type Out = CashBalance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.available),
                "customer" => Deserialize::begin(&mut self.customer),
                "customer_account" => Deserialize::begin(&mut self.customer_account),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "settings" => Deserialize::begin(&mut self.settings),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                available: Some(None),
                customer: None,
                customer_account: Some(None),
                livemode: None,
                settings: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    available: Some(available),
                    customer: Some(customer),
                    customer_account: Some(customer_account),
                    livemode: Some(livemode),
                    settings: Some(settings),
                } => Some(Self::Out { available, customer, customer_account, livemode, settings }),
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

    impl ObjectDeser for CashBalance {
        type Builder = CashBalanceBuilder;
    }

    impl FromValueOpt for CashBalance {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for CashBalance {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("CashBalance", 6)?;
        s.serialize_field("available", &self.available)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_account", &self.customer_account)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("settings", &self.settings)?;

        s.serialize_field("object", "cash_balance")?;
        s.end()
    }
}
