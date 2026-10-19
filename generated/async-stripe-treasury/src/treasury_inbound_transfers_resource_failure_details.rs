#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryInboundTransfersResourceFailureDetails {
    /// Reason for the failure.
    pub code: stripe_shared::Code,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryInboundTransfersResourceFailureDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryInboundTransfersResourceFailureDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryInboundTransfersResourceFailureDetailsBuilder {
    code: Option<stripe_shared::Code>,
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

    impl Deserialize for TreasuryInboundTransfersResourceFailureDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryInboundTransfersResourceFailureDetails>,
        builder: TreasuryInboundTransfersResourceFailureDetailsBuilder,
    }

    impl Visitor for Place<TreasuryInboundTransfersResourceFailureDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryInboundTransfersResourceFailureDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryInboundTransfersResourceFailureDetailsBuilder {
        type Out = TreasuryInboundTransfersResourceFailureDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { code: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(code),) = (self.code.take(),) else {
                return None;
            };
            Some(Self::Out { code })
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

    impl ObjectDeser for TreasuryInboundTransfersResourceFailureDetails {
        type Builder = TreasuryInboundTransfersResourceFailureDetailsBuilder;
    }

    impl FromValueOpt for TreasuryInboundTransfersResourceFailureDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryInboundTransfersResourceFailureDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "code" => b.code = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Reason for the failure.
pub type TreasuryInboundTransfersResourceFailureDetailsCode = stripe_shared::Code;
