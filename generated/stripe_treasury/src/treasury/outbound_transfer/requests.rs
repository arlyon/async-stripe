use stripe::{Client, Response};

impl stripe_treasury::treasury::outbound_transfer::OutboundTransfer {
    /// Creates an OutboundTransfer.
    pub fn create(
        client: &Client,
        params: CreateOutboundTransfer,
    ) -> Response<stripe_treasury::treasury::outbound_transfer::OutboundTransfer> {
        client.send_form("/treasury/outbound_transfers", params, http_types::Method::Post)
    }
    /// Retrieves the details of an existing OutboundTransfer by passing the unique OutboundTransfer ID from either the OutboundTransfer creation request or OutboundTransfer list.
    pub fn retrieve(
        client: &Client,
        outbound_transfer: &str,
        params: RetrieveOutboundTransfer,
    ) -> Response<stripe_treasury::treasury::outbound_transfer::OutboundTransfer> {
        client.get_query(
            &format!(
                "/treasury/outbound_transfers/{outbound_transfer}",
                outbound_transfer = outbound_transfer
            ),
            params,
        )
    }
    /// Returns a list of OutboundTransfers sent from the specified FinancialAccount.
    pub fn list(
        client: &Client,
        params: ListOutboundTransfer,
    ) -> Response<stripe_types::List<stripe_treasury::treasury::outbound_transfer::OutboundTransfer>>
    {
        client.get_query("/treasury/outbound_transfers", params)
    }
    /// An OutboundTransfer can be canceled if the funds have not yet been paid out.
    pub fn cancel(
        client: &Client,
        outbound_transfer: &str,
        params: CancelOutboundTransfer,
    ) -> Response<stripe_treasury::treasury::outbound_transfer::OutboundTransfer> {
        client.send_form(
            &format!(
                "/treasury/outbound_transfers/{outbound_transfer}/cancel",
                outbound_transfer = outbound_transfer
            ),
            params,
            http_types::Method::Post,
        )
    }
    /// Transitions a test mode created OutboundTransfer to the `failed` status.
    ///
    /// The OutboundTransfer must already be in the `processing` state.
    pub fn fail(
        client: &Client,
        outbound_transfer: &str,
        params: FailOutboundTransfer,
    ) -> Response<stripe_treasury::treasury::outbound_transfer::OutboundTransfer> {
        client.send_form(
            &format!(
                "/test_helpers/treasury/outbound_transfers/{outbound_transfer}/fail",
                outbound_transfer = outbound_transfer
            ),
            params,
            http_types::Method::Post,
        )
    }
    /// Transitions a test mode created OutboundTransfer to the `posted` status.
    ///
    /// The OutboundTransfer must already be in the `processing` state.
    pub fn post(
        client: &Client,
        outbound_transfer: &str,
        params: PostOutboundTransfer,
    ) -> Response<stripe_treasury::treasury::outbound_transfer::OutboundTransfer> {
        client.send_form(
            &format!(
                "/test_helpers/treasury/outbound_transfers/{outbound_transfer}/post",
                outbound_transfer = outbound_transfer
            ),
            params,
            http_types::Method::Post,
        )
    }
    /// Transitions a test mode created OutboundTransfer to the `returned` status.
    ///
    /// The OutboundTransfer must already be in the `processing` state.
    pub fn return_outbound_transfer(
        client: &Client,
        outbound_transfer: &str,
        params: ReturnOutboundTransferOutboundTransfer,
    ) -> Response<stripe_treasury::treasury::outbound_transfer::OutboundTransfer> {
        client.send_form(
            &format!(
                "/test_helpers/treasury/outbound_transfers/{outbound_transfer}/return",
                outbound_transfer = outbound_transfer
            ),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOutboundTransfer<'a> {
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
    /// The PaymentMethod to use as the payment instrument for the OutboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method: Option<&'a str>,
    /// Hash describing payment method configuration details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method_options:
        Option<CreateOutboundTransferDestinationPaymentMethodOptions>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The FinancialAccount to pull funds from.
    pub financial_account: &'a str,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// Statement descriptor to be shown on the receiving end of an OutboundTransfer.
    ///
    /// Maximum 10 characters for `ach` transfers or 140 characters for `wire` transfers.
    /// The default value is `transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}
impl<'a> CreateOutboundTransfer<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency, financial_account: &'a str) -> Self {
        Self {
            amount,
            currency,
            description: Default::default(),
            destination_payment_method: Default::default(),
            destination_payment_method_options: Default::default(),
            expand: Default::default(),
            financial_account,
            metadata: Default::default(),
            statement_descriptor: Default::default(),
        }
    }
}
/// Hash describing payment method configuration details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOutboundTransferDestinationPaymentMethodOptions {
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateOutboundTransferDestinationPaymentMethodOptionsUsBankAccount>,
}
impl CreateOutboundTransferDestinationPaymentMethodOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOutboundTransferDestinationPaymentMethodOptionsUsBankAccount {
    /// Designate the OutboundTransfer as using a US bank account network configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<CreateOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork>,
}
impl CreateOutboundTransferDestinationPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Designate the OutboundTransfer as using a US bank account network configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}

impl CreateOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for CreateOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListOutboundTransfer<'a> {
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
    /// Only return OutboundTransfers that have the given status: `processing`, `canceled`, `failed`, `posted`, or `returned`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListOutboundTransferStatus>,
}
impl<'a> ListOutboundTransfer<'a> {
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
/// Only return OutboundTransfers that have the given status: `processing`, `canceled`, `failed`, `posted`, or `returned`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListOutboundTransferStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl ListOutboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Failed => "failed",
            Self::Posted => "posted",
            Self::Processing => "processing",
            Self::Returned => "returned",
        }
    }
}

impl AsRef<str> for ListOutboundTransferStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListOutboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct FailOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> FailOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PostOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> PostOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnOutboundTransferOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Details about a returned OutboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_details: Option<ReturnOutboundTransferOutboundTransferReturnedDetails>,
}
impl<'a> ReturnOutboundTransferOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details about a returned OutboundTransfer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnOutboundTransferOutboundTransferReturnedDetails {
    /// Reason for the return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ReturnOutboundTransferOutboundTransferReturnedDetailsCode>,
}
impl ReturnOutboundTransferOutboundTransferReturnedDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Reason for the return.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnOutboundTransferOutboundTransferReturnedDetailsCode {
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

impl ReturnOutboundTransferOutboundTransferReturnedDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountClosed => "account_closed",
            Self::AccountFrozen => "account_frozen",
            Self::BankAccountRestricted => "bank_account_restricted",
            Self::BankOwnershipChanged => "bank_ownership_changed",
            Self::Declined => "declined",
            Self::IncorrectAccountHolderName => "incorrect_account_holder_name",
            Self::InvalidAccountNumber => "invalid_account_number",
            Self::InvalidCurrency => "invalid_currency",
            Self::NoAccount => "no_account",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for ReturnOutboundTransferOutboundTransferReturnedDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReturnOutboundTransferOutboundTransferReturnedDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
