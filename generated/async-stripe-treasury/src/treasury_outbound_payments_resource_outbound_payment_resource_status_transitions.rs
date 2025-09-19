#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
    /// Timestamp describing when an OutboundPayment changed status to `canceled`.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundPayment changed status to `failed`.
    pub failed_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundPayment changed status to `posted`.
    pub posted_at: Option<stripe_types::Timestamp>,
    /// Timestamp describing when an OutboundPayment changed status to `returned`.
    pub returned_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitionsBuilder {
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    failed_at: Option<Option<stripe_types::Timestamp>>,
    posted_at: Option<Option<stripe_types::Timestamp>>,
    returned_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions,
        >,
        builder: TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitionsBuilder,
    }

    impl Visitor for Place<TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitionsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitionsBuilder
    {
        type Out = TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "canceled_at" => Deserialize::begin(&mut self.canceled_at),
                "failed_at" => Deserialize::begin(&mut self.failed_at),
                "posted_at" => Deserialize::begin(&mut self.posted_at),
                "returned_at" => Deserialize::begin(&mut self.returned_at),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                canceled_at: Deserialize::default(),
                failed_at: Deserialize::default(),
                posted_at: Deserialize::default(),
                returned_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(canceled_at), Some(failed_at), Some(posted_at), Some(returned_at)) =
                (self.canceled_at, self.failed_at, self.posted_at, self.returned_at)
            else {
                return None;
            };
            Some(Self::Out { canceled_at, failed_at, posted_at, returned_at })
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

    impl ObjectDeser for TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
        type Builder =
            TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitionsBuilder;
    }

    impl FromValueOpt for TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "canceled_at" => b.canceled_at = FromValueOpt::from_value(v),
                    "failed_at" => b.failed_at = FromValueOpt::from_value(v),
                    "posted_at" => b.posted_at = FromValueOpt::from_value(v),
                    "returned_at" => b.returned_at = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
