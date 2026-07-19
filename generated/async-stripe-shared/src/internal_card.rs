#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InternalCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InternalCard").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: InternalCardBuilder {
                    brand: Deserialize::default(),
                    country: Deserialize::default(),
                    exp_month: Deserialize::default(),
                    exp_year: Deserialize::default(),
                    last4: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brand" => Deserialize::begin(&mut self.builder.brand),
                "country" => Deserialize::begin(&mut self.builder.country),
                "exp_month" => Deserialize::begin(&mut self.builder.exp_month),
                "exp_year" => Deserialize::begin(&mut self.builder.exp_year),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(brand), Some(country), Some(exp_month), Some(exp_year), Some(last4)) = (
                self.builder.brand.take(),
                self.builder.country.take(),
                self.builder.exp_month,
                self.builder.exp_year,
                self.builder.last4.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(InternalCard { brand, country, exp_month, exp_year, last4 });
            Ok(())
        }
    }
};
