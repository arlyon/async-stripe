#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelTreasuryInboundTransfersResourceInboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelTreasuryInboundTransfersResourceInboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CancelTreasuryInboundTransfersResourceInboundTransfer<'a> {
    /// Cancels an InboundTransfer.
    pub fn send(&self, client: &stripe::Client, inbound_transfer: &stripe_treasury::treasury_inbound_transfers_resource_inbound_transfer::TreasuryInboundTransferId) -> stripe::Response<stripe_treasury::TreasuryInboundTransfersResourceInboundTransfer> {
        client.send_form(&format!("/treasury/inbound_transfers/{inbound_transfer}/cancel", inbound_transfer = inbound_transfer), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryInboundTransfersResourceInboundTransfer<'a> {
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
impl<'a> CreateTreasuryInboundTransfersResourceInboundTransfer<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency, financial_account: &'a str, origin_payment_method: &'a str) -> Self {
        Self { amount, currency, description: Default::default(), expand: Default::default(), financial_account, metadata: Default::default(), origin_payment_method, statement_descriptor: Default::default() }
    }
}
impl<'a> CreateTreasuryInboundTransfersResourceInboundTransfer<'a> {
    /// Creates an InboundTransfer.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_treasury::TreasuryInboundTransfersResourceInboundTransfer> {
        client.send_form("/treasury/inbound_transfers", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryInboundTransfersResourceInboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryInboundTransfersResourceInboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryInboundTransfersResourceInboundTransfer<'a> {
    /// Retrieves the details of an existing InboundTransfer.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_inbound_transfers_resource_inbound_transfer::TreasuryInboundTransferId) -> stripe::Response<stripe_treasury::TreasuryInboundTransfersResourceInboundTransfer> {
        client.get_query(&format!("/treasury/inbound_transfers/{id}", id = id), self)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryInboundTransfersResourceInboundTransfer<'a> {
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
    pub status: Option<ListTreasuryInboundTransfersResourceInboundTransferStatus>,
}
impl<'a> ListTreasuryInboundTransfersResourceInboundTransfer<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self { ending_before: Default::default(), expand: Default::default(), financial_account, limit: Default::default(), starting_after: Default::default(), status: Default::default() }
    }
}
/// Only return InboundTransfers that have the given status: `processing`, `succeeded`, `failed` or `canceled`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryInboundTransfersResourceInboundTransferStatus {
    Canceled,
    Failed,
    Processing,
    Succeeded,
}

impl ListTreasuryInboundTransfersResourceInboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryInboundTransfersResourceInboundTransferStatus::*;
        match self {
            Canceled => "canceled",
            Failed => "failed",
            Processing => "processing",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for ListTreasuryInboundTransfersResourceInboundTransferStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryInboundTransfersResourceInboundTransferStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "failed" => Ok(Failed),
            "processing" => Ok(Processing),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListTreasuryInboundTransfersResourceInboundTransferStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTreasuryInboundTransfersResourceInboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryInboundTransfersResourceInboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryInboundTransfersResourceInboundTransferStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ListTreasuryInboundTransfersResourceInboundTransfer<'a> {
    /// Returns a list of InboundTransfers sent from the specified FinancialAccount.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryInboundTransfersResourceInboundTransfer>> {
        client.get_query("/treasury/inbound_transfers", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct SucceedTreasuryInboundTransfersResourceInboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> SucceedTreasuryInboundTransfersResourceInboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> SucceedTreasuryInboundTransfersResourceInboundTransfer<'a> {
    /// Transitions a test mode created InboundTransfer to the `succeeded` status.
    ///
    /// The InboundTransfer must already be in the `processing` state.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_inbound_transfers_resource_inbound_transfer::TreasuryInboundTransferId) -> stripe::Response<stripe_treasury::TreasuryInboundTransfersResourceInboundTransfer> {
        client.send_form(&format!("/test_helpers/treasury/inbound_transfers/{id}/succeed", id = id), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct FailTreasuryInboundTransfersResourceInboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Details about a failed InboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<FailTreasuryInboundTransfersResourceInboundTransferFailureDetails>,
}
impl<'a> FailTreasuryInboundTransfersResourceInboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details about a failed InboundTransfer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct FailTreasuryInboundTransfersResourceInboundTransferFailureDetails {
    /// Reason for the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<FailTreasuryInboundTransfersResourceInboundTransferFailureDetailsCode>,
}
impl FailTreasuryInboundTransfersResourceInboundTransferFailureDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Reason for the failure.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FailTreasuryInboundTransfersResourceInboundTransferFailureDetailsCode {
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

impl FailTreasuryInboundTransfersResourceInboundTransferFailureDetailsCode {
    pub fn as_str(self) -> &'static str {
        use FailTreasuryInboundTransfersResourceInboundTransferFailureDetailsCode::*;
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
        }
    }
}

impl std::str::FromStr for FailTreasuryInboundTransfersResourceInboundTransferFailureDetailsCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FailTreasuryInboundTransfersResourceInboundTransferFailureDetailsCode::*;
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
            _ => Err(()),
        }
    }
}

impl AsRef<str> for FailTreasuryInboundTransfersResourceInboundTransferFailureDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FailTreasuryInboundTransfersResourceInboundTransferFailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FailTreasuryInboundTransfersResourceInboundTransferFailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FailTreasuryInboundTransfersResourceInboundTransferFailureDetailsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> FailTreasuryInboundTransfersResourceInboundTransfer<'a> {
    /// Transitions a test mode created InboundTransfer to the `failed` status.
    ///
    /// The InboundTransfer must already be in the `processing` state.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_inbound_transfers_resource_inbound_transfer::TreasuryInboundTransferId) -> stripe::Response<stripe_treasury::TreasuryInboundTransfersResourceInboundTransfer> {
        client.send_form(&format!("/test_helpers/treasury/inbound_transfers/{id}/fail", id = id), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnInboundTransferTreasuryInboundTransfersResourceInboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ReturnInboundTransferTreasuryInboundTransfersResourceInboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ReturnInboundTransferTreasuryInboundTransfersResourceInboundTransfer<'a> {
    /// Marks the test mode InboundTransfer object as returned and links the InboundTransfer to a ReceivedDebit.
    ///
    /// The InboundTransfer must already be in the `succeeded` state.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_inbound_transfers_resource_inbound_transfer::TreasuryInboundTransferId) -> stripe::Response<stripe_treasury::TreasuryInboundTransfersResourceInboundTransfer> {
        client.send_form(&format!("/test_helpers/treasury/inbound_transfers/{id}/return", id = id), self, http_types::Method::Post)
    }
}
