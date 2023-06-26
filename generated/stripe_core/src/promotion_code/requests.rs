impl stripe_core::promotion_code::PromotionCode {
    /// Retrieves the promotion code with the given ID.
    ///
    /// In order to retrieve a promotion code by the customer-facing `code` use [list](https://stripe.com/docs/api/promotion_codes/list) with the desired `code`.
    pub fn retrieve(
        client: &stripe::Client,
        promotion_code: &stripe_core::promotion_code::PromotionCodeId,
        params: RetrievePromotionCode,
    ) -> stripe::Response<stripe_core::promotion_code::PromotionCode> {
        client.get_query(
            &format!("/promotion_codes/{promotion_code}", promotion_code = promotion_code),
            params,
        )
    }
    /// A promotion code points to a coupon.
    ///
    /// You can optionally restrict the code to a specific customer, redemption limit, and expiration date.
    pub fn create(
        client: &stripe::Client,
        params: CreatePromotionCode,
    ) -> stripe::Response<stripe_core::promotion_code::PromotionCode> {
        client.send_form("/promotion_codes", params, http_types::Method::Post)
    }
    /// Updates the specified promotion code by setting the values of the parameters passed.
    ///
    /// Most fields are, by design, not editable.
    pub fn update(
        client: &stripe::Client,
        promotion_code: &stripe_core::promotion_code::PromotionCodeId,
        params: UpdatePromotionCode,
    ) -> stripe::Response<stripe_core::promotion_code::PromotionCode> {
        client.send_form(
            &format!("/promotion_codes/{promotion_code}", promotion_code = promotion_code),
            params,
            http_types::Method::Post,
        )
    }
    /// Returns a list of your promotion codes.
    pub fn list(
        client: &stripe::Client,
        params: ListPromotionCode,
    ) -> stripe::Response<stripe_types::List<stripe_core::promotion_code::PromotionCode>> {
        client.get_query("/promotion_codes", params)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePromotionCode<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePromotionCode<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
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
    pub expires_at: Option<stripe_types::Timestamp>,
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
            active: Default::default(),
            code: Default::default(),
            coupon,
            customer: Default::default(),
            expand: Default::default(),
            expires_at: Default::default(),
            max_redemptions: Default::default(),
            metadata: Default::default(),
            restrictions: Default::default(),
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
    pub currency_options:
        Option<&'a std::collections::HashMap<stripe_types::Currency, CurrencyOption>>,
    /// A Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_time_transaction: Option<bool>,
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
    /// Three-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount_currency: Option<stripe_types::Currency>,
}
impl<'a> CreatePromotionCodeRestrictions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePromotionCode<'a> {
    /// Whether the promotion code is currently active.
    ///
    /// A promotion code can only be reactivated when the coupon is still valid and the promotion code is otherwise redeemable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Settings that restrict the redemption of the promotion code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<UpdatePromotionCodeRestrictions<'a>>,
}
impl<'a> UpdatePromotionCode<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings that restrict the redemption of the promotion code.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePromotionCodeRestrictions<'a> {
    /// Promotion codes defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options:
        Option<&'a std::collections::HashMap<stripe_types::Currency, CurrencyOption>>,
}
impl<'a> UpdatePromotionCodeRestrictions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListPromotionCode<'a> {
    /// Filter promotion codes by whether they are active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Only return promotion codes that have this case-insensitive code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,
    /// Only return promotion codes for this coupon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// Only return promotion codes that are restricted to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListPromotionCode<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CurrencyOption {
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
}
impl CurrencyOption {
    pub fn new() -> Self {
        Self::default()
    }
}
