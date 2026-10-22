use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListProductCatalogTrialOfferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prices: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListProductCatalogTrialOfferBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListProductCatalogTrialOfferBuilder").finish_non_exhaustive()
    }
}
impl ListProductCatalogTrialOfferBuilder {
    fn new() -> Self {
        Self {
            active: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            prices: None,
            starting_after: None,
        }
    }
}
/// Returns a list of trial offers.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListProductCatalogTrialOffer {
    inner: ListProductCatalogTrialOfferBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListProductCatalogTrialOffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListProductCatalogTrialOffer").finish_non_exhaustive()
    }
}
impl ListProductCatalogTrialOffer {
    /// Construct a new `ListProductCatalogTrialOffer`.
    pub fn new() -> Self {
        Self { inner: ListProductCatalogTrialOfferBuilder::new() }
    }
    /// Only return trial offers that are active (`true`) or archived (`false`).
    /// If omitted, both active and archived trial offers are returned.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Only return trial offers that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
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
    /// Only return trial offers that reference these prices (during the trial period).
    pub fn prices(mut self, prices: impl Into<Vec<String>>) -> Self {
        self.inner.prices = Some(prices.into());
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
impl Default for ListProductCatalogTrialOffer {
    fn default() -> Self {
        Self::new()
    }
}
impl ListProductCatalogTrialOffer {
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
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_product::ProductCatalogTrialOffer>,
    > {
        stripe_client_core::ListPaginator::new_list("/product_catalog/trial_offers", &self.inner)
    }
}

impl StripeRequest for ListProductCatalogTrialOffer {
    type Output = stripe_types::List<stripe_product::ProductCatalogTrialOffer>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/product_catalog/trial_offers").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveProductCatalogTrialOfferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveProductCatalogTrialOfferBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveProductCatalogTrialOfferBuilder").finish_non_exhaustive()
    }
}
impl RetrieveProductCatalogTrialOfferBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the trial offer with the given ID.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveProductCatalogTrialOffer {
    inner: RetrieveProductCatalogTrialOfferBuilder,
    id: stripe_product::ProductCatalogTrialOfferId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveProductCatalogTrialOffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveProductCatalogTrialOffer").finish_non_exhaustive()
    }
}
impl RetrieveProductCatalogTrialOffer {
    /// Construct a new `RetrieveProductCatalogTrialOffer`.
    pub fn new(id: impl Into<stripe_product::ProductCatalogTrialOfferId>) -> Self {
        Self { id: id.into(), inner: RetrieveProductCatalogTrialOfferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveProductCatalogTrialOffer {
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

impl StripeRequest for RetrieveProductCatalogTrialOffer {
    type Output = stripe_product::ProductCatalogTrialOffer;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/product_catalog/trial_offers/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateProductCatalogTrialOfferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    duration: CreateProductCatalogTrialOfferDuration,
    end_behavior: CreateProductCatalogTrialOfferEndBehavior,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    price: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateProductCatalogTrialOfferBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateProductCatalogTrialOfferBuilder").finish_non_exhaustive()
    }
}
impl CreateProductCatalogTrialOfferBuilder {
    fn new(
        duration: impl Into<CreateProductCatalogTrialOfferDuration>,
        end_behavior: impl Into<CreateProductCatalogTrialOfferEndBehavior>,
        price: impl Into<String>,
    ) -> Self {
        Self {
            active: None,
            duration: duration.into(),
            end_behavior: end_behavior.into(),
            expand: None,
            nickname: None,
            price: price.into(),
        }
    }
}
/// Duration of one service period of the trial.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateProductCatalogTrialOfferDuration {
    /// The relative duration of the trial period computed as the number of recurring price intervals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative: Option<CreateProductCatalogTrialOfferDurationRelative>,
    /// Specifies how the trial offer duration is determined.
    #[serde(rename = "type")]
    pub type_: CreateProductCatalogTrialOfferDurationType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateProductCatalogTrialOfferDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateProductCatalogTrialOfferDuration").finish_non_exhaustive()
    }
}
impl CreateProductCatalogTrialOfferDuration {
    pub fn new(type_: impl Into<CreateProductCatalogTrialOfferDurationType>) -> Self {
        Self { relative: None, type_: type_.into() }
    }
}
/// The relative duration of the trial period computed as the number of recurring price intervals.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateProductCatalogTrialOfferDurationRelative {
    /// The number of recurring price's interval to apply for the trial period.
    pub iterations: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateProductCatalogTrialOfferDurationRelative {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateProductCatalogTrialOfferDurationRelative").finish_non_exhaustive()
    }
}
impl CreateProductCatalogTrialOfferDurationRelative {
    pub fn new(iterations: impl Into<i64>) -> Self {
        Self { iterations: iterations.into() }
    }
}
/// Specifies how the trial offer duration is determined.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateProductCatalogTrialOfferDurationType {
    Relative,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateProductCatalogTrialOfferDurationType {
    pub fn as_str(&self) -> &str {
        use CreateProductCatalogTrialOfferDurationType::*;
        match self {
            Relative => "relative",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateProductCatalogTrialOfferDurationType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateProductCatalogTrialOfferDurationType::*;
        match s {
            "relative" => Ok(Relative),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateProductCatalogTrialOfferDurationType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateProductCatalogTrialOfferDurationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateProductCatalogTrialOfferDurationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateProductCatalogTrialOfferDurationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateProductCatalogTrialOfferDurationType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateProductCatalogTrialOfferDurationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateProductCatalogTrialOfferDurationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Define behavior that occurs at the end of the trial.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateProductCatalogTrialOfferEndBehavior {
    /// The transition to apply when the trial offer ends.
    pub transition: CreateProductCatalogTrialOfferEndBehaviorTransition,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateProductCatalogTrialOfferEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateProductCatalogTrialOfferEndBehavior").finish_non_exhaustive()
    }
}
impl CreateProductCatalogTrialOfferEndBehavior {
    pub fn new(transition: impl Into<CreateProductCatalogTrialOfferEndBehaviorTransition>) -> Self {
        Self { transition: transition.into() }
    }
}
/// The transition to apply when the trial offer ends.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateProductCatalogTrialOfferEndBehaviorTransition {
    /// The price to transition the recurring item to when the trial offer ends.
    pub price: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateProductCatalogTrialOfferEndBehaviorTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateProductCatalogTrialOfferEndBehaviorTransition")
            .finish_non_exhaustive()
    }
}
impl CreateProductCatalogTrialOfferEndBehaviorTransition {
    pub fn new(price: impl Into<String>) -> Self {
        Self { price: price.into() }
    }
}
/// Creates a trial offer.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateProductCatalogTrialOffer {
    inner: CreateProductCatalogTrialOfferBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateProductCatalogTrialOffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateProductCatalogTrialOffer").finish_non_exhaustive()
    }
}
impl CreateProductCatalogTrialOffer {
    /// Construct a new `CreateProductCatalogTrialOffer`.
    pub fn new(
        duration: impl Into<CreateProductCatalogTrialOfferDuration>,
        end_behavior: impl Into<CreateProductCatalogTrialOfferEndBehavior>,
        price: impl Into<String>,
    ) -> Self {
        Self {
            inner: CreateProductCatalogTrialOfferBuilder::new(
                duration.into(),
                end_behavior.into(),
                price.into(),
            ),
        }
    }
    /// Whether the trial offer can be used for new subscriptions. Defaults to true.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A brief description of the trial offer, hidden from customers.
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.inner.nickname = Some(nickname.into());
        self
    }
}
impl CreateProductCatalogTrialOffer {
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

impl StripeRequest for CreateProductCatalogTrialOffer {
    type Output = stripe_product::ProductCatalogTrialOffer;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/product_catalog/trial_offers").form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateProductCatalogTrialOfferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateProductCatalogTrialOfferBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateProductCatalogTrialOfferBuilder").finish_non_exhaustive()
    }
}
impl UpdateProductCatalogTrialOfferBuilder {
    fn new() -> Self {
        Self { active: None, expand: None }
    }
}
/// Updates the specified trial offer by setting the values of the parameters passed.
/// Any parameters not provided are left unchanged.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateProductCatalogTrialOffer {
    inner: UpdateProductCatalogTrialOfferBuilder,
    id: stripe_product::ProductCatalogTrialOfferId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateProductCatalogTrialOffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateProductCatalogTrialOffer").finish_non_exhaustive()
    }
}
impl UpdateProductCatalogTrialOffer {
    /// Construct a new `UpdateProductCatalogTrialOffer`.
    pub fn new(id: impl Into<stripe_product::ProductCatalogTrialOfferId>) -> Self {
        Self { id: id.into(), inner: UpdateProductCatalogTrialOfferBuilder::new() }
    }
    /// Whether the trial offer can be used for new purchases.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl UpdateProductCatalogTrialOffer {
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

impl StripeRequest for UpdateProductCatalogTrialOffer {
    type Output = stripe_product::ProductCatalogTrialOffer;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/product_catalog/trial_offers/{id}"))
            .form(&self.inner)
    }
}
