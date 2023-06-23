use stripe::{Client, Response};

impl stripe_treasury::treasury::inbound_transfer::InboundTransfer {
    /// Cancels an InboundTransfer.
    pub fn cancel(
        client: &Client,
        inbound_transfer: &str,
        params: CancelInboundTransfer,
    ) -> Response<stripe_treasury::treasury::inbound_transfer::InboundTransfer> {
        client.send_form(
            &format!(
                "/treasury/inbound_transfers/{inbound_transfer}/cancel",
                inbound_transfer = inbound_transfer
            ),
            params,
            http_types::Method::Post,
        )
    }
    /// Creates an InboundTransfer.
    pub fn create(
        client: &Client,
        params: CreateInboundTransfer,
    ) -> Response<stripe_treasury::treasury::inbound_transfer::InboundTransfer> {
        client.send_form("/treasury/inbound_transfers", params, http_types::Method::Post)
    }
    /// Retrieves the details of an existing InboundTransfer.
    pub fn retrieve(
        client: &Client,
        id: &str,
        params: RetrieveInboundTransfer,
    ) -> Response<stripe_treasury::treasury::inbound_transfer::InboundTransfer> {
        client.get_query(&format!("/treasury/inbound_transfers/{id}", id = id), params)
    }
    /// Returns a list of InboundTransfers sent from the specified FinancialAccount.
    pub fn list(
        client: &Client,
        params: ListInboundTransfer,
    ) -> Response<stripe_types::List<stripe_treasury::treasury::inbound_transfer::InboundTransfer>>
    {
        client.get_query("/treasury/inbound_transfers", params)
    }
    /// Transitions a test mode created InboundTransfer to the `succeeded` status.
    ///
    /// The InboundTransfer must already be in the `processing` state.
    pub fn succeed(
        client: &Client,
        id: &str,
        params: SucceedInboundTransfer,
    ) -> Response<stripe_treasury::treasury::inbound_transfer::InboundTransfer> {
        client.send_form(
            &format!("/test_helpers/treasury/inbound_transfers/{id}/succeed", id = id),
            params,
            http_types::Method::Post,
        )
    }
    /// Transitions a test mode created InboundTransfer to the `failed` status.
    ///
    /// The InboundTransfer must already be in the `processing` state.
    pub fn fail(
        client: &Client,
        id: &str,
        params: FailInboundTransfer,
    ) -> Response<stripe_treasury::treasury::inbound_transfer::InboundTransfer> {
        client.send_form(
            &format!("/test_helpers/treasury/inbound_transfers/{id}/fail", id = id),
            params,
            http_types::Method::Post,
        )
    }
    /// Marks the test mode InboundTransfer object as returned and links the InboundTransfer to a ReceivedDebit.
    ///
    /// The InboundTransfer must already be in the `succeeded` state.
    pub fn return_inbound_transfer(
        client: &Client,
        id: &str,
        params: ReturnInboundTransferInboundTransfer,
    ) -> Response<stripe_treasury::treasury::inbound_transfer::InboundTransfer> {
        client.send_form(
            &format!("/test_helpers/treasury/inbound_transfers/{id}/return", id = id),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelInboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelInboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInboundTransfer<'a> {
    /// Amount (in cents) to be transferred.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The FinancialAccount to send funds to.
    pub financial_account: &'a str,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The origin payment method to be debited for the InboundTransfer.
    pub origin_payment_method: &'a str,
    /// The complete description that appears on your customers' statements.
    ///
    /// Maximum 10 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}
impl<'a> CreateInboundTransfer<'a> {
    pub fn new(
        amount: i64,
        currency: stripe_types::Currency,
        financial_account: &'a str,
        origin_payment_method: &'a str,
    ) -> Self {
        Self {
            amount,
            currency,
            description: Default::default(),
            expand: Default::default(),
            financial_account,
            metadata: Default::default(),
            origin_payment_method,
            statement_descriptor: Default::default(),
        }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveInboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveInboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListInboundTransfer<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Returns objects associated with this FinancialAccount.
    pub financial_account: &'a str,
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
    /// Only return InboundTransfers that have the given status: `processing`, `succeeded`, `failed` or `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListInboundTransferStatus>,
}
impl<'a> ListInboundTransfer<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self {
            ending_before: Default::default(),
            expand: Default::default(),
            financial_account,
            limit: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}
/// Only return InboundTransfers that have the given status: `processing`, `succeeded`, `failed` or `canceled`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListInboundTransferStatus {
    Canceled,
    Failed,
    Processing,
    Succeeded,
}

impl ListInboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Failed => "failed",
            Self::Processing => "processing",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for ListInboundTransferStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListInboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct SucceedInboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> SucceedInboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct FailInboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Details about a failed InboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<FailInboundTransferFailureDetails>,
}
impl<'a> FailInboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details about a failed InboundTransfer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct FailInboundTransferFailureDetails {
    /// Reason for the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<FailInboundTransferFailureDetailsCode>,
}
impl FailInboundTransferFailureDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Reason for the failure.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FailInboundTransferFailureDetailsCode {
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
}

impl FailInboundTransferFailureDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountClosed => "account_closed",
            Self::AccountFrozen => "account_frozen",
            Self::BankAccountRestricted => "bank_account_restricted",
            Self::BankOwnershipChanged => "bank_ownership_changed",
            Self::DebitNotAuthorized => "debit_not_authorized",
            Self::IncorrectAccountHolderAddress => "incorrect_account_holder_address",
            Self::IncorrectAccountHolderName => "incorrect_account_holder_name",
            Self::IncorrectAccountHolderTaxId => "incorrect_account_holder_tax_id",
            Self::InsufficientFunds => "insufficient_funds",
            Self::InvalidAccountNumber => "invalid_account_number",
            Self::InvalidCurrency => "invalid_currency",
            Self::NoAccount => "no_account",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for FailInboundTransferFailureDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FailInboundTransferFailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnInboundTransferInboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ReturnInboundTransferInboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
