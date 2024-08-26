use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListTreasuryOutboundTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_treasury::TreasuryOutboundTransferStatus>,
}
impl ListTreasuryOutboundTransferBuilder {
    fn new(financial_account: impl Into<String>) -> Self {
        Self {
            ending_before: None,
            expand: None,
            financial_account: financial_account.into(),
            limit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Returns a list of OutboundTransfers sent from the specified FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryOutboundTransfer {
    inner: ListTreasuryOutboundTransferBuilder,
}
impl ListTreasuryOutboundTransfer {
    /// Construct a new `ListTreasuryOutboundTransfer`.
    pub fn new(financial_account: impl Into<String>) -> Self {
        Self { inner: ListTreasuryOutboundTransferBuilder::new(financial_account.into()) }
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
    /// Only return OutboundTransfers that have the given status: `processing`, `canceled`, `failed`, `posted`, or `returned`.
    pub fn status(
        mut self,
        status: impl Into<stripe_treasury::TreasuryOutboundTransferStatus>,
    ) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl ListTreasuryOutboundTransfer {
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
        stripe_client_core::ListPaginator::new_list("/treasury/outbound_transfers", &self.inner)
    }
}

impl StripeRequest for ListTreasuryOutboundTransfer {
    type Output = stripe_types::List<stripe_treasury::TreasuryOutboundTransfer>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/outbound_transfers").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryOutboundTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTreasuryOutboundTransferBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing OutboundTransfer by passing the unique OutboundTransfer ID from either the OutboundTransfer creation request or OutboundTransfer list.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryOutboundTransfer {
    inner: RetrieveTreasuryOutboundTransferBuilder,
    outbound_transfer: stripe_treasury::TreasuryOutboundTransferId,
}
impl RetrieveTreasuryOutboundTransfer {
    /// Construct a new `RetrieveTreasuryOutboundTransfer`.
    pub fn new(outbound_transfer: impl Into<stripe_treasury::TreasuryOutboundTransferId>) -> Self {
        Self {
            outbound_transfer: outbound_transfer.into(),
            inner: RetrieveTreasuryOutboundTransferBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTreasuryOutboundTransfer {
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

impl StripeRequest for RetrieveTreasuryOutboundTransfer {
    type Output = stripe_treasury::TreasuryOutboundTransfer;

    fn build(&self) -> RequestBuilder {
        let outbound_transfer = &self.outbound_transfer;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/treasury/outbound_transfers/{outbound_transfer}"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct FailTreasuryOutboundTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl FailTreasuryOutboundTransferBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Transitions a test mode created OutboundTransfer to the `failed` status.
/// The OutboundTransfer must already be in the `processing` state.
#[derive(Clone, Debug, serde::Serialize)]
pub struct FailTreasuryOutboundTransfer {
    inner: FailTreasuryOutboundTransferBuilder,
    outbound_transfer: String,
}
impl FailTreasuryOutboundTransfer {
    /// Construct a new `FailTreasuryOutboundTransfer`.
    pub fn new(outbound_transfer: impl Into<String>) -> Self {
        Self {
            outbound_transfer: outbound_transfer.into(),
            inner: FailTreasuryOutboundTransferBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl FailTreasuryOutboundTransfer {
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

impl StripeRequest for FailTreasuryOutboundTransfer {
    type Output = stripe_treasury::TreasuryOutboundTransfer;

    fn build(&self) -> RequestBuilder {
        let outbound_transfer = &self.outbound_transfer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/outbound_transfers/{outbound_transfer}/fail"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct PostTreasuryOutboundTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl PostTreasuryOutboundTransferBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Transitions a test mode created OutboundTransfer to the `posted` status.
/// The OutboundTransfer must already be in the `processing` state.
#[derive(Clone, Debug, serde::Serialize)]
pub struct PostTreasuryOutboundTransfer {
    inner: PostTreasuryOutboundTransferBuilder,
    outbound_transfer: String,
}
impl PostTreasuryOutboundTransfer {
    /// Construct a new `PostTreasuryOutboundTransfer`.
    pub fn new(outbound_transfer: impl Into<String>) -> Self {
        Self {
            outbound_transfer: outbound_transfer.into(),
            inner: PostTreasuryOutboundTransferBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl PostTreasuryOutboundTransfer {
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

impl StripeRequest for PostTreasuryOutboundTransfer {
    type Output = stripe_treasury::TreasuryOutboundTransfer;

    fn build(&self) -> RequestBuilder {
        let outbound_transfer = &self.outbound_transfer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/outbound_transfers/{outbound_transfer}/post"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ReturnOutboundTransferTreasuryOutboundTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    returned_details: Option<ReturnOutboundTransferTreasuryOutboundTransferReturnedDetails>,
}
impl ReturnOutboundTransferTreasuryOutboundTransferBuilder {
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
pub struct ReturnOutboundTransferTreasuryOutboundTransfer {
    inner: ReturnOutboundTransferTreasuryOutboundTransferBuilder,
    outbound_transfer: String,
}
impl ReturnOutboundTransferTreasuryOutboundTransfer {
    /// Construct a new `ReturnOutboundTransferTreasuryOutboundTransfer`.
    pub fn new(outbound_transfer: impl Into<String>) -> Self {
        Self {
            outbound_transfer: outbound_transfer.into(),
            inner: ReturnOutboundTransferTreasuryOutboundTransferBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Details about a returned OutboundTransfer.
    pub fn returned_details(
        mut self,
        returned_details: impl Into<ReturnOutboundTransferTreasuryOutboundTransferReturnedDetails>,
    ) -> Self {
        self.inner.returned_details = Some(returned_details.into());
        self
    }
}
impl ReturnOutboundTransferTreasuryOutboundTransfer {
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

impl StripeRequest for ReturnOutboundTransferTreasuryOutboundTransfer {
    type Output = stripe_treasury::TreasuryOutboundTransfer;

    fn build(&self) -> RequestBuilder {
        let outbound_transfer = &self.outbound_transfer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/outbound_transfers/{outbound_transfer}/return"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateTreasuryOutboundTransferBuilder {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_payment_method_options:
        Option<CreateTreasuryOutboundTransferDestinationPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<String>,
}
impl CreateTreasuryOutboundTransferBuilder {
    fn new(
        amount: impl Into<i64>,
        currency: impl Into<stripe_types::Currency>,
        financial_account: impl Into<String>,
    ) -> Self {
        Self {
            amount: amount.into(),
            currency: currency.into(),
            description: None,
            destination_payment_method: None,
            destination_payment_method_options: None,
            expand: None,
            financial_account: financial_account.into(),
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
pub struct CreateTreasuryOutboundTransfer {
    inner: CreateTreasuryOutboundTransferBuilder,
}
impl CreateTreasuryOutboundTransfer {
    /// Construct a new `CreateTreasuryOutboundTransfer`.
    pub fn new(
        amount: impl Into<i64>,
        currency: impl Into<stripe_types::Currency>,
        financial_account: impl Into<String>,
    ) -> Self {
        Self {
            inner: CreateTreasuryOutboundTransferBuilder::new(
                amount.into(),
                currency.into(),
                financial_account.into(),
            ),
        }
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// The PaymentMethod to use as the payment instrument for the OutboundTransfer.
    pub fn destination_payment_method(
        mut self,
        destination_payment_method: impl Into<String>,
    ) -> Self {
        self.inner.destination_payment_method = Some(destination_payment_method.into());
        self
    }
    /// Hash describing payment method configuration details.
    pub fn destination_payment_method_options(
        mut self,
        destination_payment_method_options: impl Into<
            CreateTreasuryOutboundTransferDestinationPaymentMethodOptions,
        >,
    ) -> Self {
        self.inner.destination_payment_method_options =
            Some(destination_payment_method_options.into());
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
    /// Statement descriptor to be shown on the receiving end of an OutboundTransfer.
    /// Maximum 10 characters for `ach` transfers or 140 characters for `us_domestic_wire` transfers.
    /// The default value is "transfer".
    pub fn statement_descriptor(mut self, statement_descriptor: impl Into<String>) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor.into());
        self
    }
}
impl CreateTreasuryOutboundTransfer {
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

impl StripeRequest for CreateTreasuryOutboundTransfer {
    type Output = stripe_treasury::TreasuryOutboundTransfer;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/treasury/outbound_transfers").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CancelTreasuryOutboundTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CancelTreasuryOutboundTransferBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// An OutboundTransfer can be canceled if the funds have not yet been paid out.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelTreasuryOutboundTransfer {
    inner: CancelTreasuryOutboundTransferBuilder,
    outbound_transfer: stripe_treasury::TreasuryOutboundTransferId,
}
impl CancelTreasuryOutboundTransfer {
    /// Construct a new `CancelTreasuryOutboundTransfer`.
    pub fn new(outbound_transfer: impl Into<stripe_treasury::TreasuryOutboundTransferId>) -> Self {
        Self {
            outbound_transfer: outbound_transfer.into(),
            inner: CancelTreasuryOutboundTransferBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CancelTreasuryOutboundTransfer {
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

impl StripeRequest for CancelTreasuryOutboundTransfer {
    type Output = stripe_treasury::TreasuryOutboundTransfer;

    fn build(&self) -> RequestBuilder {
        let outbound_transfer = &self.outbound_transfer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/treasury/outbound_transfers/{outbound_transfer}/cancel"),
        )
        .form(&self.inner)
    }
}
