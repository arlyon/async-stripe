#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceMandateNotificationSepaDebitData {
    /// SEPA creditor ID.
    pub creditor_identifier: Option<String>,
    /// Last 4 digits of the account number associated with the debit.
    pub last4: Option<String>,
    /// Mandate reference associated with the debit.
    pub mandate_reference: Option<String>,
}
#[doc(hidden)]
pub struct SourceMandateNotificationSepaDebitDataBuilder {
    creditor_identifier: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate_reference: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceMandateNotificationSepaDebitData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceMandateNotificationSepaDebitData>,
        builder: SourceMandateNotificationSepaDebitDataBuilder,
    }

    impl Visitor for Place<SourceMandateNotificationSepaDebitData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceMandateNotificationSepaDebitDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceMandateNotificationSepaDebitDataBuilder {
        type Out = SourceMandateNotificationSepaDebitData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "creditor_identifier" => Deserialize::begin(&mut self.creditor_identifier),
                "last4" => Deserialize::begin(&mut self.last4),
                "mandate_reference" => Deserialize::begin(&mut self.mandate_reference),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                creditor_identifier: Deserialize::default(),
                last4: Deserialize::default(),
                mandate_reference: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                creditor_identifier: self.creditor_identifier.take()?,
                last4: self.last4.take()?,
                mandate_reference: self.mandate_reference.take()?,
            })
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

    impl ObjectDeser for SourceMandateNotificationSepaDebitData {
        type Builder = SourceMandateNotificationSepaDebitDataBuilder;
    }

    impl FromValueOpt for SourceMandateNotificationSepaDebitData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceMandateNotificationSepaDebitDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "creditor_identifier" => {
                        b.creditor_identifier = Some(FromValueOpt::from_value(v)?)
                    }
                    "last4" => b.last4 = Some(FromValueOpt::from_value(v)?),
                    "mandate_reference" => b.mandate_reference = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
