use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct DetachSourceBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> DetachSourceBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Delete a specified source for a given customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DetachSource<'a> {
    inner: DetachSourceBuilder<'a>,
    customer: &'a stripe_shared::CustomerId,
    id: &'a str,
}
impl<'a> DetachSource<'a> {
    /// Construct a new `DetachSource`.
    pub fn new(customer: &'a stripe_shared::CustomerId, id: &'a str) -> Self {
        Self { customer, id, inner: DetachSourceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl DetachSource<'_> {
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

impl StripeRequest for DetachSource<'_> {
    type Output = DetachSourceReturned;

    fn build(&self) -> RequestBuilder {
        let customer = self.customer;
        let id = self.id;
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

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

    impl<'a> Map for Builder<'a> {
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

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveSourceBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveSourceBuilder<'a> {
    fn new() -> Self {
        Self { client_secret: None, expand: None }
    }
}
/// Retrieves an existing source object.
/// Supply the unique source ID from a source creation request and Stripe will return the corresponding up-to-date source object information.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveSource<'a> {
    inner: RetrieveSourceBuilder<'a>,
    source: &'a stripe_shared::SourceId,
}
impl<'a> RetrieveSource<'a> {
    /// Construct a new `RetrieveSource`.
    pub fn new(source: &'a stripe_shared::SourceId) -> Self {
        Self { source, inner: RetrieveSourceBuilder::new() }
    }
    /// The client secret of the source. Required if a publishable key is used to retrieve the source.
    pub fn client_secret(mut self, client_secret: &'a str) -> Self {
        self.inner.client_secret = Some(client_secret);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveSource<'_> {
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

impl StripeRequest for RetrieveSource<'_> {
    type Output = stripe_shared::Source;

    fn build(&self) -> RequestBuilder {
        let source = self.source;
        RequestBuilder::new(StripeMethod::Get, format!("/sources/{source}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct SourceTransactionsSourceBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> SourceTransactionsSourceBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// List source transactions for a given source.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SourceTransactionsSource<'a> {
    inner: SourceTransactionsSourceBuilder<'a>,
    source: &'a stripe_shared::SourceId,
}
impl<'a> SourceTransactionsSource<'a> {
    /// Construct a new `SourceTransactionsSource`.
    pub fn new(source: &'a stripe_shared::SourceId) -> Self {
        Self { source, inner: SourceTransactionsSourceBuilder::new() }
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
impl SourceTransactionsSource<'_> {
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
        let source = self.source;

        stripe_client_core::ListPaginator::new_list(
            format!("/sources/{source}/source_transactions"),
            self.inner,
        )
    }
}

impl StripeRequest for SourceTransactionsSource<'_> {
    type Output = stripe_types::List<stripe_shared::SourceTransaction>;

    fn build(&self) -> RequestBuilder {
        let source = self.source;
        RequestBuilder::new(StripeMethod::Get, format!("/sources/{source}/source_transactions"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateSourceBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow: Option<CreateSourceFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mandate: Option<CreateSourceMandate<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<Owner<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver: Option<CreateSourceReceiver>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<CreateSourceRedirect<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_order: Option<CreateSourceSourceOrder<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<&'a str>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<CreateSourceUsage>,
}
impl<'a> CreateSourceBuilder<'a> {
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSourceFlow {
    CodeVerification,
    None,
    Receiver,
    Redirect,
}
impl CreateSourceFlow {
    pub fn as_str(self) -> &'static str {
        use CreateSourceFlow::*;
        match self {
            CodeVerification => "code_verification",
            None => "none",
            Receiver => "receiver",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for CreateSourceFlow {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceFlow::*;
        match s {
            "code_verification" => Ok(CodeVerification),
            "none" => Ok(None),
            "receiver" => Ok(Receiver),
            "redirect" => Ok(Redirect),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateSourceFlow"))
    }
}
/// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSourceMandate<'a> {
    /// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<CreateSourceMandateAcceptance<'a>>,
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
impl<'a> CreateSourceMandate<'a> {
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
impl<'a> Default for CreateSourceMandate<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSourceMandateAcceptance<'a> {
    /// The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The parameters required to store a mandate accepted offline.
    /// Should only be set if `mandate[type]` is `offline`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<MandateOfflineAcceptanceParams<'a>>,
    /// The parameters required to store a mandate accepted online.
    /// Should only be set if `mandate[type]` is `online`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<MandateOnlineAcceptanceParams<'a>>,
    /// The status of the mandate acceptance.
    /// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
    pub status: CreateSourceMandateAcceptanceStatus,
    /// The type of acceptance information included with the mandate. Either `online` or `offline`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateSourceMandateAcceptanceType>,
    /// The user agent of the browser from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> CreateSourceMandateAcceptance<'a> {
    pub fn new(status: CreateSourceMandateAcceptanceStatus) -> Self {
        Self {
            date: None,
            ip: None,
            offline: None,
            online: None,
            status,
            type_: None,
            user_agent: None,
        }
    }
}
/// The status of the mandate acceptance.
/// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSourceMandateAcceptanceStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}
impl CreateSourceMandateAcceptanceStatus {
    pub fn as_str(self) -> &'static str {
        use CreateSourceMandateAcceptanceStatus::*;
        match self {
            Accepted => "accepted",
            Pending => "pending",
            Refused => "refused",
            Revoked => "revoked",
        }
    }
}

impl std::str::FromStr for CreateSourceMandateAcceptanceStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceMandateAcceptanceStatus::*;
        match s {
            "accepted" => Ok(Accepted),
            "pending" => Ok(Pending),
            "refused" => Ok(Refused),
            "revoked" => Ok(Revoked),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateSourceMandateAcceptanceStatus")
        })
    }
}
/// The type of acceptance information included with the mandate. Either `online` or `offline`
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSourceMandateAcceptanceType {
    Offline,
    Online,
}
impl CreateSourceMandateAcceptanceType {
    pub fn as_str(self) -> &'static str {
        use CreateSourceMandateAcceptanceType::*;
        match self {
            Offline => "offline",
            Online => "online",
        }
    }
}

impl std::str::FromStr for CreateSourceMandateAcceptanceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceMandateAcceptanceType::*;
        match s {
            "offline" => Ok(Offline),
            "online" => Ok(Online),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateSourceMandateAcceptanceType")
        })
    }
}
/// The interval of debits permitted by the mandate.
/// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSourceMandateInterval {
    OneTime,
    Scheduled,
    Variable,
}
impl CreateSourceMandateInterval {
    pub fn as_str(self) -> &'static str {
        use CreateSourceMandateInterval::*;
        match self {
            OneTime => "one_time",
            Scheduled => "scheduled",
            Variable => "variable",
        }
    }
}

impl std::str::FromStr for CreateSourceMandateInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceMandateInterval::*;
        match s {
            "one_time" => Ok(OneTime),
            "scheduled" => Ok(Scheduled),
            "variable" => Ok(Variable),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateSourceMandateInterval"))
    }
}
/// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
/// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSourceMandateNotificationMethod {
    DeprecatedNone,
    Email,
    Manual,
    None,
    StripeEmail,
}
impl CreateSourceMandateNotificationMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSourceMandateNotificationMethod::*;
        match self {
            DeprecatedNone => "deprecated_none",
            Email => "email",
            Manual => "manual",
            None => "none",
            StripeEmail => "stripe_email",
        }
    }
}

impl std::str::FromStr for CreateSourceMandateNotificationMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceMandateNotificationMethod::*;
        match s {
            "deprecated_none" => Ok(DeprecatedNone),
            "email" => Ok(Email),
            "manual" => Ok(Manual),
            "none" => Ok(None),
            "stripe_email" => Ok(StripeEmail),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateSourceMandateNotificationMethod")
        })
    }
}
/// Optional parameters for the receiver flow.
/// Can be set only if the source is a receiver (`flow` is `receiver`).
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSourceReceiverRefundAttributesMethod {
    Email,
    Manual,
    None,
}
impl CreateSourceReceiverRefundAttributesMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSourceReceiverRefundAttributesMethod::*;
        match self {
            Email => "email",
            Manual => "manual",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateSourceReceiverRefundAttributesMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceReceiverRefundAttributesMethod::*;
        match s {
            "email" => Ok(Email),
            "manual" => Ok(Manual),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateSourceReceiverRefundAttributesMethod")
        })
    }
}
/// Parameters required for the redirect flow.
/// Required if the source is authenticated by a redirect (`flow` is `redirect`).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSourceRedirect<'a> {
    /// The URL you provide to redirect the customer back to you after they authenticated their payment.
    /// It can use your application URI scheme in the context of a mobile application.
    pub return_url: &'a str,
}
impl<'a> CreateSourceRedirect<'a> {
    pub fn new(return_url: &'a str) -> Self {
        Self { return_url }
    }
}
/// Information about the items and shipping associated with the source.
/// Required for transactional credit (for example Klarna) sources before you can charge it.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSourceSourceOrder<'a> {
    /// List of items constituting the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<&'a [CreateSourceSourceOrderItems<'a>]>,
    /// Shipping address for the order.
    /// Required if any of the SKUs are for products that have `shippable` set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<OrderShipping<'a>>,
}
impl<'a> CreateSourceSourceOrder<'a> {
    pub fn new() -> Self {
        Self { items: None, shipping: None }
    }
}
impl<'a> Default for CreateSourceSourceOrder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// List of items constituting the order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSourceSourceOrderItems<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The ID of the SKU being ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<&'a str>,
    /// The quantity of this order item.
    /// When type is `sku`, this is the number of instances of the SKU to be ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateSourceSourceOrderItemsType>,
}
impl<'a> CreateSourceSourceOrderItems<'a> {
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
impl<'a> Default for CreateSourceSourceOrderItems<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSourceSourceOrderItemsType {
    Discount,
    Shipping,
    Sku,
    Tax,
}
impl CreateSourceSourceOrderItemsType {
    pub fn as_str(self) -> &'static str {
        use CreateSourceSourceOrderItemsType::*;
        match self {
            Discount => "discount",
            Shipping => "shipping",
            Sku => "sku",
            Tax => "tax",
        }
    }
}

impl std::str::FromStr for CreateSourceSourceOrderItemsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceSourceOrderItemsType::*;
        match s {
            "discount" => Ok(Discount),
            "shipping" => Ok(Shipping),
            "sku" => Ok(Sku),
            "tax" => Ok(Tax),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateSourceSourceOrderItemsType")
        })
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSourceUsage {
    Reusable,
    SingleUse,
}
impl CreateSourceUsage {
    pub fn as_str(self) -> &'static str {
        use CreateSourceUsage::*;
        match self {
            Reusable => "reusable",
            SingleUse => "single_use",
        }
    }
}

impl std::str::FromStr for CreateSourceUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSourceUsage::*;
        match s {
            "reusable" => Ok(Reusable),
            "single_use" => Ok(SingleUse),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateSourceUsage"))
    }
}
/// Creates a new source object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSource<'a> {
    inner: CreateSourceBuilder<'a>,
}
impl<'a> CreateSource<'a> {
    /// Construct a new `CreateSource`.
    pub fn new() -> Self {
        Self { inner: CreateSourceBuilder::new() }
    }
    /// Amount associated with the source.
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    /// Not supported for `receiver` type sources, where charge amount may not be specified until funds land.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source.
    /// This is the currency for which the source will be chargeable once ready.
    pub fn currency(mut self, currency: stripe_types::Currency) -> Self {
        self.inner.currency = Some(currency);
        self
    }
    /// The `Customer` to whom the original source is attached to.
    /// Must be set when the original source is not a `Source` (e.g., `Card`).
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The authentication `flow` of the source to create.
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    /// It is generally inferred unless a type supports multiple flows.
    pub fn flow(mut self, flow: CreateSourceFlow) -> Self {
        self.inner.flow = Some(flow);
        self
    }
    /// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
    pub fn mandate(mut self, mandate: CreateSourceMandate<'a>) -> Self {
        self.inner.mandate = Some(mandate);
        self
    }
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The source to share.
    pub fn original_source(mut self, original_source: &'a str) -> Self {
        self.inner.original_source = Some(original_source);
        self
    }
    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    pub fn owner(mut self, owner: Owner<'a>) -> Self {
        self.inner.owner = Some(owner);
        self
    }
    /// Optional parameters for the receiver flow.
    /// Can be set only if the source is a receiver (`flow` is `receiver`).
    pub fn receiver(mut self, receiver: CreateSourceReceiver) -> Self {
        self.inner.receiver = Some(receiver);
        self
    }
    /// Parameters required for the redirect flow.
    /// Required if the source is authenticated by a redirect (`flow` is `redirect`).
    pub fn redirect(mut self, redirect: CreateSourceRedirect<'a>) -> Self {
        self.inner.redirect = Some(redirect);
        self
    }
    /// Information about the items and shipping associated with the source.
    /// Required for transactional credit (for example Klarna) sources before you can charge it.
    pub fn source_order(mut self, source_order: CreateSourceSourceOrder<'a>) -> Self {
        self.inner.source_order = Some(source_order);
        self
    }
    /// An arbitrary string to be displayed on your customer's statement.
    /// As an example, if your website is `RunClub` and the item you're charging for is a race ticket, you may want to specify a `statement_descriptor` of `RunClub 5K race ticket.` While many payment types will display this information, some may not display it at all.
    pub fn statement_descriptor(mut self, statement_descriptor: &'a str) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor);
        self
    }
    /// An optional token used to create the source.
    /// When passed, token properties will override source parameters.
    pub fn token(mut self, token: &'a str) -> Self {
        self.inner.token = Some(token);
        self
    }
    /// The `type` of the source to create.
    /// Required unless `customer` and `original_source` are specified (see the [Cloning card Sources](https://stripe.com/docs/sources/connect#cloning-card-sources) guide).
    pub fn type_(mut self, type_: &'a str) -> Self {
        self.inner.type_ = Some(type_);
        self
    }
    pub fn usage(mut self, usage: CreateSourceUsage) -> Self {
        self.inner.usage = Some(usage);
        self
    }
}
impl<'a> Default for CreateSource<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateSource<'_> {
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

impl StripeRequest for CreateSource<'_> {
    type Output = stripe_shared::Source;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/sources").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateSourceBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mandate: Option<UpdateSourceMandate<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<Owner<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_order: Option<UpdateSourceSourceOrder<'a>>,
}
impl<'a> UpdateSourceBuilder<'a> {
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSourceMandate<'a> {
    /// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<UpdateSourceMandateAcceptance<'a>>,
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
impl<'a> UpdateSourceMandate<'a> {
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
impl<'a> Default for UpdateSourceMandate<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSourceMandateAcceptance<'a> {
    /// The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The parameters required to store a mandate accepted offline.
    /// Should only be set if `mandate[type]` is `offline`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<MandateOfflineAcceptanceParams<'a>>,
    /// The parameters required to store a mandate accepted online.
    /// Should only be set if `mandate[type]` is `online`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<MandateOnlineAcceptanceParams<'a>>,
    /// The status of the mandate acceptance.
    /// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
    pub status: UpdateSourceMandateAcceptanceStatus,
    /// The type of acceptance information included with the mandate. Either `online` or `offline`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpdateSourceMandateAcceptanceType>,
    /// The user agent of the browser from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> UpdateSourceMandateAcceptance<'a> {
    pub fn new(status: UpdateSourceMandateAcceptanceStatus) -> Self {
        Self {
            date: None,
            ip: None,
            offline: None,
            online: None,
            status,
            type_: None,
            user_agent: None,
        }
    }
}
/// The status of the mandate acceptance.
/// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSourceMandateAcceptanceStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}
impl UpdateSourceMandateAcceptanceStatus {
    pub fn as_str(self) -> &'static str {
        use UpdateSourceMandateAcceptanceStatus::*;
        match self {
            Accepted => "accepted",
            Pending => "pending",
            Refused => "refused",
            Revoked => "revoked",
        }
    }
}

impl std::str::FromStr for UpdateSourceMandateAcceptanceStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSourceMandateAcceptanceStatus::*;
        match s {
            "accepted" => Ok(Accepted),
            "pending" => Ok(Pending),
            "refused" => Ok(Refused),
            "revoked" => Ok(Revoked),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateSourceMandateAcceptanceStatus")
        })
    }
}
/// The type of acceptance information included with the mandate. Either `online` or `offline`
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSourceMandateAcceptanceType {
    Offline,
    Online,
}
impl UpdateSourceMandateAcceptanceType {
    pub fn as_str(self) -> &'static str {
        use UpdateSourceMandateAcceptanceType::*;
        match self {
            Offline => "offline",
            Online => "online",
        }
    }
}

impl std::str::FromStr for UpdateSourceMandateAcceptanceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSourceMandateAcceptanceType::*;
        match s {
            "offline" => Ok(Offline),
            "online" => Ok(Online),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateSourceMandateAcceptanceType")
        })
    }
}
/// The interval of debits permitted by the mandate.
/// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSourceMandateInterval {
    OneTime,
    Scheduled,
    Variable,
}
impl UpdateSourceMandateInterval {
    pub fn as_str(self) -> &'static str {
        use UpdateSourceMandateInterval::*;
        match self {
            OneTime => "one_time",
            Scheduled => "scheduled",
            Variable => "variable",
        }
    }
}

impl std::str::FromStr for UpdateSourceMandateInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSourceMandateInterval::*;
        match s {
            "one_time" => Ok(OneTime),
            "scheduled" => Ok(Scheduled),
            "variable" => Ok(Variable),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for UpdateSourceMandateInterval"))
    }
}
/// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
/// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSourceMandateNotificationMethod {
    DeprecatedNone,
    Email,
    Manual,
    None,
    StripeEmail,
}
impl UpdateSourceMandateNotificationMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSourceMandateNotificationMethod::*;
        match self {
            DeprecatedNone => "deprecated_none",
            Email => "email",
            Manual => "manual",
            None => "none",
            StripeEmail => "stripe_email",
        }
    }
}

impl std::str::FromStr for UpdateSourceMandateNotificationMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSourceMandateNotificationMethod::*;
        match s {
            "deprecated_none" => Ok(DeprecatedNone),
            "email" => Ok(Email),
            "manual" => Ok(Manual),
            "none" => Ok(None),
            "stripe_email" => Ok(StripeEmail),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateSourceMandateNotificationMethod")
        })
    }
}
/// Information about the items and shipping associated with the source.
/// Required for transactional credit (for example Klarna) sources before you can charge it.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSourceSourceOrder<'a> {
    /// List of items constituting the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<&'a [UpdateSourceSourceOrderItems<'a>]>,
    /// Shipping address for the order.
    /// Required if any of the SKUs are for products that have `shippable` set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<OrderShipping<'a>>,
}
impl<'a> UpdateSourceSourceOrder<'a> {
    pub fn new() -> Self {
        Self { items: None, shipping: None }
    }
}
impl<'a> Default for UpdateSourceSourceOrder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// List of items constituting the order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSourceSourceOrderItems<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The ID of the SKU being ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<&'a str>,
    /// The quantity of this order item.
    /// When type is `sku`, this is the number of instances of the SKU to be ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpdateSourceSourceOrderItemsType>,
}
impl<'a> UpdateSourceSourceOrderItems<'a> {
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
impl<'a> Default for UpdateSourceSourceOrderItems<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSourceSourceOrderItemsType {
    Discount,
    Shipping,
    Sku,
    Tax,
}
impl UpdateSourceSourceOrderItemsType {
    pub fn as_str(self) -> &'static str {
        use UpdateSourceSourceOrderItemsType::*;
        match self {
            Discount => "discount",
            Shipping => "shipping",
            Sku => "sku",
            Tax => "tax",
        }
    }
}

impl std::str::FromStr for UpdateSourceSourceOrderItemsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSourceSourceOrderItemsType::*;
        match s {
            "discount" => Ok(Discount),
            "shipping" => Ok(Shipping),
            "sku" => Ok(Sku),
            "tax" => Ok(Tax),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateSourceSourceOrderItemsType")
        })
    }
}
/// Updates the specified source by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
///
/// This request accepts the `metadata` and `owner` as arguments.
/// It is also possible to update type specific information for selected payment methods.
/// Please refer to our [payment method guides](https://stripe.com/docs/sources) for more detail.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSource<'a> {
    inner: UpdateSourceBuilder<'a>,
    source: &'a stripe_shared::SourceId,
}
impl<'a> UpdateSource<'a> {
    /// Construct a new `UpdateSource`.
    pub fn new(source: &'a stripe_shared::SourceId) -> Self {
        Self { source, inner: UpdateSourceBuilder::new() }
    }
    /// Amount associated with the source.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
    pub fn mandate(mut self, mandate: UpdateSourceMandate<'a>) -> Self {
        self.inner.mandate = Some(mandate);
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
    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    pub fn owner(mut self, owner: Owner<'a>) -> Self {
        self.inner.owner = Some(owner);
        self
    }
    /// Information about the items and shipping associated with the source.
    /// Required for transactional credit (for example Klarna) sources before you can charge it.
    pub fn source_order(mut self, source_order: UpdateSourceSourceOrder<'a>) -> Self {
        self.inner.source_order = Some(source_order);
        self
    }
}
impl UpdateSource<'_> {
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

impl StripeRequest for UpdateSource<'_> {
    type Output = stripe_shared::Source;

    fn build(&self) -> RequestBuilder {
        let source = self.source;
        RequestBuilder::new(StripeMethod::Post, format!("/sources/{source}")).form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct VerifySourceBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    values: &'a [&'a str],
}
impl<'a> VerifySourceBuilder<'a> {
    fn new(values: &'a [&'a str]) -> Self {
        Self { expand: None, values }
    }
}
/// Verify a given source.
#[derive(Clone, Debug, serde::Serialize)]
pub struct VerifySource<'a> {
    inner: VerifySourceBuilder<'a>,
    source: &'a stripe_shared::SourceId,
}
impl<'a> VerifySource<'a> {
    /// Construct a new `VerifySource`.
    pub fn new(source: &'a stripe_shared::SourceId, values: &'a [&'a str]) -> Self {
        Self { source, inner: VerifySourceBuilder::new(values) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl VerifySource<'_> {
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

impl StripeRequest for VerifySource<'_> {
    type Output = stripe_shared::Source;

    fn build(&self) -> RequestBuilder {
        let source = self.source;
        RequestBuilder::new(StripeMethod::Post, format!("/sources/{source}/verify"))
            .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct MandateOfflineAcceptanceParams<'a> {
    /// An email to contact you with if a copy of the mandate is requested, required if `type` is `offline`.
    pub contact_email: &'a str,
}
impl<'a> MandateOfflineAcceptanceParams<'a> {
    pub fn new(contact_email: &'a str) -> Self {
        Self { contact_email }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct MandateOnlineAcceptanceParams<'a> {
    /// The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<&'a str>,
    /// The user agent of the browser from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> MandateOnlineAcceptanceParams<'a> {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl<'a> Default for MandateOnlineAcceptanceParams<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SourceAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> SourceAddress<'a> {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl<'a> Default for SourceAddress<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct Address<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    pub line1: &'a str,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> Address<'a> {
    pub fn new(line1: &'a str) -> Self {
        Self { city: None, country: None, line1, line2: None, postal_code: None, state: None }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct Owner<'a> {
    /// Owner's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<SourceAddress<'a>>,
    /// Owner's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Owner's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Owner's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> Owner<'a> {
    pub fn new() -> Self {
        Self { address: None, email: None, name: None, phone: None }
    }
}
impl<'a> Default for Owner<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct OrderShipping<'a> {
    /// Shipping address.
    pub address: Address<'a>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<&'a str>,
    /// Recipient name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The tracking number for a physical product, obtained from the delivery service.
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<&'a str>,
}
impl<'a> OrderShipping<'a> {
    pub fn new(address: Address<'a>) -> Self {
        Self { address, carrier: None, name: None, phone: None, tracking_number: None }
    }
}
