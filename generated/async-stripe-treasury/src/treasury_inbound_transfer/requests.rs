use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListTreasuryInboundTransferBuilder {
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
    status: Option<stripe_treasury::TreasuryInboundTransferStatus>,
}
impl ListTreasuryInboundTransferBuilder {
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
/// Returns a list of InboundTransfers sent from the specified FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryInboundTransfer {
    inner: ListTreasuryInboundTransferBuilder,
}
impl ListTreasuryInboundTransfer {
    /// Construct a new `ListTreasuryInboundTransfer`.
    pub fn new(financial_account: impl Into<String>) -> Self {
        Self { inner: ListTreasuryInboundTransferBuilder::new(financial_account.into()) }
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
    /// Only return InboundTransfers that have the given status: `processing`, `succeeded`, `failed` or `canceled`.
    pub fn status(
        mut self,
        status: impl Into<stripe_treasury::TreasuryInboundTransferStatus>,
    ) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl ListTreasuryInboundTransfer {
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
        stripe_types::List<stripe_treasury::TreasuryInboundTransfer>,
    > {
        stripe_client_core::ListPaginator::new_list("/treasury/inbound_transfers", &self.inner)
    }
}

impl StripeRequest for ListTreasuryInboundTransfer {
    type Output = stripe_types::List<stripe_treasury::TreasuryInboundTransfer>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/inbound_transfers").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryInboundTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTreasuryInboundTransferBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing InboundTransfer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryInboundTransfer {
    inner: RetrieveTreasuryInboundTransferBuilder,
    id: stripe_treasury::TreasuryInboundTransferId,
}
impl RetrieveTreasuryInboundTransfer {
    /// Construct a new `RetrieveTreasuryInboundTransfer`.
    pub fn new(id: impl Into<stripe_treasury::TreasuryInboundTransferId>) -> Self {
        Self { id: id.into(), inner: RetrieveTreasuryInboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTreasuryInboundTransfer {
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

impl StripeRequest for RetrieveTreasuryInboundTransfer {
    type Output = stripe_treasury::TreasuryInboundTransfer;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/treasury/inbound_transfers/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct FailTreasuryInboundTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_details: Option<FailTreasuryInboundTransferFailureDetails>,
}
impl FailTreasuryInboundTransferBuilder {
    fn new() -> Self {
        Self { expand: None, failure_details: None }
    }
}
/// Details about a failed InboundTransfer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct FailTreasuryInboundTransferFailureDetails {
    /// Reason for the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<FailTreasuryInboundTransferFailureDetailsCode>,
}
impl FailTreasuryInboundTransferFailureDetails {
    pub fn new() -> Self {
        Self { code: None }
    }
}
impl Default for FailTreasuryInboundTransferFailureDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Reason for the failure.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FailTreasuryInboundTransferFailureDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    DebitNotAuthorized,
    IncorrectAccountHolderAddress,
    IncorrectAccountHolderName,
    IncorrectAccountHolderTaxId,
    InsufficientFunds,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FailTreasuryInboundTransferFailureDetailsCode {
    pub fn as_str(&self) -> &str {
        use FailTreasuryInboundTransferFailureDetailsCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            BankAccountRestricted => "bank_account_restricted",
            BankOwnershipChanged => "bank_ownership_changed",
            DebitNotAuthorized => "debit_not_authorized",
            IncorrectAccountHolderAddress => "incorrect_account_holder_address",
            IncorrectAccountHolderName => "incorrect_account_holder_name",
            IncorrectAccountHolderTaxId => "incorrect_account_holder_tax_id",
            InsufficientFunds => "insufficient_funds",
            InvalidAccountNumber => "invalid_account_number",
            InvalidCurrency => "invalid_currency",
            NoAccount => "no_account",
            Other => "other",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FailTreasuryInboundTransferFailureDetailsCode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FailTreasuryInboundTransferFailureDetailsCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "bank_account_restricted" => Ok(BankAccountRestricted),
            "bank_ownership_changed" => Ok(BankOwnershipChanged),
            "debit_not_authorized" => Ok(DebitNotAuthorized),
            "incorrect_account_holder_address" => Ok(IncorrectAccountHolderAddress),
            "incorrect_account_holder_name" => Ok(IncorrectAccountHolderName),
            "incorrect_account_holder_tax_id" => Ok(IncorrectAccountHolderTaxId),
            "insufficient_funds" => Ok(InsufficientFunds),
            "invalid_account_number" => Ok(InvalidAccountNumber),
            "invalid_currency" => Ok(InvalidCurrency),
            "no_account" => Ok(NoAccount),
            "other" => Ok(Other),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for FailTreasuryInboundTransferFailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FailTreasuryInboundTransferFailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FailTreasuryInboundTransferFailureDetailsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FailTreasuryInboundTransferFailureDetailsCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Transitions a test mode created InboundTransfer to the `failed` status.
/// The InboundTransfer must already be in the `processing` state.
#[derive(Clone, Debug, serde::Serialize)]
pub struct FailTreasuryInboundTransfer {
    inner: FailTreasuryInboundTransferBuilder,
    id: String,
}
impl FailTreasuryInboundTransfer {
    /// Construct a new `FailTreasuryInboundTransfer`.
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into(), inner: FailTreasuryInboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Details about a failed InboundTransfer.
    pub fn failure_details(
        mut self,
        failure_details: impl Into<FailTreasuryInboundTransferFailureDetails>,
    ) -> Self {
        self.inner.failure_details = Some(failure_details.into());
        self
    }
}
impl FailTreasuryInboundTransfer {
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

impl StripeRequest for FailTreasuryInboundTransfer {
    type Output = stripe_treasury::TreasuryInboundTransfer;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/inbound_transfers/{id}/fail"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ReturnInboundTransferTreasuryInboundTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl ReturnInboundTransferTreasuryInboundTransferBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Marks the test mode InboundTransfer object as returned and links the InboundTransfer to a ReceivedDebit.
/// The InboundTransfer must already be in the `succeeded` state.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReturnInboundTransferTreasuryInboundTransfer {
    inner: ReturnInboundTransferTreasuryInboundTransferBuilder,
    id: String,
}
impl ReturnInboundTransferTreasuryInboundTransfer {
    /// Construct a new `ReturnInboundTransferTreasuryInboundTransfer`.
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into(), inner: ReturnInboundTransferTreasuryInboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ReturnInboundTransferTreasuryInboundTransfer {
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

impl StripeRequest for ReturnInboundTransferTreasuryInboundTransfer {
    type Output = stripe_treasury::TreasuryInboundTransfer;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/inbound_transfers/{id}/return"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct SucceedTreasuryInboundTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl SucceedTreasuryInboundTransferBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Transitions a test mode created InboundTransfer to the `succeeded` status.
/// The InboundTransfer must already be in the `processing` state.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SucceedTreasuryInboundTransfer {
    inner: SucceedTreasuryInboundTransferBuilder,
    id: String,
}
impl SucceedTreasuryInboundTransfer {
    /// Construct a new `SucceedTreasuryInboundTransfer`.
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into(), inner: SucceedTreasuryInboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl SucceedTreasuryInboundTransfer {
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

impl StripeRequest for SucceedTreasuryInboundTransfer {
    type Output = stripe_treasury::TreasuryInboundTransfer;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/inbound_transfers/{id}/succeed"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateTreasuryInboundTransferBuilder {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    origin_payment_method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<String>,
}
impl CreateTreasuryInboundTransferBuilder {
    fn new(
        amount: impl Into<i64>,
        currency: impl Into<stripe_types::Currency>,
        financial_account: impl Into<String>,
        origin_payment_method: impl Into<String>,
    ) -> Self {
        Self {
            amount: amount.into(),
            currency: currency.into(),
            description: None,
            expand: None,
            financial_account: financial_account.into(),
            metadata: None,
            origin_payment_method: origin_payment_method.into(),
            statement_descriptor: None,
        }
    }
}
/// Creates an InboundTransfer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryInboundTransfer {
    inner: CreateTreasuryInboundTransferBuilder,
}
impl CreateTreasuryInboundTransfer {
    /// Construct a new `CreateTreasuryInboundTransfer`.
    pub fn new(
        amount: impl Into<i64>,
        currency: impl Into<stripe_types::Currency>,
        financial_account: impl Into<String>,
        origin_payment_method: impl Into<String>,
    ) -> Self {
        Self {
            inner: CreateTreasuryInboundTransferBuilder::new(
                amount.into(),
                currency.into(),
                financial_account.into(),
                origin_payment_method.into(),
            ),
        }
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
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
    /// The complete description that appears on your customers' statements. Maximum 10 characters.
    pub fn statement_descriptor(mut self, statement_descriptor: impl Into<String>) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor.into());
        self
    }
}
impl CreateTreasuryInboundTransfer {
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

impl StripeRequest for CreateTreasuryInboundTransfer {
    type Output = stripe_treasury::TreasuryInboundTransfer;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/treasury/inbound_transfers").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CancelTreasuryInboundTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CancelTreasuryInboundTransferBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Cancels an InboundTransfer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelTreasuryInboundTransfer {
    inner: CancelTreasuryInboundTransferBuilder,
    inbound_transfer: stripe_treasury::TreasuryInboundTransferId,
}
impl CancelTreasuryInboundTransfer {
    /// Construct a new `CancelTreasuryInboundTransfer`.
    pub fn new(inbound_transfer: impl Into<stripe_treasury::TreasuryInboundTransferId>) -> Self {
        Self {
            inbound_transfer: inbound_transfer.into(),
            inner: CancelTreasuryInboundTransferBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CancelTreasuryInboundTransfer {
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

impl StripeRequest for CancelTreasuryInboundTransfer {
    type Output = stripe_treasury::TreasuryInboundTransfer;

    fn build(&self) -> RequestBuilder {
        let inbound_transfer = &self.inbound_transfer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/treasury/inbound_transfers/{inbound_transfer}/cancel"),
        )
        .form(&self.inner)
    }
}
