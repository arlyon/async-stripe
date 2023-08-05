#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
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
    pub destination_payment_method_options: Option<CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptions>,
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
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Statement descriptor to be shown on the receiving end of an OutboundTransfer.
    ///
    /// Maximum 10 characters for `ach` transfers or 140 characters for `wire` transfers.
    /// The default value is `transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}
impl<'a> CreateTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
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
pub struct CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptions {
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccount>,
}
impl CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccount {
    /// Designate the OutboundTransfer as using a US bank account network configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork>,
}
impl CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Designate the OutboundTransfer as using a US bank account network configuration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}

impl CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryOutboundTransfersResourceOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    /// Creates an OutboundTransfer.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_treasury::TreasuryOutboundTransfersResourceOutboundTransfer> {
        client.send_form("/treasury/outbound_transfers", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    /// Retrieves the details of an existing OutboundTransfer by passing the unique OutboundTransfer ID from either the OutboundTransfer creation request or OutboundTransfer list.
    pub fn send(&self, client: &stripe::Client, outbound_transfer: &stripe_treasury::treasury_outbound_transfers_resource_outbound_transfer::TreasuryOutboundTransferId) -> stripe::Response<stripe_treasury::TreasuryOutboundTransfersResourceOutboundTransfer> {
        client.get_query(&format!("/treasury/outbound_transfers/{outbound_transfer}", outbound_transfer = outbound_transfer), self)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
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
    pub status: Option<ListTreasuryOutboundTransfersResourceOutboundTransferStatus>,
}
impl<'a> ListTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self { ending_before: Default::default(), expand: Default::default(), financial_account, limit: Default::default(), starting_after: Default::default(), status: Default::default() }
    }
}
/// Only return OutboundTransfers that have the given status: `processing`, `canceled`, `failed`, `posted`, or `returned`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryOutboundTransfersResourceOutboundTransferStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl ListTreasuryOutboundTransfersResourceOutboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryOutboundTransfersResourceOutboundTransferStatus::*;
        match self {
            Canceled => "canceled",
            Failed => "failed",
            Posted => "posted",
            Processing => "processing",
            Returned => "returned",
        }
    }
}

impl std::str::FromStr for ListTreasuryOutboundTransfersResourceOutboundTransferStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryOutboundTransfersResourceOutboundTransferStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "failed" => Ok(Failed),
            "posted" => Ok(Posted),
            "processing" => Ok(Processing),
            "returned" => Ok(Returned),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListTreasuryOutboundTransfersResourceOutboundTransferStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTreasuryOutboundTransfersResourceOutboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryOutboundTransfersResourceOutboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryOutboundTransfersResourceOutboundTransferStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> stripe::PaginationParams for ListTreasuryOutboundTransfersResourceOutboundTransfer<'a> {}
impl<'a> ListTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    /// Returns a list of OutboundTransfers sent from the specified FinancialAccount.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryOutboundTransfersResourceOutboundTransfer>> {
        client.get_query("/treasury/outbound_transfers", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_treasury::TreasuryOutboundTransfersResourceOutboundTransfer> {
        stripe::ListPaginator::from_params("/treasury/outbound_transfers", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CancelTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    /// An OutboundTransfer can be canceled if the funds have not yet been paid out.
    pub fn send(&self, client: &stripe::Client, outbound_transfer: &stripe_treasury::treasury_outbound_transfers_resource_outbound_transfer::TreasuryOutboundTransferId) -> stripe::Response<stripe_treasury::TreasuryOutboundTransfersResourceOutboundTransfer> {
        client.send_form(&format!("/treasury/outbound_transfers/{outbound_transfer}/cancel", outbound_transfer = outbound_transfer), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct FailTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> FailTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> FailTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    /// Transitions a test mode created OutboundTransfer to the `failed` status.
    ///
    /// The OutboundTransfer must already be in the `processing` state.
    pub fn send(&self, client: &stripe::Client, outbound_transfer: &stripe_treasury::treasury_outbound_transfers_resource_outbound_transfer::TreasuryOutboundTransferId) -> stripe::Response<stripe_treasury::TreasuryOutboundTransfersResourceOutboundTransfer> {
        client.send_form(&format!("/test_helpers/treasury/outbound_transfers/{outbound_transfer}/fail", outbound_transfer = outbound_transfer), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PostTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> PostTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> PostTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    /// Transitions a test mode created OutboundTransfer to the `posted` status.
    ///
    /// The OutboundTransfer must already be in the `processing` state.
    pub fn send(&self, client: &stripe::Client, outbound_transfer: &stripe_treasury::treasury_outbound_transfers_resource_outbound_transfer::TreasuryOutboundTransferId) -> stripe::Response<stripe_treasury::TreasuryOutboundTransfersResourceOutboundTransfer> {
        client.send_form(&format!("/test_helpers/treasury/outbound_transfers/{outbound_transfer}/post", outbound_transfer = outbound_transfer), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Details about a returned OutboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_details: Option<ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetails>,
}
impl<'a> ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details about a returned OutboundTransfer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetails {
    /// Reason for the return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetailsCode>,
}
impl ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Reason for the return.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetailsCode {
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

impl ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetailsCode {
    pub fn as_str(self) -> &'static str {
        use ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetailsCode::*;
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

impl std::str::FromStr for ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetailsCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetailsCode::*;
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
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransferReturnedDetailsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ReturnOutboundTransferTreasuryOutboundTransfersResourceOutboundTransfer<'a> {
    /// Transitions a test mode created OutboundTransfer to the `returned` status.
    ///
    /// The OutboundTransfer must already be in the `processing` state.
    pub fn send(&self, client: &stripe::Client, outbound_transfer: &stripe_treasury::treasury_outbound_transfers_resource_outbound_transfer::TreasuryOutboundTransferId) -> stripe::Response<stripe_treasury::TreasuryOutboundTransfersResourceOutboundTransfer> {
        client.send_form(&format!("/test_helpers/treasury/outbound_transfers/{outbound_transfer}/return", outbound_transfer = outbound_transfer), self, http_types::Method::Post)
    }
}
