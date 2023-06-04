use crate::{Client, Response};

impl crate::coupon::Coupon {
    /// Returns a list of your coupons.
    pub fn list(
        client: &Client,
        params: ListCoupon,
    ) -> Response<crate::List<crate::coupon::Coupon>> {
        client.get_query("/coupons", params)
    }
    /// You can create coupons easily via the [coupon management](https://dashboard.stripe.com/coupons) page of the Stripe dashboard.
    ///
    /// Coupon creation is also accessible via the API if you need to create coupons on the fly.  A coupon has either a `percent_off` or an `amount_off` and `currency`.
    /// If you set an `amount_off`, that amount will be subtracted from any invoice’s subtotal.
    /// For example, an invoice with a subtotal of $100 will have a final total of $0 if a coupon with an `amount_off` of 20000 is applied to it and an invoice with a subtotal of $300 will have a final total of $100 if a coupon with an `amount_off` of 20000 is applied to it.
    pub fn create(client: &Client, params: CreateCoupon) -> Response<crate::coupon::Coupon> {
        client.send_form("/coupons", params, http_types::Method::Post)
    }
    /// Retrieves the coupon with the given ID.
    pub fn retrieve(
        client: &Client,
        coupon: &crate::coupon::CouponId,
        params: RetrieveCoupon,
    ) -> Response<crate::coupon::Coupon> {
        client.get_query(&format!("/coupons/{coupon}", coupon = coupon), params)
    }
    /// Updates the metadata of a coupon.
    ///
    /// Other coupon details (currency, duration, amount_off) are, by design, not editable.
    pub fn update(
        client: &Client,
        coupon: &crate::coupon::CouponId,
        params: UpdateCoupon,
    ) -> Response<crate::coupon::Coupon> {
        client.send_form(
            &format!("/coupons/{coupon}", coupon = coupon),
            params,
            http_types::Method::Post,
        )
    }
    /// You can delete coupons via the [coupon management](https://dashboard.stripe.com/coupons) page of the Stripe dashboard.
    ///
    /// However, deleting a coupon does not affect any customers who have already applied the coupon; it means that new customers can’t redeem the coupon.
    /// You can also delete coupons via the API.
    pub fn delete(
        client: &Client,
        coupon: &crate::coupon::CouponId,
    ) -> Response<crate::coupon::DeletedCoupon> {
        client.send(&format!("/coupons/{coupon}", coupon = coupon), http_types::Method::Delete)
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListCoupon<'a> {
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::RangeQueryTs>,
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
impl<'a> ListCoupon<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCoupon<'a> {
    /// A positive integer representing the amount to subtract from an invoice total (required if `percent_off` is not passed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<i64>,
    /// A hash containing directions for what this Coupon will apply discounts to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<CreateCouponAppliesTo<'a>>,
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `amount_off` parameter (required if `amount_off` is passed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::Currency>,
    /// Coupons defined in each available currency option (only supported if `amount_off` is passed).
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<CreateCouponCurrencyOptions>,
    /// Specifies how long the discount will be in effect if used on a subscription.
    ///
    /// Defaults to `once`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<CreateCouponDuration>,
    /// Required only if `duration` is `repeating`, in which case it must be a positive integer that specifies the number of months the discount will be in effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_months: Option<i64>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Unique string of your choice that will be used to identify this coupon when applying it to a customer.
    ///
    /// If you don't want to specify a particular code, you can leave the ID blank and we'll generate a random code for you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// A positive integer specifying the number of times the coupon can be redeemed before it's no longer valid.
    ///
    /// For example, you might have a 50% off coupon that the first 20 readers of your blog can use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// Name of the coupon displayed to customers on, for instance invoices, or receipts.
    ///
    /// By default the `id` is shown if `name` is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// A positive float larger than 0, and smaller or equal to 100, that represents the discount the coupon will apply (required if `amount_off` is not passed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<f64>,
    /// Unix timestamp specifying the last time at which the coupon can be redeemed.
    ///
    /// After the redeem_by date, the coupon can no longer be applied to new customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_by: Option<crate::Timestamp>,
}
impl<'a> CreateCoupon<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A hash containing directions for what this Coupon will apply discounts to.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCouponAppliesTo<'a> {
    /// An array of Product IDs that this Coupon will apply to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<&'a [&'a str]>,
}
impl<'a> CreateCouponAppliesTo<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Coupons defined in each available currency option (only supported if `amount_off` is passed).
///
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCouponCurrencyOptions {
    /// A positive integer representing the amount to subtract from an invoice total.
    pub amount_off: i64,
}
impl CreateCouponCurrencyOptions {
    pub fn new(amount_off: i64) -> Self {
        Self { amount_off }
    }
}
/// Specifies how long the discount will be in effect if used on a subscription.
///
/// Defaults to `once`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateCouponDuration {
    Forever,
    Once,
    Repeating,
}

impl CreateCouponDuration {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Forever => "forever",
            Self::Once => "once",
            Self::Repeating => "repeating",
        }
    }
}

impl AsRef<str> for CreateCouponDuration {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCouponDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCoupon<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCoupon<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCoupon<'a> {
    /// Coupons defined in each available currency option (only supported if the coupon is amount-based).
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<UpdateCouponCurrencyOptions>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// Name of the coupon displayed to customers on, for instance invoices, or receipts.
    ///
    /// By default the `id` is shown if `name` is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}
impl<'a> UpdateCoupon<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Coupons defined in each available currency option (only supported if the coupon is amount-based).
///
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCouponCurrencyOptions {
    /// A positive integer representing the amount to subtract from an invoice total.
    pub amount_off: i64,
}
impl UpdateCouponCurrencyOptions {
    pub fn new(amount_off: i64) -> Self {
        Self { amount_off }
    }
}
