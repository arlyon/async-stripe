use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// You can delete coupons via the [coupon management](https://dashboard.stripe.com/coupons) page of the Stripe dashboard.
/// However, deleting a coupon does not affect any customers who have already applied the coupon; it means that new customers can’t redeem the coupon.
/// You can also delete coupons via the API.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteCoupon<'a> {
    coupon: &'a stripe_shared::CouponId,
}
impl<'a> DeleteCoupon<'a> {
    /// Construct a new `DeleteCoupon`.
    pub fn new(coupon: &'a stripe_shared::CouponId) -> Self {
        Self { coupon }
    }
}
impl DeleteCoupon<'_> {
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

impl StripeRequest for DeleteCoupon<'_> {
    type Output = stripe_shared::DeletedCoupon;

    fn build(&self) -> RequestBuilder {
        let coupon = self.coupon;
        RequestBuilder::new(StripeMethod::Delete, format!("/coupons/{coupon}"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListCouponBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListCouponBuilder<'a> {
    fn new() -> Self {
        Self { created: None, ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of your coupons.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListCoupon<'a> {
    inner: ListCouponBuilder<'a>,
}
impl<'a> ListCoupon<'a> {
    /// Construct a new `ListCoupon`.
    pub fn new() -> Self {
        Self { inner: ListCouponBuilder::new() }
    }
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
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
impl<'a> Default for ListCoupon<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListCoupon<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Coupon>> {
        stripe_client_core::ListPaginator::new_list("/coupons", self.inner)
    }
}

impl StripeRequest for ListCoupon<'_> {
    type Output = stripe_types::List<stripe_shared::Coupon>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/coupons").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveCouponBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCouponBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the coupon with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveCoupon<'a> {
    inner: RetrieveCouponBuilder<'a>,
    coupon: &'a stripe_shared::CouponId,
}
impl<'a> RetrieveCoupon<'a> {
    /// Construct a new `RetrieveCoupon`.
    pub fn new(coupon: &'a stripe_shared::CouponId) -> Self {
        Self { coupon, inner: RetrieveCouponBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveCoupon<'_> {
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

impl StripeRequest for RetrieveCoupon<'_> {
    type Output = stripe_shared::Coupon;

    fn build(&self) -> RequestBuilder {
        let coupon = self.coupon;
        RequestBuilder::new(StripeMethod::Get, format!("/coupons/{coupon}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateCouponBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_off: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    applies_to: Option<CreateCouponAppliesTo<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency_options: Option<&'a std::collections::HashMap<stripe_types::Currency, CurrencyOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<stripe_shared::CouponDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_in_months: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_redemptions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent_off: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redeem_by: Option<stripe_types::Timestamp>,
}
impl<'a> CreateCouponBuilder<'a> {
    fn new() -> Self {
        Self {
            amount_off: None,
            applies_to: None,
            currency: None,
            currency_options: None,
            duration: None,
            duration_in_months: None,
            expand: None,
            id: None,
            max_redemptions: None,
            metadata: None,
            name: None,
            percent_off: None,
            redeem_by: None,
        }
    }
}
/// A hash containing directions for what this Coupon will apply discounts to.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCouponAppliesTo<'a> {
    /// An array of Product IDs that this Coupon will apply to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<&'a [&'a str]>,
}
impl<'a> CreateCouponAppliesTo<'a> {
    pub fn new() -> Self {
        Self { products: None }
    }
}
impl<'a> Default for CreateCouponAppliesTo<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// You can create coupons easily via the [coupon management](https://dashboard.stripe.com/coupons) page of the Stripe dashboard.
/// Coupon creation is also accessible via the API if you need to create coupons on the fly.
///
/// A coupon has either a `percent_off` or an `amount_off` and `currency`.
/// If you set an `amount_off`, that amount will be subtracted from any invoice’s subtotal.
/// For example, an invoice with a subtotal of $100 will have a final total of $0 if a coupon with an `amount_off` of 20000 is applied to it and an invoice with a subtotal of $300 will have a final total of $100 if a coupon with an `amount_off` of 20000 is applied to it.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCoupon<'a> {
    inner: CreateCouponBuilder<'a>,
}
impl<'a> CreateCoupon<'a> {
    /// Construct a new `CreateCoupon`.
    pub fn new() -> Self {
        Self { inner: CreateCouponBuilder::new() }
    }
    /// A positive integer representing the amount to subtract from an invoice total (required if `percent_off` is not passed).
    pub fn amount_off(mut self, amount_off: i64) -> Self {
        self.inner.amount_off = Some(amount_off);
        self
    }
    /// A hash containing directions for what this Coupon will apply discounts to.
    pub fn applies_to(mut self, applies_to: CreateCouponAppliesTo<'a>) -> Self {
        self.inner.applies_to = Some(applies_to);
        self
    }
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `amount_off` parameter (required if `amount_off` is passed).
    pub fn currency(mut self, currency: stripe_types::Currency) -> Self {
        self.inner.currency = Some(currency);
        self
    }
    /// Coupons defined in each available currency option (only supported if `amount_off` is passed).
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency_options(
        mut self,
        currency_options: &'a std::collections::HashMap<stripe_types::Currency, CurrencyOption>,
    ) -> Self {
        self.inner.currency_options = Some(currency_options);
        self
    }
    /// Specifies how long the discount will be in effect if used on a subscription. Defaults to `once`.
    pub fn duration(mut self, duration: stripe_shared::CouponDuration) -> Self {
        self.inner.duration = Some(duration);
        self
    }
    /// Required only if `duration` is `repeating`, in which case it must be a positive integer that specifies the number of months the discount will be in effect.
    pub fn duration_in_months(mut self, duration_in_months: i64) -> Self {
        self.inner.duration_in_months = Some(duration_in_months);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Unique string of your choice that will be used to identify this coupon when applying it to a customer.
    /// If you don't want to specify a particular code, you can leave the ID blank and we'll generate a random code for you.
    pub fn id(mut self, id: &'a str) -> Self {
        self.inner.id = Some(id);
        self
    }
    /// A positive integer specifying the number of times the coupon can be redeemed before it's no longer valid.
    /// For example, you might have a 50% off coupon that the first 20 readers of your blog can use.
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
    /// Name of the coupon displayed to customers on, for instance invoices, or receipts.
    /// By default the `id` is shown if `name` is not set.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
    /// A positive float larger than 0, and smaller or equal to 100, that represents the discount the coupon will apply (required if `amount_off` is not passed).
    pub fn percent_off(mut self, percent_off: f64) -> Self {
        self.inner.percent_off = Some(percent_off);
        self
    }
    /// Unix timestamp specifying the last time at which the coupon can be redeemed.
    /// After the redeem_by date, the coupon can no longer be applied to new customers.
    pub fn redeem_by(mut self, redeem_by: stripe_types::Timestamp) -> Self {
        self.inner.redeem_by = Some(redeem_by);
        self
    }
}
impl<'a> Default for CreateCoupon<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateCoupon<'_> {
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

impl StripeRequest for CreateCoupon<'_> {
    type Output = stripe_shared::Coupon;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/coupons").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateCouponBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    currency_options: Option<&'a std::collections::HashMap<stripe_types::Currency, CurrencyOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
}
impl<'a> UpdateCouponBuilder<'a> {
    fn new() -> Self {
        Self { currency_options: None, expand: None, metadata: None, name: None }
    }
}
/// Updates the metadata of a coupon.
/// Other coupon details (currency, duration, amount_off) are, by design, not editable.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCoupon<'a> {
    inner: UpdateCouponBuilder<'a>,
    coupon: &'a stripe_shared::CouponId,
}
impl<'a> UpdateCoupon<'a> {
    /// Construct a new `UpdateCoupon`.
    pub fn new(coupon: &'a stripe_shared::CouponId) -> Self {
        Self { coupon, inner: UpdateCouponBuilder::new() }
    }
    /// Coupons defined in each available currency option (only supported if the coupon is amount-based).
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency_options(
        mut self,
        currency_options: &'a std::collections::HashMap<stripe_types::Currency, CurrencyOption>,
    ) -> Self {
        self.inner.currency_options = Some(currency_options);
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
    /// Name of the coupon displayed to customers on, for instance invoices, or receipts.
    /// By default the `id` is shown if `name` is not set.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
}
impl UpdateCoupon<'_> {
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

impl StripeRequest for UpdateCoupon<'_> {
    type Output = stripe_shared::Coupon;

    fn build(&self) -> RequestBuilder {
        let coupon = self.coupon;
        RequestBuilder::new(StripeMethod::Post, format!("/coupons/{coupon}")).form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CurrencyOption {
    /// A positive integer representing the amount to subtract from an invoice total.
    pub amount_off: i64,
}
impl CurrencyOption {
    pub fn new(amount_off: i64) -> Self {
        Self { amount_off }
    }
}
