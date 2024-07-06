use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListPromotionCodeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListPromotionCodeBuilder<'a> {
    fn new() -> Self {
        Self {
            active: None,
            code: None,
            coupon: None,
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Returns a list of your promotion codes.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListPromotionCode<'a> {
    inner: ListPromotionCodeBuilder<'a>,
}
impl<'a> ListPromotionCode<'a> {
    /// Construct a new `ListPromotionCode`.
    pub fn new() -> Self {
        Self { inner: ListPromotionCodeBuilder::new() }
    }
    /// Filter promotion codes by whether they are active.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// Only return promotion codes that have this case-insensitive code.
    pub fn code(mut self, code: &'a str) -> Self {
        self.inner.code = Some(code);
        self
    }
    /// Only return promotion codes for this coupon.
    pub fn coupon(mut self, coupon: &'a str) -> Self {
        self.inner.coupon = Some(coupon);
        self
    }
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// Only return promotion codes that are restricted to this customer.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListPromotionCode<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPromotionCode<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::PromotionCode>> {
        stripe_client_core::ListPaginator::new_list("/promotion_codes", self.inner)
    }
}

impl StripeRequest for ListPromotionCode<'_> {
    type Output = stripe_types::List<stripe_shared::PromotionCode>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/promotion_codes").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrievePromotionCodeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePromotionCodeBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the promotion code with the given ID.
/// In order to retrieve a promotion code by the customer-facing `code` use [list](https://stripe.com/docs/api/promotion_codes/list) with the desired `code`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePromotionCode<'a> {
    inner: RetrievePromotionCodeBuilder<'a>,
    promotion_code: &'a stripe_shared::PromotionCodeId,
}
impl<'a> RetrievePromotionCode<'a> {
    /// Construct a new `RetrievePromotionCode`.
    pub fn new(promotion_code: &'a stripe_shared::PromotionCodeId) -> Self {
        Self { promotion_code, inner: RetrievePromotionCodeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrievePromotionCode<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrievePromotionCode<'_> {
    type Output = stripe_shared::PromotionCode;

    fn build(&self) -> RequestBuilder {
        let promotion_code = self.promotion_code;
        RequestBuilder::new(StripeMethod::Get, format!("/promotion_codes/{promotion_code}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreatePromotionCodeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<&'a str>,
    coupon: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_redemptions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions: Option<CreatePromotionCodeRestrictions<'a>>,
}
impl<'a> CreatePromotionCodeBuilder<'a> {
    fn new(coupon: &'a str) -> Self {
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePromotionCodeRestrictions<'a> {
    /// Promotion codes defined in each available currency option.
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
    /// Three-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount_currency: Option<stripe_types::Currency>,
}
impl<'a> CreatePromotionCodeRestrictions<'a> {
    pub fn new() -> Self {
        Self {
            currency_options: None,
            first_time_transaction: None,
            minimum_amount: None,
            minimum_amount_currency: None,
        }
    }
}
impl<'a> Default for CreatePromotionCodeRestrictions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// A promotion code points to a coupon.
/// You can optionally restrict the code to a specific customer, redemption limit, and expiration date.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePromotionCode<'a> {
    inner: CreatePromotionCodeBuilder<'a>,
}
impl<'a> CreatePromotionCode<'a> {
    /// Construct a new `CreatePromotionCode`.
    pub fn new(coupon: &'a str) -> Self {
        Self { inner: CreatePromotionCodeBuilder::new(coupon) }
    }
    /// Whether the promotion code is currently active.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// The customer-facing code.
    /// Regardless of case, this code must be unique across all active promotion codes for a specific customer.
    /// If left blank, we will generate one automatically.
    pub fn code(mut self, code: &'a str) -> Self {
        self.inner.code = Some(code);
        self
    }
    /// The customer that this promotion code can be used by.
    /// If not set, the promotion code can be used by all customers.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The timestamp at which this promotion code will expire.
    /// If the coupon has specified a `redeems_by`, then this value cannot be after the coupon's `redeems_by`.
    pub fn expires_at(mut self, expires_at: stripe_types::Timestamp) -> Self {
        self.inner.expires_at = Some(expires_at);
        self
    }
    /// A positive integer specifying the number of times the promotion code can be redeemed.
    /// If the coupon has specified a `max_redemptions`, then this value cannot be greater than the coupon's `max_redemptions`.
    pub fn max_redemptions(mut self, max_redemptions: i64) -> Self {
        self.inner.max_redemptions = Some(max_redemptions);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// Settings that restrict the redemption of the promotion code.
    pub fn restrictions(mut self, restrictions: CreatePromotionCodeRestrictions<'a>) -> Self {
        self.inner.restrictions = Some(restrictions);
        self
    }
}
impl CreatePromotionCode<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreatePromotionCode<'_> {
    type Output = stripe_shared::PromotionCode;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/promotion_codes").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdatePromotionCodeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions: Option<UpdatePromotionCodeRestrictions<'a>>,
}
impl<'a> UpdatePromotionCodeBuilder<'a> {
    fn new() -> Self {
        Self { active: None, expand: None, metadata: None, restrictions: None }
    }
}
/// Settings that restrict the redemption of the promotion code.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePromotionCodeRestrictions<'a> {
    /// Promotion codes defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options:
        Option<&'a std::collections::HashMap<stripe_types::Currency, CurrencyOption>>,
}
impl<'a> UpdatePromotionCodeRestrictions<'a> {
    pub fn new() -> Self {
        Self { currency_options: None }
    }
}
impl<'a> Default for UpdatePromotionCodeRestrictions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates the specified promotion code by setting the values of the parameters passed.
/// Most fields are, by design, not editable.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePromotionCode<'a> {
    inner: UpdatePromotionCodeBuilder<'a>,
    promotion_code: &'a stripe_shared::PromotionCodeId,
}
impl<'a> UpdatePromotionCode<'a> {
    /// Construct a new `UpdatePromotionCode`.
    pub fn new(promotion_code: &'a stripe_shared::PromotionCodeId) -> Self {
        Self { promotion_code, inner: UpdatePromotionCodeBuilder::new() }
    }
    /// Whether the promotion code is currently active.
    /// A promotion code can only be reactivated when the coupon is still valid and the promotion code is otherwise redeemable.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// Settings that restrict the redemption of the promotion code.
    pub fn restrictions(mut self, restrictions: UpdatePromotionCodeRestrictions<'a>) -> Self {
        self.inner.restrictions = Some(restrictions);
        self
    }
}
impl UpdatePromotionCode<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdatePromotionCode<'_> {
    type Output = stripe_shared::PromotionCode;

    fn build(&self) -> RequestBuilder {
        let promotion_code = self.promotion_code;
        RequestBuilder::new(StripeMethod::Post, format!("/promotion_codes/{promotion_code}"))
            .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CurrencyOption {
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
}
impl CurrencyOption {
    pub fn new() -> Self {
        Self { minimum_amount: None }
    }
}
impl Default for CurrencyOption {
    fn default() -> Self {
        Self::new()
    }
}
