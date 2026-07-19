#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceIndividualName {
    /// Indicates whether individual name collection is enabled for the payment link.
    pub enabled: bool,
    /// Whether the customer is required to complete the field before checking out. Defaults to `false`.
    pub optional: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceIndividualName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceIndividualName").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceIndividualNameBuilder {
    enabled: Option<bool>,
    optional: Option<bool>,
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

    impl Deserialize for PaymentLinksResourceIndividualName {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceIndividualName>,
        builder: PaymentLinksResourceIndividualNameBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceIndividualName> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceIndividualNameBuilder {
                    enabled: Deserialize::default(),
                    optional: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "optional" => Deserialize::begin(&mut self.builder.optional),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(enabled), Some(optional)) = (self.builder.enabled, self.builder.optional)
            else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceIndividualName { enabled, optional });
            Ok(())
        }
    }
};
