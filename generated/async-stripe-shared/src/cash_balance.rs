/// A customer's `Cash balance` represents real funds.
/// Customers can add funds to their cash balance by sending a bank transfer.
/// These funds can be used for payment and can eventually be paid out to your bank account.
///
/// For more details see <<https://stripe.com/docs/api/cash_balance/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CashBalance {
    /// A hash of all cash balances available to this customer.
    /// You cannot delete a customer with any cash balances, even if the balance is 0.
    /// Amounts are represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub available: Option<std::collections::HashMap<String, i64>>,
    /// The ID of the customer whose cash balance this object represents.
    pub customer: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub settings: stripe_shared::CustomerBalanceCustomerBalanceSettings,
}
#[doc(hidden)]
pub struct CashBalanceBuilder {
    available: Option<Option<std::collections::HashMap<String, i64>>>,
    customer: Option<String>,
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
                "livemode" => Deserialize::begin(&mut self.livemode),
                "settings" => Deserialize::begin(&mut self.settings),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                available: Deserialize::default(),
                customer: Deserialize::default(),
                livemode: Deserialize::default(),
                settings: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(available), Some(customer), Some(livemode), Some(settings)) =
                (self.available.take(), self.customer.take(), self.livemode, self.settings)
            else {
                return None;
            };
            Some(Self::Out { available, customer, livemode, settings })
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

    impl ObjectDeser for CashBalance {
        type Builder = CashBalanceBuilder;
    }

    impl FromValueOpt for CashBalance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CashBalanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "available" => b.available = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "settings" => b.settings = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for CashBalance {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("CashBalance", 5)?;
        s.serialize_field("available", &self.available)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("settings", &self.settings)?;

        s.serialize_field("object", "cash_balance")?;
        s.end()
    }
}
