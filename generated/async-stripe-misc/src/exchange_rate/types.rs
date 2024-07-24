/// `ExchangeRate` objects allow you to determine the rates that Stripe is currently
/// using to convert from one currency to another. Since this number is variable
/// throughout the day, there are various reasons why you might want to know the current
/// rate (for example, to dynamically price an item for a user with a default
/// payment in a foreign currency).
///
/// Please refer to our [Exchange Rates API](https://stripe.com/docs/fx-rates) guide for more details.
///
/// *[Note: this integration path is supported but no longer recommended]* Additionally,
/// you can guarantee that a charge is made with an exchange rate that you expect is
/// current. To do so, you must pass in the exchange_rate to charges endpoints. If the
/// value is no longer up to date, the charge won't go through. Please refer to our
/// [Using with charges](https://stripe.com/docs/exchange-rates) guide for more details.
///
/// -----
///
/// &nbsp;
///
/// *This Exchange Rates API is a Beta Service and is subject to Stripe's terms of service.
/// You may use the API solely for the purpose of transacting on Stripe.
/// For example, the API may be queried in order to:*.
///
/// - *localize prices for processing payments on Stripe*
/// - *reconcile Stripe transactions*
/// - *determine how much money to send to a connected account*
/// - *determine app fees to charge a connected account*
///
/// *Using this Exchange Rates API beta for any purpose other than to transact on Stripe is strictly prohibited and constitutes a violation of Stripe's terms of service.*.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ExchangeRate {
    /// Unique identifier for the object.
    /// Represented as the three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) in lowercase.
    pub id: stripe_misc::ExchangeRateId,
    /// Hash where the keys are supported currencies and the values are the exchange rate at which the base id currency converts to the key currency.
    pub rates: std::collections::HashMap<String, f64>,
}
#[doc(hidden)]
pub struct ExchangeRateBuilder {
    id: Option<stripe_misc::ExchangeRateId>,
    rates: Option<std::collections::HashMap<String, f64>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ExchangeRate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ExchangeRate>,
        builder: ExchangeRateBuilder,
    }

    impl Visitor for Place<ExchangeRate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ExchangeRateBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ExchangeRateBuilder {
        type Out = ExchangeRate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
                "rates" => Deserialize::begin(&mut self.rates),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { id: Deserialize::default(), rates: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { id: self.id.take()?, rates: self.rates.take()? })
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

    impl ObjectDeser for ExchangeRate {
        type Builder = ExchangeRateBuilder;
    }

    impl FromValueOpt for ExchangeRate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ExchangeRateBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "rates" => b.rates = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ExchangeRate {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ExchangeRate", 3)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("rates", &self.rates)?;

        s.serialize_field("object", "exchange_rate")?;
        s.end()
    }
}
impl stripe_types::Object for ExchangeRate {
    type Id = stripe_misc::ExchangeRateId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ExchangeRateId);
