#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit {
    /// Prefix used to generate the Mandate reference.
    /// Must be at most 12 characters long.
    /// Must consist of only uppercase letters, numbers, spaces, or the following special characters: '/', '_', '-', '&', '.'.
    /// Cannot begin with 'STRIPE'.
    pub reference_prefix: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsSepaDebitBuilder {
    reference_prefix: Option<Option<String>>,
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

    impl Deserialize for SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit>,
        builder: SetupIntentPaymentMethodOptionsMandateOptionsSepaDebitBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    SetupIntentPaymentMethodOptionsMandateOptionsSepaDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsMandateOptionsSepaDebitBuilder {
        type Out = SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reference_prefix" => Deserialize::begin(&mut self.reference_prefix),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { reference_prefix: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { reference_prefix: Some(reference_prefix) } => {
                    Some(Self::Out { reference_prefix })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit {
        type Builder = SetupIntentPaymentMethodOptionsMandateOptionsSepaDebitBuilder;
    }

    impl FromValueOpt for SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
