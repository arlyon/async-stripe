#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCustomFields {
    pub dropdown: Option<stripe_shared::PaymentLinksResourceCustomFieldsDropdown>,
    /// String of your choice that your integration can use to reconcile this field.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,
    pub label: stripe_shared::PaymentLinksResourceCustomFieldsLabel,
    pub numeric: Option<stripe_shared::PaymentLinksResourceCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    /// Defaults to `false`.
    pub optional: bool,
    pub text: Option<stripe_shared::PaymentLinksResourceCustomFieldsText>,
    /// The type of the field.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::CreatePaymentLinkCustomFieldsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceCustomFields").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceCustomFieldsBuilder {
    dropdown: Option<Option<stripe_shared::PaymentLinksResourceCustomFieldsDropdown>>,
    key: Option<String>,
    label: Option<stripe_shared::PaymentLinksResourceCustomFieldsLabel>,
    numeric: Option<Option<stripe_shared::PaymentLinksResourceCustomFieldsNumeric>>,
    optional: Option<bool>,
    text: Option<Option<stripe_shared::PaymentLinksResourceCustomFieldsText>>,
    type_: Option<stripe_shared::CreatePaymentLinkCustomFieldsType>,
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

    impl Deserialize for PaymentLinksResourceCustomFields {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCustomFields>,
        builder: PaymentLinksResourceCustomFieldsBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCustomFields> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCustomFieldsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCustomFieldsBuilder {
        type Out = PaymentLinksResourceCustomFields;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dropdown" => Deserialize::begin(&mut self.dropdown),
                "key" => Deserialize::begin(&mut self.key),
                "label" => Deserialize::begin(&mut self.label),
                "numeric" => Deserialize::begin(&mut self.numeric),
                "optional" => Deserialize::begin(&mut self.optional),
                "text" => Deserialize::begin(&mut self.text),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                dropdown: Some(None),
                key: None,
                label: None,
                numeric: Some(None),
                optional: None,
                text: Some(None),
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(dropdown),
                Some(key),
                Some(label),
                Some(numeric),
                Some(optional),
                Some(text),
                Some(type_),
            ) = (
                self.dropdown.take(),
                self.key.take(),
                self.label.take(),
                self.numeric.take(),
                self.optional,
                self.text.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { dropdown, key, label, numeric, optional, text, type_ })
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

    impl ObjectDeser for PaymentLinksResourceCustomFields {
        type Builder = PaymentLinksResourceCustomFieldsBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceCustomFields {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceCustomFieldsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "dropdown" => b.dropdown = FromValueOpt::from_value(v),
                    "key" => b.key = FromValueOpt::from_value(v),
                    "label" => b.label = FromValueOpt::from_value(v),
                    "numeric" => b.numeric = FromValueOpt::from_value(v),
                    "optional" => b.optional = FromValueOpt::from_value(v),
                    "text" => b.text = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the field.
pub type PaymentLinksResourceCustomFieldsType = stripe_shared::CreatePaymentLinkCustomFieldsType;
