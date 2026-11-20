#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasurySharedResourceFailureDetailsAch {
    /// NACHA ACH return code.
    pub return_code: Option<String>,
    /// NACHA ACH return reason.
    pub return_reason: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasurySharedResourceFailureDetailsAch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasurySharedResourceFailureDetailsAch").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasurySharedResourceFailureDetailsAchBuilder {
    return_code: Option<Option<String>>,
    return_reason: Option<Option<String>>,
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

    impl Deserialize for TreasurySharedResourceFailureDetailsAch {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasurySharedResourceFailureDetailsAch>,
        builder: TreasurySharedResourceFailureDetailsAchBuilder,
    }

    impl Visitor for Place<TreasurySharedResourceFailureDetailsAch> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasurySharedResourceFailureDetailsAchBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasurySharedResourceFailureDetailsAchBuilder {
        type Out = TreasurySharedResourceFailureDetailsAch;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "return_code" => Deserialize::begin(&mut self.return_code),
                "return_reason" => Deserialize::begin(&mut self.return_reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { return_code: Some(None), return_reason: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(return_code), Some(return_reason)) =
                (self.return_code.take(), self.return_reason.take())
            else {
                return None;
            };
            Some(Self::Out { return_code, return_reason })
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

    impl ObjectDeser for TreasurySharedResourceFailureDetailsAch {
        type Builder = TreasurySharedResourceFailureDetailsAchBuilder;
    }

    impl FromValueOpt for TreasurySharedResourceFailureDetailsAch {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasurySharedResourceFailureDetailsAchBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "return_code" => b.return_code = FromValueOpt::from_value(v),
                    "return_reason" => b.return_reason = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
