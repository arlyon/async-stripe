use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct DetachSourceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl DetachSourceBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Delete a specified source for a given customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DetachSource {
    inner: DetachSourceBuilder,
    customer: stripe_shared::CustomerId,
    id: String,
}
impl DetachSource {
    /// Construct a new `DetachSource`.
    pub fn new(customer: impl Into<stripe_shared::CustomerId>, id: impl Into<String>) -> Self {
        Self { customer: customer.into(), id: id.into(), inner: DetachSourceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl DetachSource {
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

impl StripeRequest for DetachSource {
    type Output = DetachSourceReturned;

    fn build(&self) -> RequestBuilder {
        let customer = &self.customer;
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Delete, format!("/customers/{customer}/sources/{id}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(untagged))]
pub enum DetachSourceReturned {
    PaymentSource(stripe_shared::PaymentSource),
    DeletedPaymentSource(stripe_shared::DeletedPaymentSource),
}

#[derive(Default)]
pub struct DetachSourceReturnedBuilder {
    inner: stripe_types::miniserde_helpers::MaybeDeletedBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::MapBuilder;
    use stripe_types::miniserde_helpers::FromValueOpt;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<DetachSourceReturned>,
        builder: DetachSourceReturnedBuilder,
    }

    impl Deserialize for DetachSourceReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<DetachSourceReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for DetachSourceReturnedBuilder {
        type Out = DetachSourceReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (deleted, o) = self.inner.finish_inner()?;
            Some(if deleted {
                DetachSourceReturned::DeletedPaymentSource(FromValueOpt::from_value(
                    Value::Object(o),
                )?)
            } else {
                DetachSourceReturned::PaymentSource(FromValueOpt::from_value(Value::Object(o))?)
            })
        }
    }

    impl stripe_types::ObjectDeser for DetachSourceReturned {
        type Builder = DetachSourceReturnedBuilder;
    }
};

#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveSourceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveSourceBuilder {
    fn new() -> Self {
        Self { client_secret: None, expand: None }
    }
}
/// Retrieves an existing source object.
/// Supply the unique source ID from a source creation request and Stripe will return the corresponding up-to-date source object information.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveSource {
    inner: RetrieveSourceBuilder,
    source: stripe_shared::SourceId,
}
impl RetrieveSource {
    /// Construct a new `RetrieveSource`.
    pub fn new(source: impl Into<stripe_shared::SourceId>) -> Self {
        Self { source: source.into(), inner: RetrieveSourceBuilder::new() }
    }
    /// The client secret of the source. Required if a publishable key is used to retrieve the source.
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.inner.client_secret = Some(client_secret.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveSource {
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

impl StripeRequest for RetrieveSource {
    type Output = stripe_shared::Source;

    fn build(&self) -> RequestBuilder {
        let source = &self.source;
        RequestBuilder::new(StripeMethod::Get, format!("/sources/{source}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct SourceTransactionsSourceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl SourceTransactionsSourceBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// List source transactions for a given source.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SourceTransactionsSource {
    inner: SourceTransactionsSourceBuilder,
    source: stripe_shared::SourceId,
}
impl SourceTransactionsSource {
    /// Construct a new `SourceTransactionsSource`.
    pub fn new(source: impl Into<stripe_shared::SourceId>) -> Self {
        Self { source: source.into(), inner: SourceTransactionsSourceBuilder::new() }
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
impl SourceTransactionsSource {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::SourceTransaction>>
    {
        let source = &self.source;

        stripe_client_core::ListPaginator::new_list(
            format!("/sources/{source}/source_transactions"),
            &self.inner,
        )
    }
}

impl StripeRequest for SourceTransactionsSource {
    type Output = stripe_types::List<stripe_shared::SourceTransaction>;

    fn build(&self) -> RequestBuilder {
        let source = &self.source;
        RequestBuilder::new(StripeMethod::Get, format!("/sources/{source}/source_transactions"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateSourceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow: Option<CreateSourceFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mandate: Option<CreateSourceMandate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<Owner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver: Option<CreateSourceReceiver>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<CreateSourceRedirect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_order: Option<CreateSourceSourceOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<CreateSourceUsage>,
}
impl CreateSourceBuilder {
    fn new() -> Self {
        Self {
            amount: None,
            currency: None,
            customer: None,
            expand: None,
            flow: None,
            mandate: None,
            metadata: None,
            original_source: None,
            owner: None,
            receiver: None,
            redirect: None,
            source_order: None,
            statement_descriptor: None,
            token: None,
            type_: None,
            usage: None,
        }
    }
}
/// The authentication `flow` of the source to create.
/// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
/// It is generally inferred unless a type supports multiple flows.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSourceFlow {
    CodeVerification,
    None,
    Receiver,
    Redirect,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSourceFlow {
    pub fn as_str(&self) -> &str {
        use CreateSourceFlow::*;
        match self {
            CodeVerification => "code_verification",
            None => "none",
            Receiver => "receiver",
            Redirect => "redirect",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSourceFlow {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceFlow::*;
        match s {
            "code_verification" => Ok(CodeVerification),
            "none" => Ok(None),
            "receiver" => Ok(Receiver),
            "redirect" => Ok(Redirect),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreateSourceFlow");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSourceFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSourceFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSourceFlow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSourceFlow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSourceMandate {
    /// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<CreateSourceMandateAcceptance>,
    /// The amount specified by the mandate. (Leave null for a mandate covering all amounts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The currency specified by the mandate. (Must match `currency` of the source)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The interval of debits permitted by the mandate.
    /// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CreateSourceMandateInterval>,
    /// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
    /// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_method: Option<CreateSourceMandateNotificationMethod>,
}
impl CreateSourceMandate {
    pub fn new() -> Self {
        Self {
            acceptance: None,
            amount: None,
            currency: None,
            interval: None,
            notification_method: None,
        }
    }
}
impl Default for CreateSourceMandate {
    fn default() -> Self {
        Self::new()
    }
}
/// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSourceMandateAcceptance {
    /// The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The parameters required to store a mandate accepted offline.
    /// Should only be set if `mandate[type]` is `offline`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<MandateOfflineAcceptanceParams>,
    /// The parameters required to store a mandate accepted online.
    /// Should only be set if `mandate[type]` is `online`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<MandateOnlineAcceptanceParams>,
    /// The status of the mandate acceptance.
    /// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
    pub status: CreateSourceMandateAcceptanceStatus,
    /// The type of acceptance information included with the mandate. Either `online` or `offline`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateSourceMandateAcceptanceType>,
    /// The user agent of the browser from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl CreateSourceMandateAcceptance {
    pub fn new(status: impl Into<CreateSourceMandateAcceptanceStatus>) -> Self {
        Self {
            date: None,
            ip: None,
            offline: None,
            online: None,
            status: status.into(),
            type_: None,
            user_agent: None,
        }
    }
}
/// The status of the mandate acceptance.
/// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSourceMandateAcceptanceStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSourceMandateAcceptanceStatus {
    pub fn as_str(&self) -> &str {
        use CreateSourceMandateAcceptanceStatus::*;
        match self {
            Accepted => "accepted",
            Pending => "pending",
            Refused => "refused",
            Revoked => "revoked",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSourceMandateAcceptanceStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceMandateAcceptanceStatus::*;
        match s {
            "accepted" => Ok(Accepted),
            "pending" => Ok(Pending),
            "refused" => Ok(Refused),
            "revoked" => Ok(Revoked),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSourceMandateAcceptanceStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSourceMandateAcceptanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSourceMandateAcceptanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSourceMandateAcceptanceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSourceMandateAcceptanceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The type of acceptance information included with the mandate. Either `online` or `offline`
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSourceMandateAcceptanceType {
    Offline,
    Online,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSourceMandateAcceptanceType {
    pub fn as_str(&self) -> &str {
        use CreateSourceMandateAcceptanceType::*;
        match self {
            Offline => "offline",
            Online => "online",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSourceMandateAcceptanceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceMandateAcceptanceType::*;
        match s {
            "offline" => Ok(Offline),
            "online" => Ok(Online),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSourceMandateAcceptanceType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSourceMandateAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSourceMandateAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSourceMandateAcceptanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSourceMandateAcceptanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The interval of debits permitted by the mandate.
/// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSourceMandateInterval {
    OneTime,
    Scheduled,
    Variable,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSourceMandateInterval {
    pub fn as_str(&self) -> &str {
        use CreateSourceMandateInterval::*;
        match self {
            OneTime => "one_time",
            Scheduled => "scheduled",
            Variable => "variable",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSourceMandateInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceMandateInterval::*;
        match s {
            "one_time" => Ok(OneTime),
            "scheduled" => Ok(Scheduled),
            "variable" => Ok(Variable),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSourceMandateInterval"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSourceMandateInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSourceMandateInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSourceMandateInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSourceMandateInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
/// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSourceMandateNotificationMethod {
    DeprecatedNone,
    Email,
    Manual,
    None,
    StripeEmail,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSourceMandateNotificationMethod {
    pub fn as_str(&self) -> &str {
        use CreateSourceMandateNotificationMethod::*;
        match self {
            DeprecatedNone => "deprecated_none",
            Email => "email",
            Manual => "manual",
            None => "none",
            StripeEmail => "stripe_email",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSourceMandateNotificationMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceMandateNotificationMethod::*;
        match s {
            "deprecated_none" => Ok(DeprecatedNone),
            "email" => Ok(Email),
            "manual" => Ok(Manual),
            "none" => Ok(None),
            "stripe_email" => Ok(StripeEmail),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSourceMandateNotificationMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSourceMandateNotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSourceMandateNotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSourceMandateNotificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSourceMandateNotificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Optional parameters for the receiver flow.
/// Can be set only if the source is a receiver (`flow` is `receiver`).
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSourceReceiver {
    /// The method Stripe should use to request information needed to process a refund or mispayment.
    /// Either `email` (an email is sent directly to the customer) or `manual` (a `source.refund_attributes_required` event is sent to your webhooks endpoint).
    /// Refer to each payment method's documentation to learn which refund attributes may be required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_attributes_method: Option<CreateSourceReceiverRefundAttributesMethod>,
}
impl CreateSourceReceiver {
    pub fn new() -> Self {
        Self { refund_attributes_method: None }
    }
}
impl Default for CreateSourceReceiver {
    fn default() -> Self {
        Self::new()
    }
}
/// The method Stripe should use to request information needed to process a refund or mispayment.
/// Either `email` (an email is sent directly to the customer) or `manual` (a `source.refund_attributes_required` event is sent to your webhooks endpoint).
/// Refer to each payment method's documentation to learn which refund attributes may be required.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSourceReceiverRefundAttributesMethod {
    Email,
    Manual,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSourceReceiverRefundAttributesMethod {
    pub fn as_str(&self) -> &str {
        use CreateSourceReceiverRefundAttributesMethod::*;
        match self {
            Email => "email",
            Manual => "manual",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSourceReceiverRefundAttributesMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceReceiverRefundAttributesMethod::*;
        match s {
            "email" => Ok(Email),
            "manual" => Ok(Manual),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSourceReceiverRefundAttributesMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSourceReceiverRefundAttributesMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSourceReceiverRefundAttributesMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSourceReceiverRefundAttributesMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSourceReceiverRefundAttributesMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Parameters required for the redirect flow.
/// Required if the source is authenticated by a redirect (`flow` is `redirect`).
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSourceRedirect {
    /// The URL you provide to redirect the customer back to you after they authenticated their payment.
    /// It can use your application URI scheme in the context of a mobile application.
    pub return_url: String,
}
impl CreateSourceRedirect {
    pub fn new(return_url: impl Into<String>) -> Self {
        Self { return_url: return_url.into() }
    }
}
/// Information about the items and shipping associated with the source.
/// Required for transactional credit (for example Klarna) sources before you can charge it.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSourceSourceOrder {
    /// List of items constituting the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreateSourceSourceOrderItems>>,
    /// Shipping address for the order.
    /// Required if any of the SKUs are for products that have `shippable` set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<OrderShipping>,
}
impl CreateSourceSourceOrder {
    pub fn new() -> Self {
        Self { items: None, shipping: None }
    }
}
impl Default for CreateSourceSourceOrder {
    fn default() -> Self {
        Self::new()
    }
}
/// List of items constituting the order.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSourceSourceOrderItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the SKU being ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// The quantity of this order item.
    /// When type is `sku`, this is the number of instances of the SKU to be ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateSourceSourceOrderItemsType>,
}
impl CreateSourceSourceOrderItems {
    pub fn new() -> Self {
        Self {
            amount: None,
            currency: None,
            description: None,
            parent: None,
            quantity: None,
            type_: None,
        }
    }
}
impl Default for CreateSourceSourceOrderItems {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSourceSourceOrderItemsType {
    Discount,
    Shipping,
    Sku,
    Tax,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSourceSourceOrderItemsType {
    pub fn as_str(&self) -> &str {
        use CreateSourceSourceOrderItemsType::*;
        match self {
            Discount => "discount",
            Shipping => "shipping",
            Sku => "sku",
            Tax => "tax",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSourceSourceOrderItemsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceSourceOrderItemsType::*;
        match s {
            "discount" => Ok(Discount),
            "shipping" => Ok(Shipping),
            "sku" => Ok(Sku),
            "tax" => Ok(Tax),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSourceSourceOrderItemsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSourceSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSourceSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSourceSourceOrderItemsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSourceSourceOrderItemsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSourceUsage {
    Reusable,
    SingleUse,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSourceUsage {
    pub fn as_str(&self) -> &str {
        use CreateSourceUsage::*;
        match self {
            Reusable => "reusable",
            SingleUse => "single_use",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSourceUsage {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceUsage::*;
        match s {
            "reusable" => Ok(Reusable),
            "single_use" => Ok(SingleUse),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreateSourceUsage");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSourceUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSourceUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSourceUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSourceUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Creates a new source object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSource {
    inner: CreateSourceBuilder,
}
impl CreateSource {
    /// Construct a new `CreateSource`.
    pub fn new() -> Self {
        Self { inner: CreateSourceBuilder::new() }
    }
    /// Amount associated with the source.
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    /// Not supported for `receiver` type sources, where charge amount may not be specified until funds land.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source.
    /// This is the currency for which the source will be chargeable once ready.
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// The `Customer` to whom the original source is attached to.
    /// Must be set when the original source is not a `Source` (e.g., `Card`).
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The authentication `flow` of the source to create.
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    /// It is generally inferred unless a type supports multiple flows.
    pub fn flow(mut self, flow: impl Into<CreateSourceFlow>) -> Self {
        self.inner.flow = Some(flow.into());
        self
    }
    /// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
    pub fn mandate(mut self, mandate: impl Into<CreateSourceMandate>) -> Self {
        self.inner.mandate = Some(mandate.into());
        self
    }
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The source to share.
    pub fn original_source(mut self, original_source: impl Into<String>) -> Self {
        self.inner.original_source = Some(original_source.into());
        self
    }
    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    pub fn owner(mut self, owner: impl Into<Owner>) -> Self {
        self.inner.owner = Some(owner.into());
        self
    }
    /// Optional parameters for the receiver flow.
    /// Can be set only if the source is a receiver (`flow` is `receiver`).
    pub fn receiver(mut self, receiver: impl Into<CreateSourceReceiver>) -> Self {
        self.inner.receiver = Some(receiver.into());
        self
    }
    /// Parameters required for the redirect flow.
    /// Required if the source is authenticated by a redirect (`flow` is `redirect`).
    pub fn redirect(mut self, redirect: impl Into<CreateSourceRedirect>) -> Self {
        self.inner.redirect = Some(redirect.into());
        self
    }
    /// Information about the items and shipping associated with the source.
    /// Required for transactional credit (for example Klarna) sources before you can charge it.
    pub fn source_order(mut self, source_order: impl Into<CreateSourceSourceOrder>) -> Self {
        self.inner.source_order = Some(source_order.into());
        self
    }
    /// An arbitrary string to be displayed on your customer's statement.
    /// As an example, if your website is `RunClub` and the item you're charging for is a race ticket, you may want to specify a `statement_descriptor` of `RunClub 5K race ticket.` While many payment types will display this information, some may not display it at all.
    pub fn statement_descriptor(mut self, statement_descriptor: impl Into<String>) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor.into());
        self
    }
    /// An optional token used to create the source.
    /// When passed, token properties will override source parameters.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.inner.token = Some(token.into());
        self
    }
    /// The `type` of the source to create.
    /// Required unless `customer` and `original_source` are specified (see the [Cloning card Sources](https://docs.stripe.com/sources/connect#cloning-card-sources) guide).
    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
    pub fn usage(mut self, usage: impl Into<CreateSourceUsage>) -> Self {
        self.inner.usage = Some(usage.into());
        self
    }
}
impl Default for CreateSource {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateSource {
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

impl StripeRequest for CreateSource {
    type Output = stripe_shared::Source;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/sources").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateSourceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mandate: Option<UpdateSourceMandate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<Owner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_order: Option<UpdateSourceSourceOrder>,
}
impl UpdateSourceBuilder {
    fn new() -> Self {
        Self {
            amount: None,
            expand: None,
            mandate: None,
            metadata: None,
            owner: None,
            source_order: None,
        }
    }
}
/// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSourceMandate {
    /// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<UpdateSourceMandateAcceptance>,
    /// The amount specified by the mandate. (Leave null for a mandate covering all amounts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The currency specified by the mandate. (Must match `currency` of the source)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The interval of debits permitted by the mandate.
    /// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<UpdateSourceMandateInterval>,
    /// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
    /// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_method: Option<UpdateSourceMandateNotificationMethod>,
}
impl UpdateSourceMandate {
    pub fn new() -> Self {
        Self {
            acceptance: None,
            amount: None,
            currency: None,
            interval: None,
            notification_method: None,
        }
    }
}
impl Default for UpdateSourceMandate {
    fn default() -> Self {
        Self::new()
    }
}
/// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSourceMandateAcceptance {
    /// The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The parameters required to store a mandate accepted offline.
    /// Should only be set if `mandate[type]` is `offline`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<MandateOfflineAcceptanceParams>,
    /// The parameters required to store a mandate accepted online.
    /// Should only be set if `mandate[type]` is `online`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<MandateOnlineAcceptanceParams>,
    /// The status of the mandate acceptance.
    /// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
    pub status: UpdateSourceMandateAcceptanceStatus,
    /// The type of acceptance information included with the mandate. Either `online` or `offline`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpdateSourceMandateAcceptanceType>,
    /// The user agent of the browser from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl UpdateSourceMandateAcceptance {
    pub fn new(status: impl Into<UpdateSourceMandateAcceptanceStatus>) -> Self {
        Self {
            date: None,
            ip: None,
            offline: None,
            online: None,
            status: status.into(),
            type_: None,
            user_agent: None,
        }
    }
}
/// The status of the mandate acceptance.
/// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateSourceMandateAcceptanceStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateSourceMandateAcceptanceStatus {
    pub fn as_str(&self) -> &str {
        use UpdateSourceMandateAcceptanceStatus::*;
        match self {
            Accepted => "accepted",
            Pending => "pending",
            Refused => "refused",
            Revoked => "revoked",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateSourceMandateAcceptanceStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSourceMandateAcceptanceStatus::*;
        match s {
            "accepted" => Ok(Accepted),
            "pending" => Ok(Pending),
            "refused" => Ok(Refused),
            "revoked" => Ok(Revoked),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateSourceMandateAcceptanceStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateSourceMandateAcceptanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSourceMandateAcceptanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSourceMandateAcceptanceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSourceMandateAcceptanceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The type of acceptance information included with the mandate. Either `online` or `offline`
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateSourceMandateAcceptanceType {
    Offline,
    Online,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateSourceMandateAcceptanceType {
    pub fn as_str(&self) -> &str {
        use UpdateSourceMandateAcceptanceType::*;
        match self {
            Offline => "offline",
            Online => "online",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateSourceMandateAcceptanceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSourceMandateAcceptanceType::*;
        match s {
            "offline" => Ok(Offline),
            "online" => Ok(Online),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateSourceMandateAcceptanceType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateSourceMandateAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSourceMandateAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSourceMandateAcceptanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSourceMandateAcceptanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The interval of debits permitted by the mandate.
/// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateSourceMandateInterval {
    OneTime,
    Scheduled,
    Variable,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateSourceMandateInterval {
    pub fn as_str(&self) -> &str {
        use UpdateSourceMandateInterval::*;
        match self {
            OneTime => "one_time",
            Scheduled => "scheduled",
            Variable => "variable",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateSourceMandateInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSourceMandateInterval::*;
        match s {
            "one_time" => Ok(OneTime),
            "scheduled" => Ok(Scheduled),
            "variable" => Ok(Variable),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateSourceMandateInterval"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateSourceMandateInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSourceMandateInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSourceMandateInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSourceMandateInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
/// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateSourceMandateNotificationMethod {
    DeprecatedNone,
    Email,
    Manual,
    None,
    StripeEmail,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateSourceMandateNotificationMethod {
    pub fn as_str(&self) -> &str {
        use UpdateSourceMandateNotificationMethod::*;
        match self {
            DeprecatedNone => "deprecated_none",
            Email => "email",
            Manual => "manual",
            None => "none",
            StripeEmail => "stripe_email",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateSourceMandateNotificationMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSourceMandateNotificationMethod::*;
        match s {
            "deprecated_none" => Ok(DeprecatedNone),
            "email" => Ok(Email),
            "manual" => Ok(Manual),
            "none" => Ok(None),
            "stripe_email" => Ok(StripeEmail),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateSourceMandateNotificationMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateSourceMandateNotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSourceMandateNotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSourceMandateNotificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSourceMandateNotificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Information about the items and shipping associated with the source.
/// Required for transactional credit (for example Klarna) sources before you can charge it.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSourceSourceOrder {
    /// List of items constituting the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UpdateSourceSourceOrderItems>>,
    /// Shipping address for the order.
    /// Required if any of the SKUs are for products that have `shippable` set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<OrderShipping>,
}
impl UpdateSourceSourceOrder {
    pub fn new() -> Self {
        Self { items: None, shipping: None }
    }
}
impl Default for UpdateSourceSourceOrder {
    fn default() -> Self {
        Self::new()
    }
}
/// List of items constituting the order.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSourceSourceOrderItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the SKU being ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// The quantity of this order item.
    /// When type is `sku`, this is the number of instances of the SKU to be ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpdateSourceSourceOrderItemsType>,
}
impl UpdateSourceSourceOrderItems {
    pub fn new() -> Self {
        Self {
            amount: None,
            currency: None,
            description: None,
            parent: None,
            quantity: None,
            type_: None,
        }
    }
}
impl Default for UpdateSourceSourceOrderItems {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateSourceSourceOrderItemsType {
    Discount,
    Shipping,
    Sku,
    Tax,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateSourceSourceOrderItemsType {
    pub fn as_str(&self) -> &str {
        use UpdateSourceSourceOrderItemsType::*;
        match self {
            Discount => "discount",
            Shipping => "shipping",
            Sku => "sku",
            Tax => "tax",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateSourceSourceOrderItemsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSourceSourceOrderItemsType::*;
        match s {
            "discount" => Ok(Discount),
            "shipping" => Ok(Shipping),
            "sku" => Ok(Sku),
            "tax" => Ok(Tax),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateSourceSourceOrderItemsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateSourceSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSourceSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSourceSourceOrderItemsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSourceSourceOrderItemsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Updates the specified source by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
///
/// This request accepts the `metadata` and `owner` as arguments.
/// It is also possible to update type specific information for selected payment methods.
/// Please refer to our [payment method guides](https://stripe.com/docs/sources) for more detail.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSource {
    inner: UpdateSourceBuilder,
    source: stripe_shared::SourceId,
}
impl UpdateSource {
    /// Construct a new `UpdateSource`.
    pub fn new(source: impl Into<stripe_shared::SourceId>) -> Self {
        Self { source: source.into(), inner: UpdateSourceBuilder::new() }
    }
    /// Amount associated with the source.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
    pub fn mandate(mut self, mandate: impl Into<UpdateSourceMandate>) -> Self {
        self.inner.mandate = Some(mandate.into());
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
    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    pub fn owner(mut self, owner: impl Into<Owner>) -> Self {
        self.inner.owner = Some(owner.into());
        self
    }
    /// Information about the items and shipping associated with the source.
    /// Required for transactional credit (for example Klarna) sources before you can charge it.
    pub fn source_order(mut self, source_order: impl Into<UpdateSourceSourceOrder>) -> Self {
        self.inner.source_order = Some(source_order.into());
        self
    }
}
impl UpdateSource {
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

impl StripeRequest for UpdateSource {
    type Output = stripe_shared::Source;

    fn build(&self) -> RequestBuilder {
        let source = &self.source;
        RequestBuilder::new(StripeMethod::Post, format!("/sources/{source}")).form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct VerifySourceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    values: Vec<String>,
}
impl VerifySourceBuilder {
    fn new(values: impl Into<Vec<String>>) -> Self {
        Self { expand: None, values: values.into() }
    }
}
/// Verify a given source.
#[derive(Clone, Debug, serde::Serialize)]
pub struct VerifySource {
    inner: VerifySourceBuilder,
    source: stripe_shared::SourceId,
}
impl VerifySource {
    /// Construct a new `VerifySource`.
    pub fn new(source: impl Into<stripe_shared::SourceId>, values: impl Into<Vec<String>>) -> Self {
        Self { source: source.into(), inner: VerifySourceBuilder::new(values.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl VerifySource {
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

impl StripeRequest for VerifySource {
    type Output = stripe_shared::Source;

    fn build(&self) -> RequestBuilder {
        let source = &self.source;
        RequestBuilder::new(StripeMethod::Post, format!("/sources/{source}/verify"))
            .form(&self.inner)
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct MandateOfflineAcceptanceParams {
    /// An email to contact you with if a copy of the mandate is requested, required if `type` is `offline`.
    pub contact_email: String,
}
impl MandateOfflineAcceptanceParams {
    pub fn new(contact_email: impl Into<String>) -> Self {
        Self { contact_email: contact_email.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct MandateOnlineAcceptanceParams {
    /// The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user agent of the browser from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl MandateOnlineAcceptanceParams {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl Default for MandateOnlineAcceptanceParams {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct SourceAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl SourceAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for SourceAddress {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct Address {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    pub line1: String,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl Address {
    pub fn new(line1: impl Into<String>) -> Self {
        Self {
            city: None,
            country: None,
            line1: line1.into(),
            line2: None,
            postal_code: None,
            state: None,
        }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct Owner {
    /// Owner's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<SourceAddress>,
    /// Owner's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Owner's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Owner's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl Owner {
    pub fn new() -> Self {
        Self { address: None, email: None, name: None, phone: None }
    }
}
impl Default for Owner {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct OrderShipping {
    /// Shipping address.
    pub address: Address,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// Recipient name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The tracking number for a physical product, obtained from the delivery service.
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}
impl OrderShipping {
    pub fn new(address: impl Into<Address>) -> Self {
        Self {
            address: address.into(),
            carrier: None,
            name: None,
            phone: None,
            tracking_number: None,
        }
    }
}
