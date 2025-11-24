use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListPromotionCodeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coupon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListPromotionCodeBuilder {
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
pub struct ListPromotionCode {
    inner: ListPromotionCodeBuilder,
}
impl ListPromotionCode {
    /// Construct a new `ListPromotionCode`.
    pub fn new() -> Self {
        Self { inner: ListPromotionCodeBuilder::new() }
    }
    /// Filter promotion codes by whether they are active.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Only return promotion codes that have this case-insensitive code.
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.inner.code = Some(code.into());
        self
    }
    /// Only return promotion codes for this coupon.
    pub fn coupon(mut self, coupon: impl Into<String>) -> Self {
        self.inner.coupon = Some(coupon.into());
        self
    }
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return promotion codes that are restricted to this customer.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListPromotionCode {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPromotionCode {
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
        stripe_client_core::ListPaginator::new_list("/promotion_codes", &self.inner)
    }
}

impl StripeRequest for ListPromotionCode {
    type Output = stripe_types::List<stripe_shared::PromotionCode>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/promotion_codes").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrievePromotionCodeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrievePromotionCodeBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the promotion code with the given ID.
/// In order to retrieve a promotion code by the customer-facing `code` use [list](https://stripe.com/docs/api/promotion_codes/list) with the desired `code`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePromotionCode {
    inner: RetrievePromotionCodeBuilder,
    promotion_code: stripe_shared::PromotionCodeId,
}
impl RetrievePromotionCode {
    /// Construct a new `RetrievePromotionCode`.
    pub fn new(promotion_code: impl Into<stripe_shared::PromotionCodeId>) -> Self {
        Self { promotion_code: promotion_code.into(), inner: RetrievePromotionCodeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrievePromotionCode {
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

impl StripeRequest for RetrievePromotionCode {
    type Output = stripe_shared::PromotionCode;

    fn build(&self) -> RequestBuilder {
        let promotion_code = &self.promotion_code;
        RequestBuilder::new(StripeMethod::Get, format!("/promotion_codes/{promotion_code}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreatePromotionCodeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_redemptions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    promotion: CreatePromotionCodePromotion,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions: Option<CreatePromotionCodeRestrictions>,
}
impl CreatePromotionCodeBuilder {
    fn new(promotion: impl Into<CreatePromotionCodePromotion>) -> Self {
        Self {
            active: None,
            code: None,
            customer: None,
            expand: None,
            expires_at: None,
            max_redemptions: None,
            metadata: None,
            promotion: promotion.into(),
            restrictions: None,
        }
    }
}
/// The promotion referenced by this promotion code.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePromotionCodePromotion {
    /// If promotion `type` is `coupon`, the coupon for this promotion code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    /// Specifies the type of promotion.
    #[serde(rename = "type")]
    pub type_: CreatePromotionCodePromotionType,
}
impl CreatePromotionCodePromotion {
    pub fn new(type_: impl Into<CreatePromotionCodePromotionType>) -> Self {
        Self { coupon: None, type_: type_.into() }
    }
}
/// Specifies the type of promotion.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePromotionCodePromotionType {
    Coupon,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePromotionCodePromotionType {
    pub fn as_str(&self) -> &str {
        use CreatePromotionCodePromotionType::*;
        match self {
            Coupon => "coupon",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePromotionCodePromotionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePromotionCodePromotionType::*;
        match s {
            "coupon" => Ok(Coupon),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePromotionCodePromotionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePromotionCodePromotionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePromotionCodePromotionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePromotionCodePromotionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePromotionCodePromotionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Settings that restrict the redemption of the promotion code.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePromotionCodeRestrictions {
    /// Promotion codes defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<std::collections::HashMap<stripe_types::Currency, CurrencyOption>>,
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
impl CreatePromotionCodeRestrictions {
    pub fn new() -> Self {
        Self {
            currency_options: None,
            first_time_transaction: None,
            minimum_amount: None,
            minimum_amount_currency: None,
        }
    }
}
impl Default for CreatePromotionCodeRestrictions {
    fn default() -> Self {
        Self::new()
    }
}
/// A promotion code points to an underlying promotion.
/// You can optionally restrict the code to a specific customer, redemption limit, and expiration date.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePromotionCode {
    inner: CreatePromotionCodeBuilder,
}
impl CreatePromotionCode {
    /// Construct a new `CreatePromotionCode`.
    pub fn new(promotion: impl Into<CreatePromotionCodePromotion>) -> Self {
        Self { inner: CreatePromotionCodeBuilder::new(promotion.into()) }
    }
    /// Whether the promotion code is currently active.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// The customer-facing code.
    /// Regardless of case, this code must be unique across all active promotion codes for a specific customer.
    /// Valid characters are lower case letters (a-z), upper case letters (A-Z), and digits (0-9).
    ///
    /// If left blank, we will generate one automatically.
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.inner.code = Some(code.into());
        self
    }
    /// The customer that this promotion code can be used by.
    /// If not set, the promotion code can be used by all customers.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The timestamp at which this promotion code will expire.
    /// If the coupon has specified a `redeems_by`, then this value cannot be after the coupon's `redeems_by`.
    pub fn expires_at(mut self, expires_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.expires_at = Some(expires_at.into());
        self
    }
    /// A positive integer specifying the number of times the promotion code can be redeemed.
    /// If the coupon has specified a `max_redemptions`, then this value cannot be greater than the coupon's `max_redemptions`.
    pub fn max_redemptions(mut self, max_redemptions: impl Into<i64>) -> Self {
        self.inner.max_redemptions = Some(max_redemptions.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Settings that restrict the redemption of the promotion code.
    pub fn restrictions(
        mut self,
        restrictions: impl Into<CreatePromotionCodeRestrictions>,
    ) -> Self {
        self.inner.restrictions = Some(restrictions.into());
        self
    }
}
impl CreatePromotionCode {
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

impl StripeRequest for CreatePromotionCode {
    type Output = stripe_shared::PromotionCode;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/promotion_codes").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdatePromotionCodeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions: Option<UpdatePromotionCodeRestrictions>,
}
impl UpdatePromotionCodeBuilder {
    fn new() -> Self {
        Self { active: None, expand: None, metadata: None, restrictions: None }
    }
}
/// Settings that restrict the redemption of the promotion code.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePromotionCodeRestrictions {
    /// Promotion codes defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<std::collections::HashMap<stripe_types::Currency, CurrencyOption>>,
}
impl UpdatePromotionCodeRestrictions {
    pub fn new() -> Self {
        Self { currency_options: None }
    }
}
impl Default for UpdatePromotionCodeRestrictions {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates the specified promotion code by setting the values of the parameters passed.
/// Most fields are, by design, not editable.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePromotionCode {
    inner: UpdatePromotionCodeBuilder,
    promotion_code: stripe_shared::PromotionCodeId,
}
impl UpdatePromotionCode {
    /// Construct a new `UpdatePromotionCode`.
    pub fn new(promotion_code: impl Into<stripe_shared::PromotionCodeId>) -> Self {
        Self { promotion_code: promotion_code.into(), inner: UpdatePromotionCodeBuilder::new() }
    }
    /// Whether the promotion code is currently active.
    /// A promotion code can only be reactivated when the coupon is still valid and the promotion code is otherwise redeemable.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Settings that restrict the redemption of the promotion code.
    pub fn restrictions(
        mut self,
        restrictions: impl Into<UpdatePromotionCodeRestrictions>,
    ) -> Self {
        self.inner.restrictions = Some(restrictions.into());
        self
    }
}
impl UpdatePromotionCode {
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

impl StripeRequest for UpdatePromotionCode {
    type Output = stripe_shared::PromotionCode;

    fn build(&self) -> RequestBuilder {
        let promotion_code = &self.promotion_code;
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
