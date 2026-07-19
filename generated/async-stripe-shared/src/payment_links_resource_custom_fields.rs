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
    pub type_: PaymentLinksResourceCustomFieldsType,
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
    type_: Option<PaymentLinksResourceCustomFieldsType>,
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
                builder: PaymentLinksResourceCustomFieldsBuilder {
                    dropdown: Deserialize::default(),
                    key: Deserialize::default(),
                    label: Deserialize::default(),
                    numeric: Deserialize::default(),
                    optional: Deserialize::default(),
                    text: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dropdown" => Deserialize::begin(&mut self.builder.dropdown),
                "key" => Deserialize::begin(&mut self.builder.key),
                "label" => Deserialize::begin(&mut self.builder.label),
                "numeric" => Deserialize::begin(&mut self.builder.numeric),
                "optional" => Deserialize::begin(&mut self.builder.optional),
                "text" => Deserialize::begin(&mut self.builder.text),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(dropdown),
                Some(key),
                Some(label),
                Some(numeric),
                Some(optional),
                Some(text),
                Some(type_),
            ) = (
                self.builder.dropdown.take(),
                self.builder.key.take(),
                self.builder.label.take(),
                self.builder.numeric.take(),
                self.builder.optional,
                self.builder.text.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceCustomFields {
                dropdown,
                key,
                label,
                numeric,
                optional,
                text,
                type_,
            });
            Ok(())
        }
    }
};
/// The type of the field.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinksResourceCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentLinksResourceCustomFieldsType {
    pub fn as_str(&self) -> &str {
        use PaymentLinksResourceCustomFieldsType::*;
        match self {
            Dropdown => "dropdown",
            Numeric => "numeric",
            Text => "text",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceCustomFieldsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceCustomFieldsType::*;
        match s {
            "dropdown" => Ok(Dropdown),
            "numeric" => Ok(Numeric),
            "text" => Ok(Text),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentLinksResourceCustomFieldsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentLinksResourceCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentLinksResourceCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentLinksResourceCustomFieldsType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentLinksResourceCustomFieldsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentLinksResourceCustomFieldsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentLinksResourceCustomFieldsType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinksResourceCustomFieldsType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceCustomFieldsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
