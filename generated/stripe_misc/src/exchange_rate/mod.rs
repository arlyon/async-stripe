/// `Exchange Rate` objects allow you to determine the rates that Stripe is
/// currently using to convert from one currency to another.
///
/// Since this number is variable throughout the day, there are various reasons why you might want to know the current rate (for example, to dynamically price an item for a user with a default payment in a foreign currency).  If you want a guarantee that the charge is made with a certain exchange rate you expect is current, you can pass in `exchange_rate` to charges endpoints. If the value is no longer up to date, the charge won't go through.
/// Please refer to our [Exchange Rates API](https://stripe.com/docs/exchange-rates) guide for more details.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ExchangeRate {
    /// Unique identifier for the object.
    ///
    /// Represented as the three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) in lowercase.
    pub id: stripe_misc::exchange_rate::ExchangeRateId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ExchangeRateObject,
    /// Hash where the keys are supported currencies and the values are the exchange rate at which the base id currency converts to the key currency.
    pub rates: std::collections::HashMap<String, f64>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ExchangeRateObject {
    ExchangeRate,
}

impl ExchangeRateObject {
    pub fn as_str(self) -> &'static str {
        use ExchangeRateObject::*;
        match self {
            ExchangeRate => "exchange_rate",
        }
    }
}

impl std::str::FromStr for ExchangeRateObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ExchangeRateObject::*;
        match s {
            "exchange_rate" => Ok(ExchangeRate),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ExchangeRateObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ExchangeRateObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ExchangeRateObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ExchangeRateObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ExchangeRateObject"))
    }
}
impl stripe_types::Object for ExchangeRate {
    type Id = stripe_misc::exchange_rate::ExchangeRateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ExchangeRateId);
pub mod requests;
