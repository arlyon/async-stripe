#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
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
    pub destination_payment_method_data: Option<CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodData<'a>>,
    /// Payment method-specific configuration for this OutboundPayment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method_options: Option<CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptions>,
    /// End user details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_details: Option<CreateTreasuryOutboundPaymentsResourceOutboundPaymentEndUserDetails<'a>>,
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
impl<'a> CreateTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
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
pub struct CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodData<'a> {
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataBillingDetails<'a>>,
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
    pub type_: CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataType,
    /// Required hash if type is set to `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccount<'a>>,
}
impl<'a> CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodData<'a> {
    pub fn new(type_: CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataType) -> Self {
        Self { billing_details: Default::default(), financial_account: Default::default(), metadata: Default::default(), type_, us_bank_account: Default::default() }
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataBillingDetails<'a> {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataBillingDetailsAddress<'a>>,
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
impl<'a> CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataBillingDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataBillingDetailsAddress<'a> {
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
impl<'a> CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataBillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataType {
    FinancialAccount,
    UsBankAccount,
}

impl CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataType::*;
        match self {
            FinancialAccount => "financial_account",
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataType::*;
        match s {
            "financial_account" => Ok(FinancialAccount),
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Required hash if type is set to `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}

impl CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Payment method-specific configuration for this OutboundPayment.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptions {
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccount>,
}
impl CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccount {
    /// The US bank account network that must be used for this OutboundPayment.
    ///
    /// If not set, we will default to the PaymentMethod's preferred network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork>,
}
impl CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The US bank account network that must be used for this OutboundPayment.
///
/// If not set, we will default to the PaymentMethod's preferred network.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}

impl CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryOutboundPaymentsResourceOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// End user details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryOutboundPaymentsResourceOutboundPaymentEndUserDetails<'a> {
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
impl<'a> CreateTreasuryOutboundPaymentsResourceOutboundPaymentEndUserDetails<'a> {
    pub fn new(present: bool) -> Self {
        Self { ip_address: Default::default(), present }
    }
}
impl<'a> CreateTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    /// Creates an OutboundPayment.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_treasury::TreasuryOutboundPaymentsResourceOutboundPayment> {
        client.send_form("/treasury/outbound_payments", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    /// Retrieves the details of an existing OutboundPayment by passing the unique OutboundPayment ID from either the OutboundPayment creation request or OutboundPayment list.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_outbound_payments_resource_outbound_payment::TreasuryOutboundPaymentId) -> stripe::Response<stripe_treasury::TreasuryOutboundPaymentsResourceOutboundPayment> {
        client.get_query(&format!("/treasury/outbound_payments/{id}", id = id), self)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
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
    pub status: Option<ListTreasuryOutboundPaymentsResourceOutboundPaymentStatus>,
}
impl<'a> ListTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self { customer: Default::default(), ending_before: Default::default(), expand: Default::default(), financial_account, limit: Default::default(), starting_after: Default::default(), status: Default::default() }
    }
}
/// Only return OutboundPayments that have the given status: `processing`, `failed`, `posted`, `returned`, or `canceled`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl ListTreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryOutboundPaymentsResourceOutboundPaymentStatus::*;
        match self {
            Canceled => "canceled",
            Failed => "failed",
            Posted => "posted",
            Processing => "processing",
            Returned => "returned",
        }
    }
}

impl std::str::FromStr for ListTreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryOutboundPaymentsResourceOutboundPaymentStatus::*;
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

impl AsRef<str> for ListTreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> stripe::PaginationParams for ListTreasuryOutboundPaymentsResourceOutboundPayment<'a> {}
impl<'a> ListTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    /// Returns a list of OutboundPayments sent from the specified FinancialAccount.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryOutboundPaymentsResourceOutboundPayment>> {
        client.get_query("/treasury/outbound_payments", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_treasury::TreasuryOutboundPaymentsResourceOutboundPayment> {
        stripe::ListPaginator::from_params("/treasury/outbound_payments", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CancelTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    /// Cancel an OutboundPayment.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_outbound_payments_resource_outbound_payment::TreasuryOutboundPaymentId) -> stripe::Response<stripe_treasury::TreasuryOutboundPaymentsResourceOutboundPayment> {
        client.send_form(&format!("/treasury/outbound_payments/{id}/cancel", id = id), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct FailTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> FailTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> FailTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    /// Transitions a test mode created OutboundPayment to the `failed` status.
    ///
    /// The OutboundPayment must already be in the `processing` state.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_outbound_payments_resource_outbound_payment::TreasuryOutboundPaymentId) -> stripe::Response<stripe_treasury::TreasuryOutboundPaymentsResourceOutboundPayment> {
        client.send_form(&format!("/test_helpers/treasury/outbound_payments/{id}/fail", id = id), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct PostTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> PostTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> PostTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    /// Transitions a test mode created OutboundPayment to the `posted` status.
    ///
    /// The OutboundPayment must already be in the `processing` state.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_outbound_payments_resource_outbound_payment::TreasuryOutboundPaymentId) -> stripe::Response<stripe_treasury::TreasuryOutboundPaymentsResourceOutboundPayment> {
        client.send_form(&format!("/test_helpers/treasury/outbound_payments/{id}/post", id = id), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Optional hash to set the the return code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_details: Option<ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetails>,
}
impl<'a> ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Optional hash to set the the return code.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetails {
    /// The return code to be set on the OutboundPayment object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetailsCode>,
}
impl ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The return code to be set on the OutboundPayment object.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetailsCode {
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

impl ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetailsCode {
    pub fn as_str(self) -> &'static str {
        use ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetailsCode::*;
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

impl std::str::FromStr for ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetailsCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetailsCode::*;
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

impl AsRef<str> for ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPaymentReturnedDetailsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ReturnOutboundPaymentTreasuryOutboundPaymentsResourceOutboundPayment<'a> {
    /// Transitions a test mode created OutboundPayment to the `returned` status.
    ///
    /// The OutboundPayment must already be in the `processing` state.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_outbound_payments_resource_outbound_payment::TreasuryOutboundPaymentId) -> stripe::Response<stripe_treasury::TreasuryOutboundPaymentsResourceOutboundPayment> {
        client.send_form(&format!("/test_helpers/treasury/outbound_payments/{id}/return", id = id), self, http_types::Method::Post)
    }
}
