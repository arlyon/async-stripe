#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCustomFieldsLabel {
    /// Custom text for the label, displayed to the customer. Up to 50 characters.
    pub custom: Option<String>,
    /// The type of the label.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PaymentLinksResourceCustomFieldsLabelType,
}
#[doc(hidden)]
pub struct PaymentLinksResourceCustomFieldsLabelBuilder {
    custom: Option<Option<String>>,
    type_: Option<PaymentLinksResourceCustomFieldsLabelType>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceCustomFieldsLabel {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCustomFieldsLabel>,
        builder: PaymentLinksResourceCustomFieldsLabelBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCustomFieldsLabel> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCustomFieldsLabelBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCustomFieldsLabelBuilder {
        type Out = PaymentLinksResourceCustomFieldsLabel;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom" => Deserialize::begin(&mut self.custom),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { custom: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(custom), Some(type_)) = (self.custom.take(), self.type_) else {
                return None;
            };
            Some(Self::Out { custom, type_ })
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

    impl ObjectDeser for PaymentLinksResourceCustomFieldsLabel {
        type Builder = PaymentLinksResourceCustomFieldsLabelBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceCustomFieldsLabel {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceCustomFieldsLabelBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "custom" => b.custom = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the label.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourceCustomFieldsLabelType {
    Custom,
}
impl PaymentLinksResourceCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourceCustomFieldsLabelType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceCustomFieldsLabelType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceCustomFieldsLabelType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentLinksResourceCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourceCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentLinksResourceCustomFieldsLabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentLinksResourceCustomFieldsLabelType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentLinksResourceCustomFieldsLabelType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentLinksResourceCustomFieldsLabelType::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinksResourceCustomFieldsLabelType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceCustomFieldsLabelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentLinksResourceCustomFieldsLabelType")
        })
    }
}
