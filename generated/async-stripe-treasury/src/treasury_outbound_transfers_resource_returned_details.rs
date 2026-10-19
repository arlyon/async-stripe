#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundTransfersResourceReturnedDetails {
    /// Reason for the return.
    pub code: stripe_shared::TreasuryOutboundPaymentsResourceReturnedStatusCode,
    /// The Transaction associated with this object.
    pub transaction: stripe_types::Expandable<stripe_treasury::TreasuryTransaction>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryOutboundTransfersResourceReturnedDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryOutboundTransfersResourceReturnedDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryOutboundTransfersResourceReturnedDetailsBuilder {
    code: Option<stripe_shared::TreasuryOutboundPaymentsResourceReturnedStatusCode>,
    transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
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

    impl Deserialize for TreasuryOutboundTransfersResourceReturnedDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundTransfersResourceReturnedDetails>,
        builder: TreasuryOutboundTransfersResourceReturnedDetailsBuilder,
    }

    impl Visitor for Place<TreasuryOutboundTransfersResourceReturnedDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryOutboundTransfersResourceReturnedDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryOutboundTransfersResourceReturnedDetailsBuilder {
        type Out = TreasuryOutboundTransfersResourceReturnedDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.code),
                "transaction" => Deserialize::begin(&mut self.transaction),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { code: None, transaction: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(code), Some(transaction)) = (self.code.take(), self.transaction.take())
            else {
                return None;
            };
            Some(Self::Out { code, transaction })
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

    impl ObjectDeser for TreasuryOutboundTransfersResourceReturnedDetails {
        type Builder = TreasuryOutboundTransfersResourceReturnedDetailsBuilder;
    }

    impl FromValueOpt for TreasuryOutboundTransfersResourceReturnedDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryOutboundTransfersResourceReturnedDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "code" => b.code = FromValueOpt::from_value(v),
                    "transaction" => b.transaction = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Reason for the return.
pub type TreasuryOutboundTransfersResourceReturnedDetailsCode =
    stripe_shared::TreasuryOutboundPaymentsResourceReturnedStatusCode;
