#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodCardPresentNetworks {
    /// All available networks for the card.
    pub available: Vec<String>,
    /// The preferred network for the card.
    pub preferred: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodCardPresentNetworksBuilder {
    available: Option<Vec<String>>,
    preferred: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for PaymentMethodCardPresentNetworks {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCardPresentNetworks>,
        builder: PaymentMethodCardPresentNetworksBuilder,
    }

    impl Visitor for Place<PaymentMethodCardPresentNetworks> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodCardPresentNetworksBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodCardPresentNetworksBuilder {
        type Out = PaymentMethodCardPresentNetworks;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.available),
                "preferred" => Deserialize::begin(&mut self.preferred),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { available: Deserialize::default(), preferred: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(available), Some(preferred)) = (self.available.take(), self.preferred.take())
            else {
                return None;
            };
            Some(Self::Out { available, preferred })
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

    impl ObjectDeser for PaymentMethodCardPresentNetworks {
        type Builder = PaymentMethodCardPresentNetworksBuilder;
    }

    impl FromValueOpt for PaymentMethodCardPresentNetworks {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodCardPresentNetworksBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "available" => b.available = FromValueOpt::from_value(v),
                    "preferred" => b.preferred = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
