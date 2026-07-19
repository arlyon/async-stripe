#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCustomFieldsDropdown {
    /// The value that pre-fills on the payment page.
    pub default_value: Option<String>,
    /// The options available for the customer to select. Up to 200 options allowed.
    pub options: Vec<stripe_shared::PaymentLinksResourceCustomFieldsDropdownOption>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceCustomFieldsDropdown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceCustomFieldsDropdown").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceCustomFieldsDropdownBuilder {
    default_value: Option<Option<String>>,
    options: Option<Vec<stripe_shared::PaymentLinksResourceCustomFieldsDropdownOption>>,
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

    impl Deserialize for PaymentLinksResourceCustomFieldsDropdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCustomFieldsDropdown>,
        builder: PaymentLinksResourceCustomFieldsDropdownBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCustomFieldsDropdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCustomFieldsDropdownBuilder {
                    default_value: Deserialize::default(),
                    options: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "default_value" => Deserialize::begin(&mut self.builder.default_value),
                "options" => Deserialize::begin(&mut self.builder.options),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(default_value), Some(options)) =
                (self.builder.default_value.take(), self.builder.options.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceCustomFieldsDropdown { default_value, options });
            Ok(())
        }
    }
};
