use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListTreasuryOutboundTransferBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    financial_account: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_treasury::TreasuryOutboundTransferStatus>,
}
impl<'a> ListTreasuryOutboundTransferBuilder<'a> {
    fn new(financial_account: &'a str) -> Self {
        Self {
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Returns a list of OutboundTransfers sent from the specified FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryOutboundTransfer<'a> {
    inner: ListTreasuryOutboundTransferBuilder<'a>,
}
impl<'a> ListTreasuryOutboundTransfer<'a> {
    /// Construct a new `ListTreasuryOutboundTransfer`.
    pub fn new(financial_account: &'a str) -> Self {
        Self { inner: ListTreasuryOutboundTransferBuilder::new(financial_account) }
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
    /// Only return OutboundTransfers that have the given status: `processing`, `canceled`, `failed`, `posted`, or `returned`.
    pub fn status(mut self, status: stripe_treasury::TreasuryOutboundTransferStatus) -> Self {
        self.inner.status = Some(status);
        self
    }
}
impl ListTreasuryOutboundTransfer<'_> {
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
        stripe_types::List<stripe_treasury::TreasuryOutboundTransfer>,
    > {
        stripe_client_core::ListPaginator::new_list("/treasury/outbound_transfers", self.inner)
    }
}

impl StripeRequest for ListTreasuryOutboundTransfer<'_> {
    type Output = stripe_types::List<stripe_treasury::TreasuryOutboundTransfer>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/outbound_transfers").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryOutboundTransferBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryOutboundTransferBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing OutboundTransfer by passing the unique OutboundTransfer ID from either the OutboundTransfer creation request or OutboundTransfer list.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryOutboundTransfer<'a> {
    inner: RetrieveTreasuryOutboundTransferBuilder<'a>,
    outbound_transfer: &'a stripe_treasury::TreasuryOutboundTransferId,
}
impl<'a> RetrieveTreasuryOutboundTransfer<'a> {
    /// Construct a new `RetrieveTreasuryOutboundTransfer`.
    pub fn new(outbound_transfer: &'a stripe_treasury::TreasuryOutboundTransferId) -> Self {
        Self { outbound_transfer, inner: RetrieveTreasuryOutboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTreasuryOutboundTransfer<'_> {
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

impl StripeRequest for RetrieveTreasuryOutboundTransfer<'_> {
    type Output = stripe_treasury::TreasuryOutboundTransfer;

    fn build(&self) -> RequestBuilder {
        let outbound_transfer = self.outbound_transfer;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/treasury/outbound_transfers/{outbound_transfer}"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct FailTreasuryOutboundTransferBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> FailTreasuryOutboundTransferBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Transitions a test mode created OutboundTransfer to the `failed` status.
/// The OutboundTransfer must already be in the `processing` state.
#[derive(Clone, Debug, serde::Serialize)]
pub struct FailTreasuryOutboundTransfer<'a> {
    inner: FailTreasuryOutboundTransferBuilder<'a>,
    outbound_transfer: &'a str,
}
impl<'a> FailTreasuryOutboundTransfer<'a> {
    /// Construct a new `FailTreasuryOutboundTransfer`.
    pub fn new(outbound_transfer: &'a str) -> Self {
        Self { outbound_transfer, inner: FailTreasuryOutboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl FailTreasuryOutboundTransfer<'_> {
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

impl StripeRequest for FailTreasuryOutboundTransfer<'_> {
    type Output = stripe_treasury::TreasuryOutboundTransfer;

    fn build(&self) -> RequestBuilder {
        let outbound_transfer = self.outbound_transfer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/outbound_transfers/{outbound_transfer}/fail"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct PostTreasuryOutboundTransferBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> PostTreasuryOutboundTransferBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Transitions a test mode created OutboundTransfer to the `posted` status.
/// The OutboundTransfer must already be in the `processing` state.
#[derive(Clone, Debug, serde::Serialize)]
pub struct PostTreasuryOutboundTransfer<'a> {
    inner: PostTreasuryOutboundTransferBuilder<'a>,
    outbound_transfer: &'a str,
}
impl<'a> PostTreasuryOutboundTransfer<'a> {
    /// Construct a new `PostTreasuryOutboundTransfer`.
    pub fn new(outbound_transfer: &'a str) -> Self {
        Self { outbound_transfer, inner: PostTreasuryOutboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl PostTreasuryOutboundTransfer<'_> {
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

impl StripeRequest for PostTreasuryOutboundTransfer<'_> {
    type Output = stripe_treasury::TreasuryOutboundTransfer;

    fn build(&self) -> RequestBuilder {
        let outbound_transfer = self.outbound_transfer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/outbound_transfers/{outbound_transfer}/post"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ReturnOutboundTransferTreasuryOutboundTransferBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    returned_details: Option<ReturnOutboundTransferTreasuryOutboundTransferReturnedDetails>,
}
impl<'a> ReturnOutboundTransferTreasuryOutboundTransferBuilder<'a> {
    fn new() -> Self {
        Self { expand: None, returned_details: None }
    }
}
/// Details about a returned OutboundTransfer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ReturnOutboundTransferTreasuryOutboundTransferReturnedDetails {
    /// Reason for the return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ReturnOutboundTransferTreasuryOutboundTransferReturnedDetailsCode>,
}
impl ReturnOutboundTransferTreasuryOutboundTransferReturnedDetails {
    pub fn new() -> Self {
        Self { code: None }
    }
}
impl Default for ReturnOutboundTransferTreasuryOutboundTransferReturnedDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Reason for the return.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReturnOutboundTransferTreasuryOutboundTransferReturnedDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    Declined,
    IncorrectAccountHolderName,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
}
impl ReturnOutboundTransferTreasuryOutboundTransferReturnedDetailsCode {
    pub fn as_str(self) -> &'static str {
        use ReturnOutboundTransferTreasuryOutboundTransferReturnedDetailsCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            BankAccountRestricted => "bank_account_restricted",
            BankOwnershipChanged => "bank_ownership_changed",
            Declined => "declined",
            IncorrectAccountHolderName => "incorrect_account_holder_name",
            InvalidAccountNumber => "invalid_account_number",
            InvalidCurrency => "invalid_currency",
            NoAccount => "no_account",
            Other => "other",
        }
    }
}

impl std::str::FromStr for ReturnOutboundTransferTreasuryOutboundTransferReturnedDetailsCode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReturnOutboundTransferTreasuryOutboundTransferReturnedDetailsCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "bank_account_restricted" => Ok(BankAccountRestricted),
            "bank_ownership_changed" => Ok(BankOwnershipChanged),
            "declined" => Ok(Declined),
            "incorrect_account_holder_name" => Ok(IncorrectAccountHolderName),
            "invalid_account_number" => Ok(InvalidAccountNumber),
            "invalid_currency" => Ok(InvalidCurrency),
            "no_account" => Ok(NoAccount),
            "other" => Ok(Other),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ReturnOutboundTransferTreasuryOutboundTransferReturnedDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReturnOutboundTransferTreasuryOutboundTransferReturnedDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReturnOutboundTransferTreasuryOutboundTransferReturnedDetailsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ReturnOutboundTransferTreasuryOutboundTransferReturnedDetailsCode
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ReturnOutboundTransferTreasuryOutboundTransferReturnedDetailsCode"))
    }
}
/// Transitions a test mode created OutboundTransfer to the `returned` status.
/// The OutboundTransfer must already be in the `processing` state.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReturnOutboundTransferTreasuryOutboundTransfer<'a> {
    inner: ReturnOutboundTransferTreasuryOutboundTransferBuilder<'a>,
    outbound_transfer: &'a str,
}
impl<'a> ReturnOutboundTransferTreasuryOutboundTransfer<'a> {
    /// Construct a new `ReturnOutboundTransferTreasuryOutboundTransfer`.
    pub fn new(outbound_transfer: &'a str) -> Self {
        Self {
            outbound_transfer,
            inner: ReturnOutboundTransferTreasuryOutboundTransferBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Details about a returned OutboundTransfer.
    pub fn returned_details(
        mut self,
        returned_details: ReturnOutboundTransferTreasuryOutboundTransferReturnedDetails,
    ) -> Self {
        self.inner.returned_details = Some(returned_details);
        self
    }
}
impl ReturnOutboundTransferTreasuryOutboundTransfer<'_> {
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

impl StripeRequest for ReturnOutboundTransferTreasuryOutboundTransfer<'_> {
    type Output = stripe_treasury::TreasuryOutboundTransfer;

    fn build(&self) -> RequestBuilder {
        let outbound_transfer = self.outbound_transfer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/outbound_transfers/{outbound_transfer}/return"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateTreasuryOutboundTransferBuilder<'a> {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_payment_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_payment_method_options:
        Option<CreateTreasuryOutboundTransferDestinationPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    financial_account: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<&'a str>,
}
impl<'a> CreateTreasuryOutboundTransferBuilder<'a> {
    fn new(amount: i64, currency: stripe_types::Currency, financial_account: &'a str) -> Self {
        Self {
            amount,
            currency,
            description: None,
            destination_payment_method: None,
            destination_payment_method_options: None,
            expand: None,
            financial_account,
            metadata: None,
            statement_descriptor: None,
        }
    }
}
/// Hash describing payment method configuration details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryOutboundTransferDestinationPaymentMethodOptions {
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccount>,
}
impl CreateTreasuryOutboundTransferDestinationPaymentMethodOptions {
    pub fn new() -> Self {
        Self { us_bank_account: None }
    }
}
impl Default for CreateTreasuryOutboundTransferDestinationPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccount {
    /// Specifies the network rails to be used.
    /// If not set, will default to the PaymentMethod's preferred network.
    /// See the [docs](https://stripe.com/docs/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network:
        Option<CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork>,
}
impl CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self { network: None }
    }
}
impl Default for CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Specifies the network rails to be used.
/// If not set, will default to the PaymentMethod's preferred network.
/// See the [docs](https://stripe.com/docs/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}
impl CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr
    for CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork"))
    }
}
/// Creates an OutboundTransfer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryOutboundTransfer<'a> {
    inner: CreateTreasuryOutboundTransferBuilder<'a>,
}
impl<'a> CreateTreasuryOutboundTransfer<'a> {
    /// Construct a new `CreateTreasuryOutboundTransfer`.
    pub fn new(amount: i64, currency: stripe_types::Currency, financial_account: &'a str) -> Self {
        Self {
            inner: CreateTreasuryOutboundTransferBuilder::new(amount, currency, financial_account),
        }
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// The PaymentMethod to use as the payment instrument for the OutboundTransfer.
    pub fn destination_payment_method(mut self, destination_payment_method: &'a str) -> Self {
        self.inner.destination_payment_method = Some(destination_payment_method);
        self
    }
    /// Hash describing payment method configuration details.
    pub fn destination_payment_method_options(
        mut self,
        destination_payment_method_options: CreateTreasuryOutboundTransferDestinationPaymentMethodOptions,
    ) -> Self {
        self.inner.destination_payment_method_options = Some(destination_payment_method_options);
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
    /// Statement descriptor to be shown on the receiving end of an OutboundTransfer.
    /// Maximum 10 characters for `ach` transfers or 140 characters for `us_domestic_wire` transfers.
    /// The default value is "transfer".
    pub fn statement_descriptor(mut self, statement_descriptor: &'a str) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor);
        self
    }
}
impl CreateTreasuryOutboundTransfer<'_> {
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

impl StripeRequest for CreateTreasuryOutboundTransfer<'_> {
    type Output = stripe_treasury::TreasuryOutboundTransfer;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/treasury/outbound_transfers").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CancelTreasuryOutboundTransferBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> CancelTreasuryOutboundTransferBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// An OutboundTransfer can be canceled if the funds have not yet been paid out.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelTreasuryOutboundTransfer<'a> {
    inner: CancelTreasuryOutboundTransferBuilder<'a>,
    outbound_transfer: &'a stripe_treasury::TreasuryOutboundTransferId,
}
impl<'a> CancelTreasuryOutboundTransfer<'a> {
    /// Construct a new `CancelTreasuryOutboundTransfer`.
    pub fn new(outbound_transfer: &'a stripe_treasury::TreasuryOutboundTransferId) -> Self {
        Self { outbound_transfer, inner: CancelTreasuryOutboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CancelTreasuryOutboundTransfer<'_> {
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

impl StripeRequest for CancelTreasuryOutboundTransfer<'_> {
    type Output = stripe_treasury::TreasuryOutboundTransfer;

    fn build(&self) -> RequestBuilder {
        let outbound_transfer = self.outbound_transfer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/treasury/outbound_transfers/{outbound_transfer}/cancel"),
        )
        .form(&self.inner)
    }
}
