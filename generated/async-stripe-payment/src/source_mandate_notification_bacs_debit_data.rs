#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceMandateNotificationBacsDebitData {
    /// Last 4 digits of the account number associated with the debit.
    pub last4: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceMandateNotificationBacsDebitData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceMandateNotificationBacsDebitData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceMandateNotificationBacsDebitDataBuilder {
    last4: Option<Option<String>>,
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
                builder: SourceMandateNotificationBacsDebitDataBuilder {
                    last4: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "last4" => Deserialize::begin(&mut self.builder.last4),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(last4),) = (self.builder.last4.take(),) else {
                return Ok(());
            };
            *self.out = Some(SourceMandateNotificationBacsDebitData { last4 });
            Ok(())
        }
    }
};
