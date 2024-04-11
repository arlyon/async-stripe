#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsUsBankAccountMandateOptions {
    /// Mandate collection method
    pub collection_method: Option<PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}
#[doc(hidden)]
pub struct PaymentMethodOptionsUsBankAccountMandateOptionsBuilder {
    collection_method:
        Option<Option<PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodOptionsUsBankAccountMandateOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsUsBankAccountMandateOptions>,
        builder: PaymentMethodOptionsUsBankAccountMandateOptionsBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsUsBankAccountMandateOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsUsBankAccountMandateOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsUsBankAccountMandateOptionsBuilder {
        type Out = PaymentMethodOptionsUsBankAccountMandateOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "collection_method" => Deserialize::begin(&mut self.collection_method),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { collection_method: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { collection_method: self.collection_method? })
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

    impl ObjectDeser for PaymentMethodOptionsUsBankAccountMandateOptions {
        type Builder = PaymentMethodOptionsUsBankAccountMandateOptionsBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsUsBankAccountMandateOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsUsBankAccountMandateOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "collection_method" => b.collection_method = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Mandate collection method
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    Paper,
}
impl PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::*;
        match self {
            Paper => "paper",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::*;
        match s {
            "paper" => Ok(Paper),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod",
            )
        })
    }
}
