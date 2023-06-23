use stripe::{Client, Response};

impl stripe_treasury::treasury::outbound_payment::OutboundPayment {
    /// Creates an OutboundPayment.
    pub fn create(
        client: &Client,
        params: CreateOutboundPayment,
    ) -> Response<stripe_treasury::treasury::outbound_payment::OutboundPayment> {
        client.send_form("/treasury/outbound_payments", params, http_types::Method::Post)
    }
    /// Retrieves the details of an existing OutboundPayment by passing the unique OutboundPayment ID from either the OutboundPayment creation request or OutboundPayment list.
    pub fn retrieve(
        client: &Client,
        id: &str,
        params: RetrieveOutboundPayment,
    ) -> Response<stripe_treasury::treasury::outbound_payment::OutboundPayment> {
        client.get_query(&format!("/treasury/outbound_payments/{id}", id = id), params)
    }
    /// Returns a list of OutboundPayments sent from the specified FinancialAccount.
    pub fn list(
        client: &Client,
        params: ListOutboundPayment,
    ) -> Response<stripe_types::List<stripe_treasury::treasury::outbound_payment::OutboundPayment>>
    {
        client.get_query("/treasury/outbound_payments", params)
    }
    /// Cancel an OutboundPayment.
    pub fn cancel(
        client: &Client,
        id: &str,
        params: CancelOutboundPayment,
    ) -> Response<stripe_treasury::treasury::outbound_payment::OutboundPayment> {
        client.send_form(
            &format!("/treasury/outbound_payments/{id}/cancel", id = id),
            params,
            http_types::Method::Post,
        )
    }
    /// Transitions a test mode created OutboundPayment to the `failed` status.
    ///
    /// The OutboundPayment must already be in the `processing` state.
    pub fn fail(
        client: &Client,
        id: &str,
        params: FailOutboundPayment,
    ) -> Response<stripe_treasury::treasury::outbound_payment::OutboundPayment> {
        client.send_form(
            &format!("/test_helpers/treasury/outbound_payments/{id}/fail", id = id),
            params,
            http_types::Method::Post,
        )
    }
    /// Transitions a test mode created OutboundPayment to the `posted` status.
    ///
    /// The OutboundPayment must already be in the `processing` state.
    pub fn post(
        client: &Client,
        id: &str,
        params: PostOutboundPayment,
    ) -> Response<stripe_treasury::treasury::outbound_payment::OutboundPayment> {
        client.send_form(
            &format!("/test_helpers/treasury/outbound_payments/{id}/post", id = id),
            params,
            http_types::Method::Post,
        )
    }
    /// Transitions a test mode created OutboundPayment to the `returned` status.
    ///
    /// The OutboundPayment must already be in the `processing` state.
    pub fn return_outbound_payment(
        client: &Client,
        id: &str,
        params: ReturnOutboundPaymentOutboundPayment,
    ) -> Response<stripe_treasury::treasury::outbound_payment::OutboundPayment> {
        client.send_form(
            &format!("/test_helpers/treasury/outbound_payments/{id}/return", id = id),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOutboundPayment<'a> {
    /// Amount (in cents) to be transferred.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the customer to whom the OutboundPayment is sent.
    ///
    /// Must match the Customer attached to the `destination_payment_method` passed in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The PaymentMethod to use as the payment instrument for the OutboundPayment.
    ///
    /// Exclusive with `destination_payment_method_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method: Option<&'a str>,
    /// Hash used to generate the PaymentMethod to be used for this OutboundPayment.
    ///
    /// Exclusive with `destination_payment_method`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method_data:
        Option<CreateOutboundPaymentDestinationPaymentMethodData<'a>>,
    /// Payment method-specific configuration for this OutboundPayment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method_options:
        Option<CreateOutboundPaymentDestinationPaymentMethodOptions>,
    /// End user details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_details: Option<CreateOutboundPaymentEndUserDetails<'a>>,
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
    /// The description that appears on the receiving end for this OutboundPayment (for example, bank statement for external bank transfer).
    ///
    /// Maximum 10 characters for `ach` payments, 140 characters for `wire` payments, or 500 characters for `stripe` network transfers.
    /// The default value is `payment`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}
impl<'a> CreateOutboundPayment<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency, financial_account: &'a str) -> Self {
        Self {
            amount,
            currency,
            customer: Default::default(),
            description: Default::default(),
            destination_payment_method: Default::default(),
            destination_payment_method_data: Default::default(),
            destination_payment_method_options: Default::default(),
            end_user_details: Default::default(),
            expand: Default::default(),
            financial_account,
            metadata: Default::default(),
            statement_descriptor: Default::default(),
        }
    }
}
/// Hash used to generate the PaymentMethod to be used for this OutboundPayment.
///
/// Exclusive with `destination_payment_method`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOutboundPaymentDestinationPaymentMethodData<'a> {
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details:
        Option<CreateOutboundPaymentDestinationPaymentMethodDataBillingDetails<'a>>,
    /// Required if type is set to `financial_account`.
    ///
    /// The FinancialAccount ID to send funds to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: CreateOutboundPaymentDestinationPaymentMethodDataType,
    /// Required hash if type is set to `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccount<'a>>,
}
impl<'a> CreateOutboundPaymentDestinationPaymentMethodData<'a> {
    pub fn new(type_: CreateOutboundPaymentDestinationPaymentMethodDataType) -> Self {
        Self {
            billing_details: Default::default(),
            financial_account: Default::default(),
            metadata: Default::default(),
            type_,
            us_bank_account: Default::default(),
        }
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOutboundPaymentDestinationPaymentMethodDataBillingDetails<'a> {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateOutboundPaymentDestinationPaymentMethodDataBillingDetailsAddress<'a>>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> CreateOutboundPaymentDestinationPaymentMethodDataBillingDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOutboundPaymentDestinationPaymentMethodDataBillingDetailsAddress<'a> {
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
impl<'a> CreateOutboundPaymentDestinationPaymentMethodDataBillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateOutboundPaymentDestinationPaymentMethodDataType {
    FinancialAccount,
    UsBankAccount,
}

impl CreateOutboundPaymentDestinationPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinancialAccount => "financial_account",
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for CreateOutboundPaymentDestinationPaymentMethodDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOutboundPaymentDestinationPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Required hash if type is set to `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type:
        Option<CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str>
    for CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}

impl CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str> for CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Payment method-specific configuration for this OutboundPayment.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOutboundPaymentDestinationPaymentMethodOptions {
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateOutboundPaymentDestinationPaymentMethodOptionsUsBankAccount>,
}
impl CreateOutboundPaymentDestinationPaymentMethodOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOutboundPaymentDestinationPaymentMethodOptionsUsBankAccount {
    /// The US bank account network that must be used for this OutboundPayment.
    ///
    /// If not set, we will default to the PaymentMethod's preferred network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<CreateOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork>,
}
impl CreateOutboundPaymentDestinationPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The US bank account network that must be used for this OutboundPayment.
///
/// If not set, we will default to the PaymentMethod's preferred network.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}

impl CreateOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for CreateOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// End user details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOutboundPaymentEndUserDetails<'a> {
    /// IP address of the user initiating the OutboundPayment.
    ///
    /// Must be supplied if `present` is set to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
    /// `True` if the OutboundPayment creation request is being made on behalf of an end user by a platform.
    ///
    /// Otherwise, `false`.
    pub present: bool,
}
impl<'a> CreateOutboundPaymentEndUserDetails<'a> {
    pub fn new(present: bool) -> Self {
        Self { ip_address: Default::default(), present }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveOutboundPayment<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveOutboundPayment<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListOutboundPayment<'a> {
    /// Only return OutboundPayments sent to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
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
    /// Only return OutboundPayments that have the given status: `processing`, `failed`, `posted`, `returned`, or `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListOutboundPaymentStatus>,
}
impl<'a> ListOutboundPayment<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self {
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            financial_account,
            limit: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}
/// Only return OutboundPayments that have the given status: `processing`, `failed`, `posted`, `returned`, or `canceled`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListOutboundPaymentStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl ListOutboundPaymentStatus {
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

impl AsRef<str> for ListOutboundPaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListOutboundPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelOutboundPayment<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelOutboundPayment<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct FailOutboundPayment<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> FailOutboundPayment<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PostOutboundPayment<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> PostOutboundPayment<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnOutboundPaymentOutboundPayment<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Optional hash to set the the return code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_details: Option<ReturnOutboundPaymentOutboundPaymentReturnedDetails>,
}
impl<'a> ReturnOutboundPaymentOutboundPayment<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Optional hash to set the the return code.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnOutboundPaymentOutboundPaymentReturnedDetails {
    /// The return code to be set on the OutboundPayment object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ReturnOutboundPaymentOutboundPaymentReturnedDetailsCode>,
}
impl ReturnOutboundPaymentOutboundPaymentReturnedDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The return code to be set on the OutboundPayment object.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnOutboundPaymentOutboundPaymentReturnedDetailsCode {
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

impl ReturnOutboundPaymentOutboundPaymentReturnedDetailsCode {
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

impl AsRef<str> for ReturnOutboundPaymentOutboundPaymentReturnedDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReturnOutboundPaymentOutboundPaymentReturnedDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
