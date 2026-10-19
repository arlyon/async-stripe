use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListIssuingCardBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    cardholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp_month: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp_year: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    personalization_design: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::IssuingCardStatus>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::IssuingCardType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIssuingCardBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIssuingCardBuilder").finish_non_exhaustive()
    }
}
impl ListIssuingCardBuilder {
    fn new() -> Self {
        Self {
            cardholder: None,
            created: None,
            ending_before: None,
            exp_month: None,
            exp_year: None,
            expand: None,
            last4: None,
            limit: None,
            personalization_design: None,
            starting_after: None,
            status: None,
            type_: None,
        }
    }
}
/// Returns a list of Issuing `Card` objects.
/// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListIssuingCard {
    inner: ListIssuingCardBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIssuingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIssuingCard").finish_non_exhaustive()
    }
}
impl ListIssuingCard {
    /// Construct a new `ListIssuingCard`.
    pub fn new() -> Self {
        Self { inner: ListIssuingCardBuilder::new() }
    }
    /// Only return cards belonging to the Cardholder with the provided ID.
    pub fn cardholder(mut self, cardholder: impl Into<String>) -> Self {
        self.inner.cardholder = Some(cardholder.into());
        self
    }
    /// Only return cards that were issued during the given date interval.
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
    /// Only return cards that have the given expiration month.
    pub fn exp_month(mut self, exp_month: impl Into<i64>) -> Self {
        self.inner.exp_month = Some(exp_month.into());
        self
    }
    /// Only return cards that have the given expiration year.
    pub fn exp_year(mut self, exp_year: impl Into<i64>) -> Self {
        self.inner.exp_year = Some(exp_year.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Only return cards that have the given last four digits.
    pub fn last4(mut self, last4: impl Into<String>) -> Self {
        self.inner.last4 = Some(last4.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    pub fn personalization_design(mut self, personalization_design: impl Into<String>) -> Self {
        self.inner.personalization_design = Some(personalization_design.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return cards that have the given status. One of `active`, `inactive`, or `canceled`.
    pub fn status(mut self, status: impl Into<stripe_shared::IssuingCardStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
    /// Only return cards that have the given type. One of `virtual` or `physical`.
    pub fn type_(mut self, type_: impl Into<stripe_shared::IssuingCardType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
}
impl Default for ListIssuingCard {
    fn default() -> Self {
        Self::new()
    }
}
impl ListIssuingCard {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::IssuingCard>> {
        stripe_client_core::ListPaginator::new_list("/issuing/cards", &self.inner)
    }
}

impl StripeRequest for ListIssuingCard {
    type Output = stripe_types::List<stripe_shared::IssuingCard>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/issuing/cards").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveIssuingCardBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIssuingCardBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIssuingCardBuilder").finish_non_exhaustive()
    }
}
impl RetrieveIssuingCardBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an Issuing `Card` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveIssuingCard {
    inner: RetrieveIssuingCardBuilder,
    card: stripe_shared::IssuingCardId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIssuingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIssuingCard").finish_non_exhaustive()
    }
}
impl RetrieveIssuingCard {
    /// Construct a new `RetrieveIssuingCard`.
    pub fn new(card: impl Into<stripe_shared::IssuingCardId>) -> Self {
        Self { card: card.into(), inner: RetrieveIssuingCardBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveIssuingCard {
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

impl StripeRequest for RetrieveIssuingCard {
    type Output = stripe_shared::IssuingCard;

    fn build(&self) -> RequestBuilder {
        let card = &self.card;
        RequestBuilder::new(StripeMethod::Get, format!("/issuing/cards/{card}")).query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateIssuingCardBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    cardholder: Option<String>,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp_month: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp_year: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    financial_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_controls: Option<CreateIssuingCardLifecycleControls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    personalization_design: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pin: Option<EncryptedPinParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replacement_for: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replacement_reason: Option<CreateIssuingCardReplacementReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    second_line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping: Option<ShippingSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spending_controls: Option<AuthorizationControlsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::BillingMeterStatus>,
    #[serde(rename = "type")]
    type_: stripe_shared::IssuingCardType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingCardBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingCardBuilder").finish_non_exhaustive()
    }
}
impl CreateIssuingCardBuilder {
    fn new(
        currency: impl Into<stripe_types::Currency>,
        type_: impl Into<stripe_shared::IssuingCardType>,
    ) -> Self {
        Self {
            cardholder: None,
            currency: currency.into(),
            exp_month: None,
            exp_year: None,
            expand: None,
            financial_account: None,
            lifecycle_controls: None,
            metadata: None,
            personalization_design: None,
            pin: None,
            replacement_for: None,
            replacement_reason: None,
            second_line: None,
            shipping: None,
            spending_controls: None,
            status: None,
            type_: type_.into(),
        }
    }
}
/// Rules that control the lifecycle of this card, such as automatic cancellation.
/// Refer to our [documentation](/issuing/controls/lifecycle-controls) for more details.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingCardLifecycleControls {
    /// Cancels the card after the specified conditions are met.
    pub cancel_after: CreateIssuingCardLifecycleControlsCancelAfter,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingCardLifecycleControls {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingCardLifecycleControls").finish_non_exhaustive()
    }
}
impl CreateIssuingCardLifecycleControls {
    pub fn new(cancel_after: impl Into<CreateIssuingCardLifecycleControlsCancelAfter>) -> Self {
        Self { cancel_after: cancel_after.into() }
    }
}
/// Cancels the card after the specified conditions are met.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingCardLifecycleControlsCancelAfter {
    /// The card is automatically cancelled when it makes this number of non-zero payment authorizations and transactions.
    /// The count includes penny authorizations, but doesn't include non-payment actions, such as authorization advice.
    pub payment_count: u64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingCardLifecycleControlsCancelAfter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingCardLifecycleControlsCancelAfter").finish_non_exhaustive()
    }
}
impl CreateIssuingCardLifecycleControlsCancelAfter {
    pub fn new(payment_count: impl Into<u64>) -> Self {
        Self { payment_count: payment_count.into() }
    }
}
/// If `replacement_for` is specified, this should indicate why that card is being replaced.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateIssuingCardReplacementReason {
    Damaged,
    Expired,
    Lost,
    Stolen,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateIssuingCardReplacementReason {
    pub fn as_str(&self) -> &str {
        use CreateIssuingCardReplacementReason::*;
        match self {
            Damaged => "damaged",
            Expired => "expired",
            Lost => "lost",
            Stolen => "stolen",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateIssuingCardReplacementReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingCardReplacementReason::*;
        match s {
            "damaged" => Ok(Damaged),
            "expired" => Ok(Expired),
            "lost" => Ok(Lost),
            "stolen" => Ok(Stolen),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateIssuingCardReplacementReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateIssuingCardReplacementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateIssuingCardReplacementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingCardReplacementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateIssuingCardReplacementReason)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateIssuingCardReplacementReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateIssuingCardReplacementReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Creates an Issuing `Card` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingCard {
    inner: CreateIssuingCardBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingCard").finish_non_exhaustive()
    }
}
impl CreateIssuingCard {
    /// Construct a new `CreateIssuingCard`.
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        type_: impl Into<stripe_shared::IssuingCardType>,
    ) -> Self {
        Self { inner: CreateIssuingCardBuilder::new(currency.into(), type_.into()) }
    }
    /// The [Cardholder](https://docs.stripe.com/api#issuing_cardholder_object) object with which the card will be associated.
    pub fn cardholder(mut self, cardholder: impl Into<String>) -> Self {
        self.inner.cardholder = Some(cardholder.into());
        self
    }
    /// The desired expiration month (1-12) for this card if [specifying a custom expiration date](/issuing/cards/virtual/issue-cards?testing-method=with-code#exp-dates).
    pub fn exp_month(mut self, exp_month: impl Into<i64>) -> Self {
        self.inner.exp_month = Some(exp_month.into());
        self
    }
    /// The desired 4-digit expiration year for this card if [specifying a custom expiration date](/issuing/cards/virtual/issue-cards?testing-method=with-code#exp-dates).
    pub fn exp_year(mut self, exp_year: impl Into<i64>) -> Self {
        self.inner.exp_year = Some(exp_year.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The new financial account ID the card will be associated with.
    /// This field allows a card to be reassigned to a different financial account.
    pub fn financial_account(mut self, financial_account: impl Into<String>) -> Self {
        self.inner.financial_account = Some(financial_account.into());
        self
    }
    /// Rules that control the lifecycle of this card, such as automatic cancellation.
    /// Refer to our [documentation](/issuing/controls/lifecycle-controls) for more details.
    pub fn lifecycle_controls(
        mut self,
        lifecycle_controls: impl Into<CreateIssuingCardLifecycleControls>,
    ) -> Self {
        self.inner.lifecycle_controls = Some(lifecycle_controls.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
    /// The personalization design object belonging to this card.
    pub fn personalization_design(mut self, personalization_design: impl Into<String>) -> Self {
        self.inner.personalization_design = Some(personalization_design.into());
        self
    }
    /// The desired PIN for this card.
    pub fn pin(mut self, pin: impl Into<EncryptedPinParam>) -> Self {
        self.inner.pin = Some(pin.into());
        self
    }
    /// The card this is meant to be a replacement for (if any).
    pub fn replacement_for(mut self, replacement_for: impl Into<String>) -> Self {
        self.inner.replacement_for = Some(replacement_for.into());
        self
    }
    /// If `replacement_for` is specified, this should indicate why that card is being replaced.
    pub fn replacement_reason(
        mut self,
        replacement_reason: impl Into<CreateIssuingCardReplacementReason>,
    ) -> Self {
        self.inner.replacement_reason = Some(replacement_reason.into());
        self
    }
    /// The second line to print on the card. Max length: 24 characters.
    pub fn second_line(mut self, second_line: impl Into<String>) -> Self {
        self.inner.second_line = Some(second_line.into());
        self
    }
    /// The address where the card will be shipped.
    pub fn shipping(mut self, shipping: impl Into<ShippingSpecs>) -> Self {
        self.inner.shipping = Some(shipping.into());
        self
    }
    /// Rules that control spending for this card.
    /// Refer to our [documentation](https://docs.stripe.com/issuing/controls/spending-controls) for more details.
    pub fn spending_controls(
        mut self,
        spending_controls: impl Into<AuthorizationControlsParam>,
    ) -> Self {
        self.inner.spending_controls = Some(spending_controls.into());
        self
    }
    /// Whether authorizations can be approved on this card.
    /// May be blocked from activating cards depending on past-due Cardholder requirements.
    /// Defaults to `inactive`.
    pub fn status(mut self, status: impl Into<stripe_shared::BillingMeterStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl CreateIssuingCard {
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

impl StripeRequest for CreateIssuingCard {
    type Output = stripe_shared::IssuingCard;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/issuing/cards").form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateIssuingCardBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    cancellation_reason: Option<UpdateIssuingCardCancellationReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    personalization_design: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pin: Option<EncryptedPinParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping: Option<ShippingSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spending_controls: Option<AuthorizationControlsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::IssuingCardStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingCardBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingCardBuilder").finish_non_exhaustive()
    }
}
impl UpdateIssuingCardBuilder {
    fn new() -> Self {
        Self {
            cancellation_reason: None,
            expand: None,
            metadata: None,
            personalization_design: None,
            pin: None,
            shipping: None,
            spending_controls: None,
            status: None,
        }
    }
}
/// Reason why the `status` of this card is `canceled`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateIssuingCardCancellationReason {
    Lost,
    Stolen,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateIssuingCardCancellationReason {
    pub fn as_str(&self) -> &str {
        use UpdateIssuingCardCancellationReason::*;
        match self {
            Lost => "lost",
            Stolen => "stolen",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateIssuingCardCancellationReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIssuingCardCancellationReason::*;
        match s {
            "lost" => Ok(Lost),
            "stolen" => Ok(Stolen),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateIssuingCardCancellationReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateIssuingCardCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UpdateIssuingCardCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingCardCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UpdateIssuingCardCancellationReason)).finish_non_exhaustive()
    }
}
impl serde::Serialize for UpdateIssuingCardCancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateIssuingCardCancellationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Updates the specified Issuing `Card` object by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateIssuingCard {
    inner: UpdateIssuingCardBuilder,
    card: stripe_shared::IssuingCardId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingCard").finish_non_exhaustive()
    }
}
impl UpdateIssuingCard {
    /// Construct a new `UpdateIssuingCard`.
    pub fn new(card: impl Into<stripe_shared::IssuingCardId>) -> Self {
        Self { card: card.into(), inner: UpdateIssuingCardBuilder::new() }
    }
    /// Reason why the `status` of this card is `canceled`.
    pub fn cancellation_reason(
        mut self,
        cancellation_reason: impl Into<UpdateIssuingCardCancellationReason>,
    ) -> Self {
        self.inner.cancellation_reason = Some(cancellation_reason.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
    pub fn personalization_design(mut self, personalization_design: impl Into<String>) -> Self {
        self.inner.personalization_design = Some(personalization_design.into());
        self
    }
    /// The desired new PIN for this card.
    pub fn pin(mut self, pin: impl Into<EncryptedPinParam>) -> Self {
        self.inner.pin = Some(pin.into());
        self
    }
    /// Updated shipping information for the card.
    pub fn shipping(mut self, shipping: impl Into<ShippingSpecs>) -> Self {
        self.inner.shipping = Some(shipping.into());
        self
    }
    /// Rules that control spending for this card.
    /// Refer to our [documentation](https://docs.stripe.com/issuing/controls/spending-controls) for more details.
    pub fn spending_controls(
        mut self,
        spending_controls: impl Into<AuthorizationControlsParam>,
    ) -> Self {
        self.inner.spending_controls = Some(spending_controls.into());
        self
    }
    /// Dictates whether authorizations can be approved on this card.
    /// May be blocked from activating cards depending on past-due Cardholder requirements.
    /// Defaults to `inactive`.
    /// If this card is being canceled because it was lost or stolen, this information should be provided as `cancellation_reason`.
    pub fn status(mut self, status: impl Into<stripe_shared::IssuingCardStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl UpdateIssuingCard {
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

impl StripeRequest for UpdateIssuingCard {
    type Output = stripe_shared::IssuingCard;

    fn build(&self) -> RequestBuilder {
        let card = &self.card;
        RequestBuilder::new(StripeMethod::Post, format!("/issuing/cards/{card}")).form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct DeliverCardIssuingCardBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeliverCardIssuingCardBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeliverCardIssuingCardBuilder").finish_non_exhaustive()
    }
}
impl DeliverCardIssuingCardBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Updates the shipping status of the specified Issuing `Card` object to `delivered`.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeliverCardIssuingCard {
    inner: DeliverCardIssuingCardBuilder,
    card: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeliverCardIssuingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeliverCardIssuingCard").finish_non_exhaustive()
    }
}
impl DeliverCardIssuingCard {
    /// Construct a new `DeliverCardIssuingCard`.
    pub fn new(card: impl Into<String>) -> Self {
        Self { card: card.into(), inner: DeliverCardIssuingCardBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl DeliverCardIssuingCard {
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

impl StripeRequest for DeliverCardIssuingCard {
    type Output = stripe_shared::IssuingCard;

    fn build(&self) -> RequestBuilder {
        let card = &self.card;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/issuing/cards/{card}/shipping/deliver"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct FailCardIssuingCardBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FailCardIssuingCardBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FailCardIssuingCardBuilder").finish_non_exhaustive()
    }
}
impl FailCardIssuingCardBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Updates the shipping status of the specified Issuing `Card` object to `failure`.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FailCardIssuingCard {
    inner: FailCardIssuingCardBuilder,
    card: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FailCardIssuingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FailCardIssuingCard").finish_non_exhaustive()
    }
}
impl FailCardIssuingCard {
    /// Construct a new `FailCardIssuingCard`.
    pub fn new(card: impl Into<String>) -> Self {
        Self { card: card.into(), inner: FailCardIssuingCardBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl FailCardIssuingCard {
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

impl StripeRequest for FailCardIssuingCard {
    type Output = stripe_shared::IssuingCard;

    fn build(&self) -> RequestBuilder {
        let card = &self.card;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/issuing/cards/{card}/shipping/fail"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ReturnCardIssuingCardBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReturnCardIssuingCardBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReturnCardIssuingCardBuilder").finish_non_exhaustive()
    }
}
impl ReturnCardIssuingCardBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Updates the shipping status of the specified Issuing `Card` object to `returned`.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ReturnCardIssuingCard {
    inner: ReturnCardIssuingCardBuilder,
    card: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReturnCardIssuingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReturnCardIssuingCard").finish_non_exhaustive()
    }
}
impl ReturnCardIssuingCard {
    /// Construct a new `ReturnCardIssuingCard`.
    pub fn new(card: impl Into<String>) -> Self {
        Self { card: card.into(), inner: ReturnCardIssuingCardBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ReturnCardIssuingCard {
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

impl StripeRequest for ReturnCardIssuingCard {
    type Output = stripe_shared::IssuingCard;

    fn build(&self) -> RequestBuilder {
        let card = &self.card;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/issuing/cards/{card}/shipping/return"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ShipCardIssuingCardBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShipCardIssuingCardBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShipCardIssuingCardBuilder").finish_non_exhaustive()
    }
}
impl ShipCardIssuingCardBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Updates the shipping status of the specified Issuing `Card` object to `shipped`.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ShipCardIssuingCard {
    inner: ShipCardIssuingCardBuilder,
    card: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShipCardIssuingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShipCardIssuingCard").finish_non_exhaustive()
    }
}
impl ShipCardIssuingCard {
    /// Construct a new `ShipCardIssuingCard`.
    pub fn new(card: impl Into<String>) -> Self {
        Self { card: card.into(), inner: ShipCardIssuingCardBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ShipCardIssuingCard {
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

impl StripeRequest for ShipCardIssuingCard {
    type Output = stripe_shared::IssuingCard;

    fn build(&self) -> RequestBuilder {
        let card = &self.card;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/issuing/cards/{card}/shipping/ship"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct SubmitCardIssuingCardBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubmitCardIssuingCardBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubmitCardIssuingCardBuilder").finish_non_exhaustive()
    }
}
impl SubmitCardIssuingCardBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Updates the shipping status of the specified Issuing `Card` object to `submitted`.
/// This method requires Stripe Version ‘2024-09-30.acacia’ or later.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SubmitCardIssuingCard {
    inner: SubmitCardIssuingCardBuilder,
    card: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubmitCardIssuingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubmitCardIssuingCard").finish_non_exhaustive()
    }
}
impl SubmitCardIssuingCard {
    /// Construct a new `SubmitCardIssuingCard`.
    pub fn new(card: impl Into<String>) -> Self {
        Self { card: card.into(), inner: SubmitCardIssuingCardBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl SubmitCardIssuingCard {
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

impl StripeRequest for SubmitCardIssuingCard {
    type Output = stripe_shared::IssuingCard;

    fn build(&self) -> RequestBuilder {
        let card = &self.card;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/issuing/cards/{card}/shipping/submit"),
        )
        .form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct EncryptedPinParam {
    /// The card's desired new PIN, encrypted under Stripe's public key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for EncryptedPinParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("EncryptedPinParam").finish_non_exhaustive()
    }
}
impl EncryptedPinParam {
    pub fn new() -> Self {
        Self { encrypted_number: None }
    }
}
impl Default for EncryptedPinParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RequiredAddress {
    /// City, district, suburb, town, or village.
    pub city: String,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Address line 1, such as the street, PO Box, or company name.
    pub line1: String,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    pub postal_code: String,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RequiredAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RequiredAddress").finish_non_exhaustive()
    }
}
impl RequiredAddress {
    pub fn new(
        city: impl Into<String>,
        country: impl Into<String>,
        line1: impl Into<String>,
        postal_code: impl Into<String>,
    ) -> Self {
        Self {
            city: city.into(),
            country: country.into(),
            line1: line1.into(),
            line2: None,
            postal_code: postal_code.into(),
            state: None,
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct AddressValidationParam {
    /// The address validation capabilities to use.
    pub mode: stripe_shared::IssuingCardShippingAddressValidationMode,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AddressValidationParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AddressValidationParam").finish_non_exhaustive()
    }
}
impl AddressValidationParam {
    pub fn new(mode: impl Into<stripe_shared::IssuingCardShippingAddressValidationMode>) -> Self {
        Self { mode: mode.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CustomsParam {
    /// The Economic Operators Registration and Identification (EORI) number to use for Customs.
    /// Required for bulk shipments to Europe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eori_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomsParam").finish_non_exhaustive()
    }
}
impl CustomsParam {
    pub fn new() -> Self {
        Self { eori_number: None }
    }
}
impl Default for CustomsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SpendingLimitsParam {
    /// Maximum amount allowed to spend per interval.
    pub amount: i64,
    /// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) this limit applies to.
    /// Omitting this field will apply the limit to all categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<stripe_shared::IssuingCardSpendingLimitCategories>>,
    /// Interval (or event) to which the amount applies.
    pub interval: stripe_shared::IssuingCardSpendingLimitInterval,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SpendingLimitsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SpendingLimitsParam").finish_non_exhaustive()
    }
}
impl SpendingLimitsParam {
    pub fn new(
        amount: impl Into<i64>,
        interval: impl Into<stripe_shared::IssuingCardSpendingLimitInterval>,
    ) -> Self {
        Self { amount: amount.into(), categories: None, interval: interval.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ShippingSpecs {
    /// The address that the card is shipped to.
    pub address: RequiredAddress,
    /// Address validation settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_validation: Option<AddressValidationParam>,
    /// The name of the business at the shipping address, used on the shipping label to ensure delivery when the card is shipped to a cardholder's workplace.
    /// Allowed characters: `A-Z`, `a-z`, `0-9`, ` `, `.`, `-`.
    /// All other characters are stripped or ASCII-normalized when printed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// Customs information for the shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customs: Option<CustomsParam>,
    /// The name printed on the shipping label when shipping the card.
    pub name: String,
    /// Phone number of the recipient of the shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Whether a signature is required for card delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_signature: Option<bool>,
    /// Shipment service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<stripe_shared::Service>,
    /// Packaging options.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<stripe_shared::IssuingCardShippingType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShippingSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShippingSpecs").finish_non_exhaustive()
    }
}
impl ShippingSpecs {
    pub fn new(address: impl Into<RequiredAddress>, name: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            address_validation: None,
            business_name: None,
            customs: None,
            name: name.into(),
            phone_number: None,
            require_signature: None,
            service: None,
            type_: None,
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct AuthorizationControlsParam {
    /// Array of card presence statuses from which authorizations will be allowed.
    /// Possible options are `present`, `not_present`.
    /// All other statuses will be blocked.
    /// Cannot be set with `blocked_card_presences`.
    /// Provide an empty value to unset this control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_card_presences: Option<Vec<stripe_shared::IssuingAuthorizationCardPresence>>,
    /// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
    /// All other categories will be blocked.
    /// Cannot be set with `blocked_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<stripe_shared::IssuingCardSpendingLimitCategories>>,
    /// Array of strings containing representing countries from which authorizations will be allowed.
    /// Authorizations from merchants in all other countries will be declined.
    /// Country codes should be ISO 3166 alpha-2 country codes (e.g.
    /// `US`).
    /// Cannot be set with `blocked_merchant_countries`.
    /// Provide an empty value to unset this control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_merchant_countries: Option<Vec<String>>,
    /// Array of card presence statuses from which authorizations will be declined.
    /// Possible options are `present`, `not_present`.
    /// Cannot be set with `allowed_card_presences`.
    /// Provide an empty value to unset this control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_card_presences: Option<Vec<stripe_shared::IssuingAuthorizationCardPresence>>,
    /// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
    /// All other categories will be allowed.
    /// Cannot be set with `allowed_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<stripe_shared::IssuingCardSpendingLimitCategories>>,
    /// Array of strings containing representing countries from which authorizations will be declined.
    /// Country codes should be ISO 3166 alpha-2 country codes (e.g.
    /// `US`).
    /// Cannot be set with `allowed_merchant_countries`.
    /// Provide an empty value to unset this control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_merchant_countries: Option<Vec<String>>,
    /// Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<SpendingLimitsParam>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AuthorizationControlsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AuthorizationControlsParam").finish_non_exhaustive()
    }
}
impl AuthorizationControlsParam {
    pub fn new() -> Self {
        Self {
            allowed_card_presences: None,
            allowed_categories: None,
            allowed_merchant_countries: None,
            blocked_card_presences: None,
            blocked_categories: None,
            blocked_merchant_countries: None,
            spending_limits: None,
        }
    }
}
impl Default for AuthorizationControlsParam {
    fn default() -> Self {
        Self::new()
    }
}
/// Only return cards that have the given status. One of `active`, `inactive`, or `canceled`.
pub type ListIssuingCardStatus = stripe_shared::IssuingCardStatus;
/// Only return cards that have the given type. One of `virtual` or `physical`.
pub type ListIssuingCardType = stripe_shared::IssuingCardType;
/// The address validation capabilities to use.
pub type CreateIssuingCardShippingAddressValidationMode =
    stripe_shared::IssuingCardShippingAddressValidationMode;
/// Shipment service.
pub type CreateIssuingCardShippingService = stripe_shared::Service;
/// Packaging options.
pub type CreateIssuingCardShippingType = stripe_shared::IssuingCardShippingType;
/// Array of card presence statuses from which authorizations will be allowed.
/// Possible options are `present`, `not_present`.
/// All other statuses will be blocked.
/// Cannot be set with `blocked_card_presences`.
/// Provide an empty value to unset this control.
pub type CreateIssuingCardSpendingControlsAllowedCardPresences =
    stripe_shared::IssuingAuthorizationCardPresence;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
/// All other categories will be blocked.
/// Cannot be set with `blocked_categories`.
pub type CreateIssuingCardSpendingControlsAllowedCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Array of card presence statuses from which authorizations will be declined.
/// Possible options are `present`, `not_present`.
/// Cannot be set with `allowed_card_presences`.
/// Provide an empty value to unset this control.
pub type CreateIssuingCardSpendingControlsBlockedCardPresences =
    stripe_shared::IssuingAuthorizationCardPresence;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
/// All other categories will be allowed.
/// Cannot be set with `allowed_categories`.
pub type CreateIssuingCardSpendingControlsBlockedCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) this limit applies to.
/// Omitting this field will apply the limit to all categories.
pub type CreateIssuingCardSpendingControlsSpendingLimitsCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Interval (or event) to which the amount applies.
pub type CreateIssuingCardSpendingControlsSpendingLimitsInterval =
    stripe_shared::IssuingCardSpendingLimitInterval;
/// Whether authorizations can be approved on this card.
/// May be blocked from activating cards depending on past-due Cardholder requirements.
/// Defaults to `inactive`.
pub type CreateIssuingCardStatus = stripe_shared::BillingMeterStatus;
/// The type of card to issue. Possible values are `physical` or `virtual`.
pub type CreateIssuingCardType = stripe_shared::IssuingCardType;
/// The address validation capabilities to use.
pub type UpdateIssuingCardShippingAddressValidationMode =
    stripe_shared::IssuingCardShippingAddressValidationMode;
/// Shipment service.
pub type UpdateIssuingCardShippingService = stripe_shared::Service;
/// Packaging options.
pub type UpdateIssuingCardShippingType = stripe_shared::IssuingCardShippingType;
/// Array of card presence statuses from which authorizations will be allowed.
/// Possible options are `present`, `not_present`.
/// All other statuses will be blocked.
/// Cannot be set with `blocked_card_presences`.
/// Provide an empty value to unset this control.
pub type UpdateIssuingCardSpendingControlsAllowedCardPresences =
    stripe_shared::IssuingAuthorizationCardPresence;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
/// All other categories will be blocked.
/// Cannot be set with `blocked_categories`.
pub type UpdateIssuingCardSpendingControlsAllowedCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Array of card presence statuses from which authorizations will be declined.
/// Possible options are `present`, `not_present`.
/// Cannot be set with `allowed_card_presences`.
/// Provide an empty value to unset this control.
pub type UpdateIssuingCardSpendingControlsBlockedCardPresences =
    stripe_shared::IssuingAuthorizationCardPresence;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
/// All other categories will be allowed.
/// Cannot be set with `allowed_categories`.
pub type UpdateIssuingCardSpendingControlsBlockedCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) this limit applies to.
/// Omitting this field will apply the limit to all categories.
pub type UpdateIssuingCardSpendingControlsSpendingLimitsCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Interval (or event) to which the amount applies.
pub type UpdateIssuingCardSpendingControlsSpendingLimitsInterval =
    stripe_shared::IssuingCardSpendingLimitInterval;
/// Dictates whether authorizations can be approved on this card.
/// May be blocked from activating cards depending on past-due Cardholder requirements.
/// Defaults to `inactive`.
/// If this card is being canceled because it was lost or stolen, this information should be provided as `cancellation_reason`.
pub type UpdateIssuingCardStatus = stripe_shared::IssuingCardStatus;
/// The desired PIN for this card.
pub type CreateIssuingCardPin = EncryptedPinParam;
/// The address that the card is shipped to.
pub type CreateIssuingCardShippingAddress = RequiredAddress;
/// Address validation settings.
pub type CreateIssuingCardShippingAddressValidation = AddressValidationParam;
/// Customs information for the shipment.
pub type CreateIssuingCardShippingCustoms = CustomsParam;
/// Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
pub type CreateIssuingCardSpendingControlsSpendingLimits = SpendingLimitsParam;
/// The desired new PIN for this card.
pub type UpdateIssuingCardPin = EncryptedPinParam;
/// The address that the card is shipped to.
pub type UpdateIssuingCardShippingAddress = RequiredAddress;
/// Address validation settings.
pub type UpdateIssuingCardShippingAddressValidation = AddressValidationParam;
/// Customs information for the shipment.
pub type UpdateIssuingCardShippingCustoms = CustomsParam;
/// Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
pub type UpdateIssuingCardSpendingControlsSpendingLimits = SpendingLimitsParam;
/// The address where the card will be shipped.
pub type CreateIssuingCardShipping = ShippingSpecs;
/// Rules that control spending for this card.
/// Refer to our [documentation](https://docs.stripe.com/issuing/controls/spending-controls) for more details.
pub type CreateIssuingCardSpendingControls = AuthorizationControlsParam;
/// Updated shipping information for the card.
pub type UpdateIssuingCardShipping = ShippingSpecs;
/// Rules that control spending for this card.
/// Refer to our [documentation](https://docs.stripe.com/issuing/controls/spending-controls) for more details.
pub type UpdateIssuingCardSpendingControls = AuthorizationControlsParam;
