/// This is an object representing your Stripe balance. You can retrieve it to see
/// the balance currently on your Stripe account.
///
/// The top-level `available` and `pending` comprise your "payments balance."
///
/// Related guide: [Balances and settlement time](https://docs.stripe.com/payments/balances), [Understanding Connect account balances](https://docs.stripe.com/connect/account-balances).
///
/// For more details see <<https://stripe.com/docs/api/balance/balance_object>>.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Balance {
    /// Available funds that you can transfer or pay out automatically by Stripe or explicitly through the [Transfers API](https://api.stripe.com#transfers) or [Payouts API](https://api.stripe.com#payouts).
    /// You can find the available balance for each currency and payment type in the `source_types` property.
    pub available: Vec<stripe_core::BalanceAmount>,
    /// Funds held due to negative balances on connected accounts where [account.controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    /// You can find the connect reserve balance for each currency and payment type in the `source_types` property.
    pub connect_reserved: Option<Vec<stripe_core::BalanceAmount>>,
    /// Funds that you can pay out using Instant Payouts.
    pub instant_available: Option<Vec<stripe_core::BalanceAmountNet>>,
    pub issuing: Option<stripe_core::BalanceDetail>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Funds that aren't available in the balance yet.
    /// You can find the pending balance for each currency and each payment type in the `source_types` property.
    pub pending: Vec<stripe_core::BalanceAmount>,
    pub refund_and_dispute_prefunding: Option<stripe_core::BalanceDetailUngated>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Balance").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BalanceBuilder {
    available: Option<Vec<stripe_core::BalanceAmount>>,
    connect_reserved: Option<Option<Vec<stripe_core::BalanceAmount>>>,
    instant_available: Option<Option<Vec<stripe_core::BalanceAmountNet>>>,
    issuing: Option<Option<stripe_core::BalanceDetail>>,
    livemode: Option<bool>,
    pending: Option<Vec<stripe_core::BalanceAmount>>,
    refund_and_dispute_prefunding: Option<Option<stripe_core::BalanceDetailUngated>>,
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
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceBuilder {
                    available: Deserialize::default(),
                    connect_reserved: Deserialize::default(),
                    instant_available: Deserialize::default(),
                    issuing: Deserialize::default(),
                    livemode: Deserialize::default(),
                    pending: Deserialize::default(),
                    refund_and_dispute_prefunding: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.builder.available),
                "connect_reserved" => Deserialize::begin(&mut self.builder.connect_reserved),
                "instant_available" => Deserialize::begin(&mut self.builder.instant_available),
                "issuing" => Deserialize::begin(&mut self.builder.issuing),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "pending" => Deserialize::begin(&mut self.builder.pending),
                "refund_and_dispute_prefunding" => {
                    Deserialize::begin(&mut self.builder.refund_and_dispute_prefunding)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(available),
                Some(connect_reserved),
                Some(instant_available),
                Some(issuing),
                Some(livemode),
                Some(pending),
                Some(refund_and_dispute_prefunding),
            ) = (
                self.builder.available.take(),
                self.builder.connect_reserved.take(),
                self.builder.instant_available.take(),
                self.builder.issuing.take(),
                self.builder.livemode,
                self.builder.pending.take(),
                self.builder.refund_and_dispute_prefunding.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(Balance {
                available,
                connect_reserved,
                instant_available,
                issuing,
                livemode,
                pending,
                refund_and_dispute_prefunding,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Balance {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Balance", 8)?;
        s.serialize_field("available", &self.available)?;
        s.serialize_field("connect_reserved", &self.connect_reserved)?;
        s.serialize_field("instant_available", &self.instant_available)?;
        s.serialize_field("issuing", &self.issuing)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("pending", &self.pending)?;
        s.serialize_field("refund_and_dispute_prefunding", &self.refund_and_dispute_prefunding)?;

        s.serialize_field("object", "balance")?;
        s.end()
    }
}
