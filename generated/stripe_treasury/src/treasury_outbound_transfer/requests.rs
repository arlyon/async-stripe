#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListTreasuryOutboundTransfer<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Returns objects associated with this FinancialAccount.
    pub financial_account: &'a str,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return OutboundTransfers that have the given status: `processing`, `canceled`, `failed`, `posted`, or `returned`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_treasury::TreasuryOutboundTransferStatus>,
}
impl<'a> ListTreasuryOutboundTransfer<'a> {
    pub fn new(financial_account: &'a str) -> Self {
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
impl<'a> ListTreasuryOutboundTransfer<'a> {
    /// Returns a list of OutboundTransfers sent from the specified FinancialAccount.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryOutboundTransfer>> {
        client.get_query("/treasury/outbound_transfers", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_treasury::TreasuryOutboundTransfer>> {
        stripe::ListPaginator::from_list_params("/treasury/outbound_transfers", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryOutboundTransfer<'a> {
    /// Retrieves the details of an existing OutboundTransfer by passing the unique OutboundTransfer ID from either the OutboundTransfer creation request or OutboundTransfer list.
    pub fn send(
        &self,
        client: &stripe::Client,
        outbound_transfer: &stripe_treasury::TreasuryOutboundTransferId,
    ) -> stripe::Response<stripe_treasury::TreasuryOutboundTransfer> {
        client.get_query(&format!("/treasury/outbound_transfers/{outbound_transfer}"), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct FailTreasuryOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> FailTreasuryOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> FailTreasuryOutboundTransfer<'a> {
    /// Transitions a test mode created OutboundTransfer to the `failed` status.
    /// The OutboundTransfer must already be in the `processing` state.
    pub fn send(
        &self,
        client: &stripe::Client,
        outbound_transfer: &str,
    ) -> stripe::Response<stripe_treasury::TreasuryOutboundTransfer> {
        client.send_form(
            &format!("/test_helpers/treasury/outbound_transfers/{outbound_transfer}/fail"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PostTreasuryOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> PostTreasuryOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> PostTreasuryOutboundTransfer<'a> {
    /// Transitions a test mode created OutboundTransfer to the `posted` status.
    /// The OutboundTransfer must already be in the `processing` state.
    pub fn send(
        &self,
        client: &stripe::Client,
        outbound_transfer: &str,
    ) -> stripe::Response<stripe_treasury::TreasuryOutboundTransfer> {
        client.send_form(
            &format!("/test_helpers/treasury/outbound_transfers/{outbound_transfer}/post"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnOutboundTransferTreasuryOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Details about a returned OutboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_details: Option<ReturnOutboundTransferTreasuryOutboundTransferReturnedDetails>,
}
impl<'a> ReturnOutboundTransferTreasuryOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details about a returned OutboundTransfer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnOutboundTransferTreasuryOutboundTransferReturnedDetails {
    /// Reason for the return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ReturnOutboundTransferTreasuryOutboundTransferReturnedDetailsCode>,
}
impl ReturnOutboundTransferTreasuryOutboundTransferReturnedDetails {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
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
            _ => Err(()),
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
impl<'a> ReturnOutboundTransferTreasuryOutboundTransfer<'a> {
    /// Transitions a test mode created OutboundTransfer to the `returned` status.
    /// The OutboundTransfer must already be in the `processing` state.
    pub fn send(
        &self,
        client: &stripe::Client,
        outbound_transfer: &str,
    ) -> stripe::Response<stripe_treasury::TreasuryOutboundTransfer> {
        client.send_form(
            &format!("/test_helpers/treasury/outbound_transfers/{outbound_transfer}/return"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryOutboundTransfer<'a> {
    /// Amount (in cents) to be transferred.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The PaymentMethod to use as the payment instrument for the OutboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method: Option<&'a str>,
    /// Hash describing payment method configuration details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method_options:
        Option<CreateTreasuryOutboundTransferDestinationPaymentMethodOptions>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The FinancialAccount to pull funds from.
    pub financial_account: &'a str,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Statement descriptor to be shown on the receiving end of an OutboundTransfer.
    /// Maximum 10 characters for `ach` transfers or 140 characters for `us_domestic_wire` transfers.
    /// The default value is "transfer".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}
impl<'a> CreateTreasuryOutboundTransfer<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency, financial_account: &'a str) -> Self {
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryOutboundTransferDestinationPaymentMethodOptions {
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccount>,
}
impl CreateTreasuryOutboundTransferDestinationPaymentMethodOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccount {
    /// Designate the OutboundTransfer as using a US bank account network configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network:
        Option<CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork>,
}
impl CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Designate the OutboundTransfer as using a US bank account network configuration.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryOutboundTransferDestinationPaymentMethodOptionsUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
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
impl<'a> CreateTreasuryOutboundTransfer<'a> {
    /// Creates an OutboundTransfer.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_treasury::TreasuryOutboundTransfer> {
        client.send_form("/treasury/outbound_transfers", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelTreasuryOutboundTransfer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelTreasuryOutboundTransfer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CancelTreasuryOutboundTransfer<'a> {
    /// An OutboundTransfer can be canceled if the funds have not yet been paid out.
    pub fn send(
        &self,
        client: &stripe::Client,
        outbound_transfer: &stripe_treasury::TreasuryOutboundTransferId,
    ) -> stripe::Response<stripe_treasury::TreasuryOutboundTransfer> {
        client.send_form(
            &format!("/treasury/outbound_transfers/{outbound_transfer}/cancel"),
            self,
            http_types::Method::Post,
        )
    }
}
