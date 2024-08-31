use crate::client::{Client, Response};
use crate::resources::{Currency, PromotionCode};

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePromotionCode<'a> {
    /// Whether the promotion code is currently active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The customer-facing code.
    ///
    /// Regardless of case, this code must be unique across all active promotion codes for a specific customer.
    /// If left blank, we will generate one automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,
    /// The coupon for this promotion code.
    pub coupon: &'a str,
    /// The customer that this promotion code can be used by.
    ///
    /// If not set, the promotion code can be used by all customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The timestamp at which this promotion code will expire.
    ///
    /// If the coupon has specified a `redeems_by`, then this value cannot be after the coupon's `redeems_by`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    /// A positive integer specifying the number of times the promotion code can be redeemed.
    ///
    /// If the coupon has specified a `max_redemptions`, then this value cannot be greater than the coupon's `max_redemptions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Settings that restrict the redemption of the promotion code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<CreatePromotionCodeRestrictions<'a>>,
}
impl<'a> CreatePromotionCode<'a> {
    pub fn new(coupon: &'a str) -> Self {
        Self {
            active: None,
            code: None,
            coupon,
            customer: None,
            expand: None,
            expires_at: None,
            max_redemptions: None,
            metadata: None,
            restrictions: None,
        }
    }
}
/// Settings that restrict the redemption of the promotion code.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePromotionCodeRestrictions<'a> {
    /// Promotion codes defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        &'a std::collections::HashMap<Currency, CreatePromotionCodeRestrictionsCurrencyOptions>,
    >,
    /// A Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_time_transaction: Option<bool>,
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
    /// Three-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount_currency: Option<Currency>,
}
impl<'a> CreatePromotionCodeRestrictions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Promotion codes defined in each available currency option.
///
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePromotionCodeRestrictionsCurrencyOptions {
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
}
impl CreatePromotionCodeRestrictionsCurrencyOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CreatePromotionCode<'a> {
    /// A promotion code points to a coupon.
    ///
    /// You can optionally restrict the code to a specific customer, redemption limit, and expiration date.
    pub fn send(&self, client: &Client) -> Response<PromotionCode> {
        client.post_form("/promotion_codes", self)
    }
}
