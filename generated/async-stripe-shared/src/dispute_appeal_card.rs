#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeAppealCard {
    /// The reason for filing the appeal.
    pub reason_for_filing: Option<String>,
    /// One or more document IDs returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value of `dispute_evidence` to support the appeal.
    pub supporting_files: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeAppealCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputeAppealCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DisputeAppealCardBuilder {
    reason_for_filing: Option<Option<String>>,
    supporting_files: Option<Option<Vec<String>>>,
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

    impl Deserialize for DisputeAppealCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputeAppealCard>,
        builder: DisputeAppealCardBuilder,
    }

    impl Visitor for Place<DisputeAppealCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputeAppealCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputeAppealCardBuilder {
        type Out = DisputeAppealCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reason_for_filing" => Deserialize::begin(&mut self.reason_for_filing),
                "supporting_files" => Deserialize::begin(&mut self.supporting_files),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { reason_for_filing: Some(None), supporting_files: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(reason_for_filing), Some(supporting_files)) =
                (self.reason_for_filing.take(), self.supporting_files.take())
            else {
                return None;
            };
            Some(Self::Out { reason_for_filing, supporting_files })
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

    impl ObjectDeser for DisputeAppealCard {
        type Builder = DisputeAppealCardBuilder;
    }

    impl FromValueOpt for DisputeAppealCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputeAppealCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "reason_for_filing" => b.reason_for_filing = FromValueOpt::from_value(v),
                    "supporting_files" => b.supporting_files = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
