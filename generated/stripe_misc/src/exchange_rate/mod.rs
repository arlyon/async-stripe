/// `Exchange Rate` objects allow you to determine the rates that Stripe is
/// currently using to convert from one currency to another.
///
/// Since this number is variable throughout the day, there are various reasons why you might want to know the current rate (for example, to dynamically price an item for a user with a default payment in a foreign currency).  If you want a guarantee that the charge is made with a certain exchange rate you expect is current, you can pass in `exchange_rate` to charges endpoints. If the value is no longer up to date, the charge won't go through.
/// Please refer to our [Exchange Rates API](https://stripe.com/docs/exchange-rates) guide for more details.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ExchangeRate {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ExchangeRateObject {
    ExchangeRate,
}

impl ExchangeRateObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExchangeRate => "exchange_rate",
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
impl stripe_types::Object for ExchangeRate {
    type Id = stripe_misc::exchange_rate::ExchangeRateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ExchangeRateId);
pub mod requests;
