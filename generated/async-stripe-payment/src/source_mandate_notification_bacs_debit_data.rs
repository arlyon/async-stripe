#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceMandateNotificationBacsDebitData {
    /// Last 4 digits of the account number associated with the debit.
    pub last4: Option<String>,
}
#[doc(hidden)]
pub struct SourceMandateNotificationBacsDebitDataBuilder {
    last4: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for SourceMandateNotificationBacsDebitData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceMandateNotificationBacsDebitData>,
        builder: SourceMandateNotificationBacsDebitDataBuilder,
    }

    impl Visitor for Place<SourceMandateNotificationBacsDebitData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceMandateNotificationBacsDebitDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceMandateNotificationBacsDebitDataBuilder {
        type Out = SourceMandateNotificationBacsDebitData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "last4" => Deserialize::begin(&mut self.last4),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { last4: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(last4),) = (self.last4.take(),) else {
                return None;
            };
            Some(Self::Out { last4 })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SourceMandateNotificationBacsDebitData {
        type Builder = SourceMandateNotificationBacsDebitDataBuilder;
    }

    impl FromValueOpt for SourceMandateNotificationBacsDebitData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceMandateNotificationBacsDebitDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "last4" => b.last4 = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
