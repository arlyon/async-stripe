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
    /// Hash where the keys are supported currencies and the values are the exchange rate at which the base id currency converts to the key currency.
    pub rates: std::collections::HashMap<String, f64>,
}
impl stripe_types::Object for ExchangeRate {
    type Id = stripe_misc::exchange_rate::ExchangeRateId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(ExchangeRateId);
#[cfg(feature = "exchange_rate")]
mod requests;
#[cfg(feature = "exchange_rate")]
pub use requests::*;
