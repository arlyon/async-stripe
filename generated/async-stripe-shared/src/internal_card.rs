#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InternalCard {
    /// Brand of the card used in the transaction
    pub brand: Option<String>,
    /// Two-letter ISO code representing the country of the card
    pub country: Option<String>,
    /// Two digit number representing the card's expiration month
    pub exp_month: Option<i64>,
    /// Two digit number representing the card's expiration year
    pub exp_year: Option<i64>,
    /// The last 4 digits of the card
    pub last4: Option<String>,
}
#[doc(hidden)]
pub struct InternalCardBuilder {
    brand: Option<Option<String>>,
    country: Option<Option<String>>,
    exp_month: Option<Option<i64>>,
    exp_year: Option<Option<i64>>,
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

    impl Deserialize for InternalCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InternalCard>,
        builder: InternalCardBuilder,
    }

    impl Visitor for Place<InternalCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InternalCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InternalCardBuilder {
        type Out = InternalCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brand" => Deserialize::begin(&mut self.brand),
                "country" => Deserialize::begin(&mut self.country),
                "exp_month" => Deserialize::begin(&mut self.exp_month),
                "exp_year" => Deserialize::begin(&mut self.exp_year),
                "last4" => Deserialize::begin(&mut self.last4),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                brand: Deserialize::default(),
                country: Deserialize::default(),
                exp_month: Deserialize::default(),
                exp_year: Deserialize::default(),
                last4: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(brand), Some(country), Some(exp_month), Some(exp_year), Some(last4)) = (
                self.brand.take(),
                self.country.take(),
                self.exp_month,
                self.exp_year,
                self.last4.take(),
            ) else {
                return None;
            };
            Some(Self::Out { brand, country, exp_month, exp_year, last4 })
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

    impl ObjectDeser for InternalCard {
        type Builder = InternalCardBuilder;
    }

    impl FromValueOpt for InternalCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InternalCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "brand" => b.brand = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "exp_month" => b.exp_month = FromValueOpt::from_value(v),
                    "exp_year" => b.exp_year = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
