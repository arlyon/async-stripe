#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceMandateNotificationSepaDebitData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceMandateNotificationSepaDebitData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceMandateNotificationSepaDebitDataBuilder {
    creditor_identifier: Option<Option<String>>,
    last4: Option<Option<String>>,
    mandate_reference: Option<Option<String>>,
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
                builder: SourceMandateNotificationSepaDebitDataBuilder {
                    creditor_identifier: Deserialize::default(),
                    last4: Deserialize::default(),
                    mandate_reference: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "creditor_identifier" => Deserialize::begin(&mut self.builder.creditor_identifier),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "mandate_reference" => Deserialize::begin(&mut self.builder.mandate_reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(creditor_identifier), Some(last4), Some(mandate_reference)) = (
                self.builder.creditor_identifier.take(),
                self.builder.last4.take(),
                self.builder.mandate_reference.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SourceMandateNotificationSepaDebitData {
                creditor_identifier,
                last4,
                mandate_reference,
            });
            Ok(())
        }
    }
};
