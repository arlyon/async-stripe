#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceMandateNotificationAcssDebitData {
    /// The statement descriptor associate with the debit.
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceMandateNotificationAcssDebitData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceMandateNotificationAcssDebitData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceMandateNotificationAcssDebitDataBuilder {
    statement_descriptor: Option<Option<String>>,
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

    impl Deserialize for SourceMandateNotificationAcssDebitData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceMandateNotificationAcssDebitData>,
        builder: SourceMandateNotificationAcssDebitDataBuilder,
    }

    impl Visitor for Place<SourceMandateNotificationAcssDebitData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceMandateNotificationAcssDebitDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceMandateNotificationAcssDebitDataBuilder {
        type Out = SourceMandateNotificationAcssDebitData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { statement_descriptor: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { statement_descriptor: Some(statement_descriptor) } => {
                    Some(Self::Out { statement_descriptor })
                }
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

    impl ObjectDeser for SourceMandateNotificationAcssDebitData {
        type Builder = SourceMandateNotificationAcssDebitDataBuilder;
    }

    impl FromValueOpt for SourceMandateNotificationAcssDebitData {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
