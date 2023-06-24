use stripe::{Client, Response};

impl stripe_core::source::Source {
    /// Delete a specified source for a given customer.
    pub fn detach(
        client: &Client,
        customer: &stripe_core::customer::CustomerId,
        id: &str,
        params: DetachSource,
    ) -> Response<DetachReturned> {
        client.send_form(
            &format!("/customers/{customer}/sources/{id}", customer = customer, id = id),
            params,
            http_types::Method::Delete,
        )
    }
    /// Retrieves an existing source object.
    ///
    /// Supply the unique source ID from a source creation request and Stripe will return the corresponding up-to-date source object information.
    pub fn retrieve(
        client: &Client,
        source: &stripe_core::source::SourceId,
        params: RetrieveSource,
    ) -> Response<stripe_core::source::Source> {
        client.get_query(&format!("/sources/{source}", source = source), params)
    }
    /// Creates a new source object.
    pub fn create(client: &Client, params: CreateSource) -> Response<stripe_core::source::Source> {
        client.send_form("/sources", params, http_types::Method::Post)
    }
    /// Updates the specified source by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.  This request accepts the `metadata` and `owner` as arguments.
    /// It is also possible to update type specific information for selected payment methods.
    /// Please refer to our [payment method guides](https://stripe.com/docs/sources) for more detail.
    pub fn update(
        client: &Client,
        source: &stripe_core::source::SourceId,
        params: UpdateSource,
    ) -> Response<stripe_core::source::Source> {
        client.send_form(
            &format!("/sources/{source}", source = source),
            params,
            http_types::Method::Post,
        )
    }
    /// Verify a given source.
    pub fn verify(
        client: &Client,
        source: &stripe_core::source::SourceId,
        params: VerifySource,
    ) -> Response<stripe_core::source::Source> {
        client.send_form(
            &format!("/sources/{source}/verify", source = source),
            params,
            http_types::Method::Post,
        )
    }
    /// List source transactions for a given source.
    pub fn source_transactions(
        client: &Client,
        source: &stripe_core::source::SourceId,
        params: SourceTransactionsSource,
    ) -> Response<stripe_types::List<stripe_types::source_transaction::SourceTransaction>> {
        client.get_query(&format!("/sources/{source}/source_transactions", source = source), params)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[serde(untagged, rename_all = "snake_case")]
pub enum DetachReturned {
    PaymentSource(stripe_core::payment_source::PaymentSource),
    DeletedPaymentSource(stripe_core::payment_source::DeletedPaymentSource),
}
#[cfg(feature = "min-ser")]
impl crate::deser::StripeDeserialize for DetachReturned {
    fn deserialize(str: &str) -> Result<Self, crate::StripeError> {
        use miniserde::json::from_str;

        use crate::deser::StripeDeserialize;
        let maybe_deleted: crate::deser::MaybeDeleted = from_str(str)?;
        let deleted = maybe_deleted.deleted.unwrap_or(false);
        if deleted {
            Ok(Self::DeletedPaymentSource(StripeDeserialize::deserialize(str)?))
        } else {
            Ok(Self::PaymentSource(StripeDeserialize::deserialize(str)?))
        }
    }
}

#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DetachSource<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> DetachSource<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveSource<'a> {
    /// The client secret of the source.
    ///
    /// Required if a publishable key is used to retrieve the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveSource<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSource<'a> {
    /// Amount associated with the source.
    ///
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    /// Not supported for `receiver` type sources, where charge amount may not be specified until funds land.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source.
    ///
    /// This is the currency for which the source will be chargeable once ready.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The `Customer` to whom the original source is attached to.
    ///
    /// Must be set when the original source is not a `Source` (e.g., `Card`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The authentication `flow` of the source to create.
    ///
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    /// It is generally inferred unless a type supports multiple flows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<CreateSourceFlow>,
    /// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<CreateSourceMandate<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The source to share.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_source: Option<&'a str>,
    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<CreateSourceOwner<'a>>,
    /// Optional parameters for the receiver flow.
    ///
    /// Can be set only if the source is a receiver (`flow` is `receiver`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<CreateSourceReceiver>,
    /// Parameters required for the redirect flow.
    ///
    /// Required if the source is authenticated by a redirect (`flow` is `redirect`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<CreateSourceRedirect<'a>>,
    /// Information about the items and shipping associated with the source.
    ///
    /// Required for transactional credit (for example Klarna) sources before you can charge it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order: Option<CreateSourceSourceOrder<'a>>,
    /// An arbitrary string to be displayed on your customer's statement.
    ///
    /// As an example, if your website is `RunClub` and the item you're charging for is a race ticket, you may want to specify a `statement_descriptor` of `RunClub 5K race ticket.` While many payment types will display this information, some may not display it at all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// An optional token used to create the source.
    ///
    /// When passed, token properties will override source parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<&'a str>,
    /// The `type` of the source to create.
    ///
    /// Required unless `customer` and `original_source` are specified (see the [Cloning card Sources](https://stripe.com/docs/sources/connect#cloning-card-sources) guide).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<CreateSourceUsage>,
}
impl<'a> CreateSource<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The authentication `flow` of the source to create.
///
/// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
/// It is generally inferred unless a type supports multiple flows.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateSourceFlow {
    CodeVerification,
    None,
    Receiver,
    Redirect,
}

impl CreateSourceFlow {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CodeVerification => "code_verification",
            Self::None => "none",
            Self::Receiver => "receiver",
            Self::Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for CreateSourceFlow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "code_verification" => Ok(Self::CodeVerification),
            "none" => Ok(Self::None),
            "receiver" => Ok(Self::Receiver),
            "redirect" => Ok(Self::Redirect),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateSourceFlow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSourceFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSourceMandate<'a> {
    /// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<CreateSourceMandateAcceptance<'a>>,
    /// The amount specified by the mandate.
    ///
    /// (Leave null for a mandate covering all amounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The currency specified by the mandate.
    ///
    /// (Must match `currency` of the source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The interval of debits permitted by the mandate.
    ///
    /// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CreateSourceMandateInterval>,
    /// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
    ///
    /// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_method: Option<CreateSourceMandateNotificationMethod>,
}
impl<'a> CreateSourceMandate<'a> {
    pub fn new() -> Self {
        Self::default()
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
    ///
    /// Should only be set if `mandate[type]` is `offline`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<CreateSourceMandateAcceptanceOffline<'a>>,
    /// The parameters required to store a mandate accepted online.
    ///
    /// Should only be set if `mandate[type]` is `online`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<CreateSourceMandateAcceptanceOnline<'a>>,
    /// The status of the mandate acceptance.
    ///
    /// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
    pub status: CreateSourceMandateAcceptanceStatus,
    /// The type of acceptance information included with the mandate.
    ///
    /// Either `online` or `offline`.
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
            date: Default::default(),
            ip: Default::default(),
            offline: Default::default(),
            online: Default::default(),
            status,
            type_: Default::default(),
            user_agent: Default::default(),
        }
    }
}
/// The parameters required to store a mandate accepted offline.
///
/// Should only be set if `mandate[type]` is `offline`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSourceMandateAcceptanceOffline<'a> {
    /// An email to contact you with if a copy of the mandate is requested, required if `type` is `offline`.
    pub contact_email: &'a str,
}
impl<'a> CreateSourceMandateAcceptanceOffline<'a> {
    pub fn new(contact_email: &'a str) -> Self {
        Self { contact_email }
    }
}
/// The parameters required to store a mandate accepted online.
///
/// Should only be set if `mandate[type]` is `online`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSourceMandateAcceptanceOnline<'a> {
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
impl<'a> CreateSourceMandateAcceptanceOnline<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The status of the mandate acceptance.
///
/// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateSourceMandateAcceptanceStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}

impl CreateSourceMandateAcceptanceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Pending => "pending",
            Self::Refused => "refused",
            Self::Revoked => "revoked",
        }
    }
}

impl std::str::FromStr for CreateSourceMandateAcceptanceStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "accepted" => Ok(Self::Accepted),
            "pending" => Ok(Self::Pending),
            "refused" => Ok(Self::Refused),
            "revoked" => Ok(Self::Revoked),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateSourceMandateAcceptanceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSourceMandateAcceptanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// The type of acceptance information included with the mandate.
///
/// Either `online` or `offline`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateSourceMandateAcceptanceType {
    Offline,
    Online,
}

impl CreateSourceMandateAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl std::str::FromStr for CreateSourceMandateAcceptanceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "offline" => Ok(Self::Offline),
            "online" => Ok(Self::Online),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateSourceMandateAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSourceMandateAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// The interval of debits permitted by the mandate.
///
/// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateSourceMandateInterval {
    OneTime,
    Scheduled,
    Variable,
}

impl CreateSourceMandateInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OneTime => "one_time",
            Self::Scheduled => "scheduled",
            Self::Variable => "variable",
        }
    }
}

impl std::str::FromStr for CreateSourceMandateInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one_time" => Ok(Self::OneTime),
            "scheduled" => Ok(Self::Scheduled),
            "variable" => Ok(Self::Variable),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateSourceMandateInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSourceMandateInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
///
/// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateSourceMandateNotificationMethod {
    DeprecatedNone,
    Email,
    Manual,
    None,
    StripeEmail,
}

impl CreateSourceMandateNotificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeprecatedNone => "deprecated_none",
            Self::Email => "email",
            Self::Manual => "manual",
            Self::None => "none",
            Self::StripeEmail => "stripe_email",
        }
    }
}

impl std::str::FromStr for CreateSourceMandateNotificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "deprecated_none" => Ok(Self::DeprecatedNone),
            "email" => Ok(Self::Email),
            "manual" => Ok(Self::Manual),
            "none" => Ok(Self::None),
            "stripe_email" => Ok(Self::StripeEmail),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateSourceMandateNotificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSourceMandateNotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// Information about the owner of the payment instrument that may be used or required by particular source types.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSourceOwner<'a> {
    /// Owner's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateSourceOwnerAddress<'a>>,
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
impl<'a> CreateSourceOwner<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Owner's address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSourceOwnerAddress<'a> {
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
impl<'a> CreateSourceOwnerAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Optional parameters for the receiver flow.
///
/// Can be set only if the source is a receiver (`flow` is `receiver`).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSourceReceiver {
    /// The method Stripe should use to request information needed to process a refund or mispayment.
    ///
    /// Either `email` (an email is sent directly to the customer) or `manual` (a `source.refund_attributes_required` event is sent to your webhooks endpoint).
    /// Refer to each payment method's documentation to learn which refund attributes may be required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_attributes_method: Option<CreateSourceReceiverRefundAttributesMethod>,
}
impl CreateSourceReceiver {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The method Stripe should use to request information needed to process a refund or mispayment.
///
/// Either `email` (an email is sent directly to the customer) or `manual` (a `source.refund_attributes_required` event is sent to your webhooks endpoint).
/// Refer to each payment method's documentation to learn which refund attributes may be required.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateSourceReceiverRefundAttributesMethod {
    Email,
    Manual,
    None,
}

impl CreateSourceReceiverRefundAttributesMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Manual => "manual",
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for CreateSourceReceiverRefundAttributesMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "email" => Ok(Self::Email),
            "manual" => Ok(Self::Manual),
            "none" => Ok(Self::None),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateSourceReceiverRefundAttributesMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSourceReceiverRefundAttributesMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// Parameters required for the redirect flow.
///
/// Required if the source is authenticated by a redirect (`flow` is `redirect`).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSourceRedirect<'a> {
    /// The URL you provide to redirect the customer back to you after they authenticated their payment.
    ///
    /// It can use your application URI scheme in the context of a mobile application.
    pub return_url: &'a str,
}
impl<'a> CreateSourceRedirect<'a> {
    pub fn new(return_url: &'a str) -> Self {
        Self { return_url }
    }
}
/// Information about the items and shipping associated with the source.
///
/// Required for transactional credit (for example Klarna) sources before you can charge it.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSourceSourceOrder<'a> {
    /// List of items constituting the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<&'a [CreateSourceSourceOrderItems<'a>]>,
    /// Shipping address for the order.
    ///
    /// Required if any of the SKUs are for products that have `shippable` set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateSourceSourceOrderShipping<'a>>,
}
impl<'a> CreateSourceSourceOrder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// List of items constituting the order.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
    ///
    /// When type is `sku`, this is the number of instances of the SKU to be ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateSourceSourceOrderItemsType>,
}
impl<'a> CreateSourceSourceOrderItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateSourceSourceOrderItemsType {
    Discount,
    Shipping,
    Sku,
    Tax,
}

impl CreateSourceSourceOrderItemsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Discount => "discount",
            Self::Shipping => "shipping",
            Self::Sku => "sku",
            Self::Tax => "tax",
        }
    }
}

impl std::str::FromStr for CreateSourceSourceOrderItemsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "discount" => Ok(Self::Discount),
            "shipping" => Ok(Self::Shipping),
            "sku" => Ok(Self::Sku),
            "tax" => Ok(Self::Tax),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateSourceSourceOrderItemsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSourceSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// Shipping address for the order.
///
/// Required if any of the SKUs are for products that have `shippable` set to true.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSourceSourceOrderShipping<'a> {
    /// Shipping address.
    pub address: CreateSourceSourceOrderShippingAddress<'a>,
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
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<&'a str>,
}
impl<'a> CreateSourceSourceOrderShipping<'a> {
    pub fn new(address: CreateSourceSourceOrderShippingAddress<'a>) -> Self {
        Self {
            address,
            carrier: Default::default(),
            name: Default::default(),
            phone: Default::default(),
            tracking_number: Default::default(),
        }
    }
}
/// Shipping address.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSourceSourceOrderShippingAddress<'a> {
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
impl<'a> CreateSourceSourceOrderShippingAddress<'a> {
    pub fn new(line1: &'a str) -> Self {
        Self {
            city: Default::default(),
            country: Default::default(),
            line1,
            line2: Default::default(),
            postal_code: Default::default(),
            state: Default::default(),
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateSourceUsage {
    Reusable,
    SingleUse,
}

impl CreateSourceUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Reusable => "reusable",
            Self::SingleUse => "single_use",
        }
    }
}

impl std::str::FromStr for CreateSourceUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "reusable" => Ok(Self::Reusable),
            "single_use" => Ok(Self::SingleUse),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateSourceUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSourceUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSource<'a> {
    /// Amount associated with the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<UpdateSourceMandate<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UpdateSourceOwner<'a>>,
    /// Information about the items and shipping associated with the source.
    ///
    /// Required for transactional credit (for example Klarna) sources before you can charge it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order: Option<UpdateSourceSourceOrder<'a>>,
}
impl<'a> UpdateSource<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSourceMandate<'a> {
    /// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<UpdateSourceMandateAcceptance<'a>>,
    /// The amount specified by the mandate.
    ///
    /// (Leave null for a mandate covering all amounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The currency specified by the mandate.
    ///
    /// (Must match `currency` of the source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The interval of debits permitted by the mandate.
    ///
    /// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<UpdateSourceMandateInterval>,
    /// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
    ///
    /// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_method: Option<UpdateSourceMandateNotificationMethod>,
}
impl<'a> UpdateSourceMandate<'a> {
    pub fn new() -> Self {
        Self::default()
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
    ///
    /// Should only be set if `mandate[type]` is `offline`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<UpdateSourceMandateAcceptanceOffline<'a>>,
    /// The parameters required to store a mandate accepted online.
    ///
    /// Should only be set if `mandate[type]` is `online`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<UpdateSourceMandateAcceptanceOnline<'a>>,
    /// The status of the mandate acceptance.
    ///
    /// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
    pub status: UpdateSourceMandateAcceptanceStatus,
    /// The type of acceptance information included with the mandate.
    ///
    /// Either `online` or `offline`.
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
            date: Default::default(),
            ip: Default::default(),
            offline: Default::default(),
            online: Default::default(),
            status,
            type_: Default::default(),
            user_agent: Default::default(),
        }
    }
}
/// The parameters required to store a mandate accepted offline.
///
/// Should only be set if `mandate[type]` is `offline`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSourceMandateAcceptanceOffline<'a> {
    /// An email to contact you with if a copy of the mandate is requested, required if `type` is `offline`.
    pub contact_email: &'a str,
}
impl<'a> UpdateSourceMandateAcceptanceOffline<'a> {
    pub fn new(contact_email: &'a str) -> Self {
        Self { contact_email }
    }
}
/// The parameters required to store a mandate accepted online.
///
/// Should only be set if `mandate[type]` is `online`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSourceMandateAcceptanceOnline<'a> {
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
impl<'a> UpdateSourceMandateAcceptanceOnline<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The status of the mandate acceptance.
///
/// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateSourceMandateAcceptanceStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}

impl UpdateSourceMandateAcceptanceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Pending => "pending",
            Self::Refused => "refused",
            Self::Revoked => "revoked",
        }
    }
}

impl std::str::FromStr for UpdateSourceMandateAcceptanceStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "accepted" => Ok(Self::Accepted),
            "pending" => Ok(Self::Pending),
            "refused" => Ok(Self::Refused),
            "revoked" => Ok(Self::Revoked),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateSourceMandateAcceptanceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSourceMandateAcceptanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// The type of acceptance information included with the mandate.
///
/// Either `online` or `offline`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateSourceMandateAcceptanceType {
    Offline,
    Online,
}

impl UpdateSourceMandateAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl std::str::FromStr for UpdateSourceMandateAcceptanceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "offline" => Ok(Self::Offline),
            "online" => Ok(Self::Online),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateSourceMandateAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSourceMandateAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// The interval of debits permitted by the mandate.
///
/// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateSourceMandateInterval {
    OneTime,
    Scheduled,
    Variable,
}

impl UpdateSourceMandateInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OneTime => "one_time",
            Self::Scheduled => "scheduled",
            Self::Variable => "variable",
        }
    }
}

impl std::str::FromStr for UpdateSourceMandateInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one_time" => Ok(Self::OneTime),
            "scheduled" => Ok(Self::Scheduled),
            "variable" => Ok(Self::Variable),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateSourceMandateInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSourceMandateInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
///
/// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateSourceMandateNotificationMethod {
    DeprecatedNone,
    Email,
    Manual,
    None,
    StripeEmail,
}

impl UpdateSourceMandateNotificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeprecatedNone => "deprecated_none",
            Self::Email => "email",
            Self::Manual => "manual",
            Self::None => "none",
            Self::StripeEmail => "stripe_email",
        }
    }
}

impl std::str::FromStr for UpdateSourceMandateNotificationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "deprecated_none" => Ok(Self::DeprecatedNone),
            "email" => Ok(Self::Email),
            "manual" => Ok(Self::Manual),
            "none" => Ok(Self::None),
            "stripe_email" => Ok(Self::StripeEmail),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateSourceMandateNotificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSourceMandateNotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// Information about the owner of the payment instrument that may be used or required by particular source types.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSourceOwner<'a> {
    /// Owner's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateSourceOwnerAddress<'a>>,
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
impl<'a> UpdateSourceOwner<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Owner's address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSourceOwnerAddress<'a> {
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
impl<'a> UpdateSourceOwnerAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the items and shipping associated with the source.
///
/// Required for transactional credit (for example Klarna) sources before you can charge it.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSourceSourceOrder<'a> {
    /// List of items constituting the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<&'a [UpdateSourceSourceOrderItems<'a>]>,
    /// Shipping address for the order.
    ///
    /// Required if any of the SKUs are for products that have `shippable` set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<UpdateSourceSourceOrderShipping<'a>>,
}
impl<'a> UpdateSourceSourceOrder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// List of items constituting the order.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
    ///
    /// When type is `sku`, this is the number of instances of the SKU to be ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpdateSourceSourceOrderItemsType>,
}
impl<'a> UpdateSourceSourceOrderItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateSourceSourceOrderItemsType {
    Discount,
    Shipping,
    Sku,
    Tax,
}

impl UpdateSourceSourceOrderItemsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Discount => "discount",
            Self::Shipping => "shipping",
            Self::Sku => "sku",
            Self::Tax => "tax",
        }
    }
}

impl std::str::FromStr for UpdateSourceSourceOrderItemsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "discount" => Ok(Self::Discount),
            "shipping" => Ok(Self::Shipping),
            "sku" => Ok(Self::Sku),
            "tax" => Ok(Self::Tax),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateSourceSourceOrderItemsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSourceSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// Shipping address for the order.
///
/// Required if any of the SKUs are for products that have `shippable` set to true.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSourceSourceOrderShipping<'a> {
    /// Shipping address.
    pub address: UpdateSourceSourceOrderShippingAddress<'a>,
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
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<&'a str>,
}
impl<'a> UpdateSourceSourceOrderShipping<'a> {
    pub fn new(address: UpdateSourceSourceOrderShippingAddress<'a>) -> Self {
        Self {
            address,
            carrier: Default::default(),
            name: Default::default(),
            phone: Default::default(),
            tracking_number: Default::default(),
        }
    }
}
/// Shipping address.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSourceSourceOrderShippingAddress<'a> {
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
impl<'a> UpdateSourceSourceOrderShippingAddress<'a> {
    pub fn new(line1: &'a str) -> Self {
        Self {
            city: Default::default(),
            country: Default::default(),
            line1,
            line2: Default::default(),
            postal_code: Default::default(),
            state: Default::default(),
        }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct VerifySource<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The values needed to verify the source.
    pub values: &'a [&'a str],
}
impl<'a> VerifySource<'a> {
    pub fn new(values: &'a [&'a str]) -> Self {
        Self { expand: Default::default(), values }
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceTransactionsSource<'a> {
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
impl<'a> SourceTransactionsSource<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
