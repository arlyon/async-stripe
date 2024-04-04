#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AccountAnnualRevenue {
    /// A non-negative integer representing the amount in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// The close-out date of the preceding fiscal year in ISO 8601 format.
    /// E.g.
    /// 2023-12-31 for the 31st of December, 2023.
    pub fiscal_year_end: Option<String>,
}
