#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomFieldsText {
    /// The value that pre-fills the field on the payment page.
    pub default_value: Option<String>,
    /// The maximum character length constraint for the customer's input.
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    pub minimum_length: Option<i64>,
    /// The value entered by the customer.
    pub value: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionCustomFieldsText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionCustomFieldsText").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCustomFieldsTextBuilder {
    default_value: Option<Option<String>>,
    maximum_length: Option<Option<i64>>,
    minimum_length: Option<Option<i64>>,
    value: Option<Option<String>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionCustomFieldsText {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomFieldsText>,
        builder: PaymentPagesCheckoutSessionCustomFieldsTextBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomFieldsText> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionCustomFieldsTextBuilder {
                    default_value: Deserialize::default(),
                    maximum_length: Deserialize::default(),
                    minimum_length: Deserialize::default(),
                    value: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "default_value" => Deserialize::begin(&mut self.builder.default_value),
                "maximum_length" => Deserialize::begin(&mut self.builder.maximum_length),
                "minimum_length" => Deserialize::begin(&mut self.builder.minimum_length),
                "value" => Deserialize::begin(&mut self.builder.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(default_value), Some(maximum_length), Some(minimum_length), Some(value)) = (
                self.builder.default_value.take(),
                self.builder.maximum_length,
                self.builder.minimum_length,
                self.builder.value.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionCustomFieldsText {
                default_value,
                maximum_length,
                minimum_length,
                value,
            });
            Ok(())
        }
    }
};
