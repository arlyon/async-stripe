#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Restrictions {
    /// Promotion code restrictions defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        std::collections::HashMap<
            stripe_types::Currency,
            stripe_types::currency_option::CurrencyOption,
        >,
    >,
    /// A Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices.
    pub first_time_transaction: bool,
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    pub minimum_amount: Option<i64>,
    /// Three-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount.
    pub minimum_amount_currency: Option<stripe_types::Currency>,
}
