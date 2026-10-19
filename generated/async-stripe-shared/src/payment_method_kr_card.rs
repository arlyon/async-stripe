#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodKrCard {
    /// The local credit or debit card brand.
    pub brand: Option<stripe_shared::Brand>,
    /// The last four digits of the card. This may not be present for American Express cards.
    pub last4: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodKrCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodKrCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodKrCardBuilder {
    brand: Option<Option<stripe_shared::Brand>>,
    last4: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodKrCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodKrCard>,
        builder: PaymentMethodKrCardBuilder,
    }

    impl Visitor for Place<PaymentMethodKrCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodKrCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodKrCardBuilder {
        type Out = PaymentMethodKrCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brand" => Deserialize::begin(&mut self.brand),
                "last4" => Deserialize::begin(&mut self.last4),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { brand: Some(None), last4: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(brand), Some(last4)) = (self.brand.take(), self.last4.take()) else {
                return None;
            };
            Some(Self::Out { brand, last4 })
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

    impl ObjectDeser for PaymentMethodKrCard {
        type Builder = PaymentMethodKrCardBuilder;
    }

    impl FromValueOpt for PaymentMethodKrCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodKrCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "brand" => b.brand = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The local credit or debit card brand.
pub type PaymentMethodKrCardBrand = stripe_shared::Brand;
