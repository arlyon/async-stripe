#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionLodgingData {
    /// The time of checking into the lodging.
    pub check_in_at: Option<i64>,
    /// The number of nights stayed at the lodging.
    pub nights: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingTransactionLodgingData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingTransactionLodgingData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingTransactionLodgingDataBuilder {
    check_in_at: Option<Option<i64>>,
    nights: Option<Option<i64>>,
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

    impl Deserialize for IssuingTransactionLodgingData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionLodgingData>,
        builder: IssuingTransactionLodgingDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionLodgingData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionLodgingDataBuilder {
                    check_in_at: Deserialize::default(),
                    nights: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "check_in_at" => Deserialize::begin(&mut self.builder.check_in_at),
                "nights" => Deserialize::begin(&mut self.builder.nights),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(check_in_at), Some(nights)) = (self.builder.check_in_at, self.builder.nights)
            else {
                return Ok(());
            };
            *self.out = Some(IssuingTransactionLodgingData { check_in_at, nights });
            Ok(())
        }
    }
};
