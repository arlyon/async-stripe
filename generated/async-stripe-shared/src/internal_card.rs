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
                brand: Some(None),
                country: Some(None),
                exp_month: Some(None),
                exp_year: Some(None),
                last4: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    brand: Some(brand),
                    country: Some(country),
                    exp_month: Some(exp_month),
                    exp_year: Some(exp_year),
                    last4: Some(last4),
                } => Some(Self::Out { brand, country, exp_month, exp_year, last4 }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for InternalCard {
        type Builder = InternalCardBuilder;
    }

    impl FromValueOpt for InternalCard {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
