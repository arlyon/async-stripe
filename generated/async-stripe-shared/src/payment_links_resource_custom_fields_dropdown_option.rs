#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCustomFieldsDropdownOption {
    /// The label for the option, displayed to the customer. Up to 100 characters.
    pub label: String,
    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceCustomFieldsDropdownOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceCustomFieldsDropdownOption").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceCustomFieldsDropdownOptionBuilder {
    label: Option<String>,
    value: Option<String>,
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

    impl Deserialize for PaymentLinksResourceCustomFieldsDropdownOption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCustomFieldsDropdownOption>,
        builder: PaymentLinksResourceCustomFieldsDropdownOptionBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCustomFieldsDropdownOption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCustomFieldsDropdownOptionBuilder {
                    label: Deserialize::default(),
                    value: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "label" => Deserialize::begin(&mut self.builder.label),
                "value" => Deserialize::begin(&mut self.builder.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(label), Some(value)) = (self.builder.label.take(), self.builder.value.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceCustomFieldsDropdownOption { label, value });
            Ok(())
        }
    }
};
