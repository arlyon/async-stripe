use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListTreasuryInboundTransferBuilder<'a> {
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
    status: Option<stripe_treasury::TreasuryInboundTransferStatus>,
}
impl<'a> ListTreasuryInboundTransferBuilder<'a> {
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
/// Returns a list of InboundTransfers sent from the specified FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryInboundTransfer<'a> {
    inner: ListTreasuryInboundTransferBuilder<'a>,
}
impl<'a> ListTreasuryInboundTransfer<'a> {
    /// Construct a new `ListTreasuryInboundTransfer`.
    pub fn new(financial_account: &'a str) -> Self {
        Self { inner: ListTreasuryInboundTransferBuilder::new(financial_account) }
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
    /// Only return InboundTransfers that have the given status: `processing`, `succeeded`, `failed` or `canceled`.
    pub fn status(mut self, status: stripe_treasury::TreasuryInboundTransferStatus) -> Self {
        self.inner.status = Some(status);
        self
    }
}
impl ListTreasuryInboundTransfer<'_> {
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
        stripe_client_core::ListPaginator::new_list("/treasury/inbound_transfers", self.inner)
    }
}

impl StripeRequest for ListTreasuryInboundTransfer<'_> {
    type Output = stripe_types::List<stripe_treasury::TreasuryInboundTransfer>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/inbound_transfers").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryInboundTransferBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryInboundTransferBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing InboundTransfer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryInboundTransfer<'a> {
    inner: RetrieveTreasuryInboundTransferBuilder<'a>,
    id: &'a stripe_treasury::TreasuryInboundTransferId,
}
impl<'a> RetrieveTreasuryInboundTransfer<'a> {
    /// Construct a new `RetrieveTreasuryInboundTransfer`.
    pub fn new(id: &'a stripe_treasury::TreasuryInboundTransferId) -> Self {
        Self { id, inner: RetrieveTreasuryInboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTreasuryInboundTransfer<'_> {
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

impl StripeRequest for RetrieveTreasuryInboundTransfer<'_> {
    type Output = stripe_treasury::TreasuryInboundTransfer;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/treasury/inbound_transfers/{id}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct FailTreasuryInboundTransferBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_details: Option<FailTreasuryInboundTransferFailureDetails>,
}
impl<'a> FailTreasuryInboundTransferBuilder<'a> {
    fn new() -> Self {
        Self { expand: None, failure_details: None }
    }
}
/// Details about a failed InboundTransfer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Eq, PartialEq)]
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
    Unknown,
}
impl FailTreasuryInboundTransferFailureDetailsCode {
    pub fn as_str(self) -> &'static str {
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
            Unknown => "unknown",
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
            _ => Ok(Self::Unknown),
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
pub struct FailTreasuryInboundTransfer<'a> {
    inner: FailTreasuryInboundTransferBuilder<'a>,
    id: &'a str,
}
impl<'a> FailTreasuryInboundTransfer<'a> {
    /// Construct a new `FailTreasuryInboundTransfer`.
    pub fn new(id: &'a str) -> Self {
        Self { id, inner: FailTreasuryInboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Details about a failed InboundTransfer.
    pub fn failure_details(
        mut self,
        failure_details: FailTreasuryInboundTransferFailureDetails,
    ) -> Self {
        self.inner.failure_details = Some(failure_details);
        self
    }
}
impl FailTreasuryInboundTransfer<'_> {
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

impl StripeRequest for FailTreasuryInboundTransfer<'_> {
    type Output = stripe_treasury::TreasuryInboundTransfer;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/inbound_transfers/{id}/fail"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ReturnInboundTransferTreasuryInboundTransferBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> ReturnInboundTransferTreasuryInboundTransferBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Marks the test mode InboundTransfer object as returned and links the InboundTransfer to a ReceivedDebit.
/// The InboundTransfer must already be in the `succeeded` state.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReturnInboundTransferTreasuryInboundTransfer<'a> {
    inner: ReturnInboundTransferTreasuryInboundTransferBuilder<'a>,
    id: &'a str,
}
impl<'a> ReturnInboundTransferTreasuryInboundTransfer<'a> {
    /// Construct a new `ReturnInboundTransferTreasuryInboundTransfer`.
    pub fn new(id: &'a str) -> Self {
        Self { id, inner: ReturnInboundTransferTreasuryInboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl ReturnInboundTransferTreasuryInboundTransfer<'_> {
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

impl StripeRequest for ReturnInboundTransferTreasuryInboundTransfer<'_> {
    type Output = stripe_treasury::TreasuryInboundTransfer;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/inbound_transfers/{id}/return"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct SucceedTreasuryInboundTransferBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> SucceedTreasuryInboundTransferBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Transitions a test mode created InboundTransfer to the `succeeded` status.
/// The InboundTransfer must already be in the `processing` state.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SucceedTreasuryInboundTransfer<'a> {
    inner: SucceedTreasuryInboundTransferBuilder<'a>,
    id: &'a str,
}
impl<'a> SucceedTreasuryInboundTransfer<'a> {
    /// Construct a new `SucceedTreasuryInboundTransfer`.
    pub fn new(id: &'a str) -> Self {
        Self { id, inner: SucceedTreasuryInboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl SucceedTreasuryInboundTransfer<'_> {
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

impl StripeRequest for SucceedTreasuryInboundTransfer<'_> {
    type Output = stripe_treasury::TreasuryInboundTransfer;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/inbound_transfers/{id}/succeed"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateTreasuryInboundTransferBuilder<'a> {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    financial_account: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    origin_payment_method: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<&'a str>,
}
impl<'a> CreateTreasuryInboundTransferBuilder<'a> {
    fn new(
        amount: i64,
        currency: stripe_types::Currency,
        financial_account: &'a str,
        origin_payment_method: &'a str,
    ) -> Self {
        Self {
            amount,
            currency,
            description: None,
            expand: None,
            financial_account,
            metadata: None,
            origin_payment_method,
            statement_descriptor: None,
        }
    }
}
/// Creates an InboundTransfer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryInboundTransfer<'a> {
    inner: CreateTreasuryInboundTransferBuilder<'a>,
}
impl<'a> CreateTreasuryInboundTransfer<'a> {
    /// Construct a new `CreateTreasuryInboundTransfer`.
    pub fn new(
        amount: i64,
        currency: stripe_types::Currency,
        financial_account: &'a str,
        origin_payment_method: &'a str,
    ) -> Self {
        Self {
            inner: CreateTreasuryInboundTransferBuilder::new(
                amount,
                currency,
                financial_account,
                origin_payment_method,
            ),
        }
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
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
    /// The complete description that appears on your customers' statements. Maximum 10 characters.
    pub fn statement_descriptor(mut self, statement_descriptor: &'a str) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor);
        self
    }
}
impl CreateTreasuryInboundTransfer<'_> {
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

impl StripeRequest for CreateTreasuryInboundTransfer<'_> {
    type Output = stripe_treasury::TreasuryInboundTransfer;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/treasury/inbound_transfers").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CancelTreasuryInboundTransferBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> CancelTreasuryInboundTransferBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Cancels an InboundTransfer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelTreasuryInboundTransfer<'a> {
    inner: CancelTreasuryInboundTransferBuilder<'a>,
    inbound_transfer: &'a stripe_treasury::TreasuryInboundTransferId,
}
impl<'a> CancelTreasuryInboundTransfer<'a> {
    /// Construct a new `CancelTreasuryInboundTransfer`.
    pub fn new(inbound_transfer: &'a stripe_treasury::TreasuryInboundTransferId) -> Self {
        Self { inbound_transfer, inner: CancelTreasuryInboundTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CancelTreasuryInboundTransfer<'_> {
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

impl StripeRequest for CancelTreasuryInboundTransfer<'_> {
    type Output = stripe_treasury::TreasuryInboundTransfer;

    fn build(&self) -> RequestBuilder {
        let inbound_transfer = self.inbound_transfer;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/treasury/inbound_transfers/{inbound_transfer}/cancel"),
        )
        .form(&self.inner)
    }
}
