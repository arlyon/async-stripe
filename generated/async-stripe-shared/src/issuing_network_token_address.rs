#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingNetworkTokenAddress {
    /// The street address of the cardholder tokenizing the card.
    pub line1: String,
    /// The postal code of the cardholder tokenizing the card.
    pub postal_code: String,
}
#[doc(hidden)]
pub struct IssuingNetworkTokenAddressBuilder {
    line1: Option<String>,
    postal_code: Option<String>,
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

    impl Deserialize for IssuingNetworkTokenAddress {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingNetworkTokenAddress>,
        builder: IssuingNetworkTokenAddressBuilder,
    }

    impl Visitor for Place<IssuingNetworkTokenAddress> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingNetworkTokenAddressBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingNetworkTokenAddressBuilder {
        type Out = IssuingNetworkTokenAddress;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "line1" => Deserialize::begin(&mut self.line1),
                "postal_code" => Deserialize::begin(&mut self.postal_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { line1: Deserialize::default(), postal_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(line1), Some(postal_code)) = (self.line1.take(), self.postal_code.take())
            else {
                return None;
            };
            Some(Self::Out { line1, postal_code })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for IssuingNetworkTokenAddress {
        type Builder = IssuingNetworkTokenAddressBuilder;
    }

    impl FromValueOpt for IssuingNetworkTokenAddress {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingNetworkTokenAddressBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "line1" => b.line1 = FromValueOpt::from_value(v),
                    "postal_code" => b.postal_code = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
