/// This is an object representing your Stripe balance. You can retrieve it to see
/// the balance currently on your Stripe account.
///
/// You can also retrieve the balance history, which contains a list of
/// [transactions](https://stripe.com/docs/reporting/balance-transaction-types) that contributed to the balance.
/// (charges, payouts, and so forth).
///
/// The available and pending amounts for each currency are broken down further by
/// payment source types.
///
/// Related guide: [Understanding Connect account balances](https://stripe.com/docs/connect/account-balances).
///
/// For more details see <<https://stripe.com/docs/api/balance/balance_object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Balance {
    /// Available funds that you can transfer or pay out automatically by Stripe or explicitly through the [Transfers API](https://stripe.com/docs/api#transfers) or [Payouts API](https://stripe.com/docs/api#payouts).
    /// You can find the available balance for each currency and payment type in the `source_types` property.
    pub available: Vec<stripe_core::BalanceAmount>,
    /// Funds held due to negative balances on connected accounts where [account.controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    /// You can find the connect reserve balance for each currency and payment type in the `source_types` property.
    pub connect_reserved: Option<Vec<stripe_core::BalanceAmount>>,
    /// Funds that you can pay out using Instant Payouts.
    pub instant_available: Option<Vec<stripe_core::BalanceAmountNet>>,
    pub issuing: Option<stripe_core::BalanceDetail>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Funds that aren't available in the balance yet.
    /// You can find the pending balance for each currency and each payment type in the `source_types` property.
    pub pending: Vec<stripe_core::BalanceAmount>,
}
#[doc(hidden)]
pub struct BalanceBuilder {
    available: Option<Vec<stripe_core::BalanceAmount>>,
    connect_reserved: Option<Option<Vec<stripe_core::BalanceAmount>>>,
    instant_available: Option<Option<Vec<stripe_core::BalanceAmountNet>>>,
    issuing: Option<Option<stripe_core::BalanceDetail>>,
    livemode: Option<bool>,
    pending: Option<Vec<stripe_core::BalanceAmount>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Balance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Balance>,
        builder: BalanceBuilder,
    }

    impl Visitor for Place<Balance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: BalanceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for BalanceBuilder {
        type Out = Balance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.available),
                "connect_reserved" => Deserialize::begin(&mut self.connect_reserved),
                "instant_available" => Deserialize::begin(&mut self.instant_available),
                "issuing" => Deserialize::begin(&mut self.issuing),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "pending" => Deserialize::begin(&mut self.pending),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                available: Deserialize::default(),
                connect_reserved: Deserialize::default(),
                instant_available: Deserialize::default(),
                issuing: Deserialize::default(),
                livemode: Deserialize::default(),
                pending: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(available),
                Some(connect_reserved),
                Some(instant_available),
                Some(issuing),
                Some(livemode),
                Some(pending),
            ) = (
                self.available.take(),
                self.connect_reserved.take(),
                self.instant_available.take(),
                self.issuing.take(),
                self.livemode,
                self.pending.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                available,
                connect_reserved,
                instant_available,
                issuing,
                livemode,
                pending,
            })
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

    impl ObjectDeser for Balance {
        type Builder = BalanceBuilder;
    }

    impl FromValueOpt for Balance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BalanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "available" => b.available = FromValueOpt::from_value(v),
                    "connect_reserved" => b.connect_reserved = FromValueOpt::from_value(v),
                    "instant_available" => b.instant_available = FromValueOpt::from_value(v),
                    "issuing" => b.issuing = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "pending" => b.pending = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Balance {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Balance", 7)?;
        s.serialize_field("available", &self.available)?;
        s.serialize_field("connect_reserved", &self.connect_reserved)?;
        s.serialize_field("instant_available", &self.instant_available)?;
        s.serialize_field("issuing", &self.issuing)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("pending", &self.pending)?;

        s.serialize_field("object", "balance")?;
        s.end()
    }
}
