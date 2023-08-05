#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryReceivedDebitsResourceReceivedDebit<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The FinancialAccount that funds were pulled from.
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
    /// Only return ReceivedDebits that have the given status: `succeeded` or `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTreasuryReceivedDebitsResourceReceivedDebitStatus>,
}
impl<'a> ListTreasuryReceivedDebitsResourceReceivedDebit<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self { ending_before: Default::default(), expand: Default::default(), financial_account, limit: Default::default(), starting_after: Default::default(), status: Default::default() }
    }
}
/// Only return ReceivedDebits that have the given status: `succeeded` or `failed`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryReceivedDebitsResourceReceivedDebitStatus {
    Failed,
    Succeeded,
}

impl ListTreasuryReceivedDebitsResourceReceivedDebitStatus {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryReceivedDebitsResourceReceivedDebitStatus::*;
        match self {
            Failed => "failed",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for ListTreasuryReceivedDebitsResourceReceivedDebitStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryReceivedDebitsResourceReceivedDebitStatus::*;
        match s {
            "failed" => Ok(Failed),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListTreasuryReceivedDebitsResourceReceivedDebitStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTreasuryReceivedDebitsResourceReceivedDebitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryReceivedDebitsResourceReceivedDebitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryReceivedDebitsResourceReceivedDebitStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> stripe::PaginationParams for ListTreasuryReceivedDebitsResourceReceivedDebit<'a> {}
impl<'a> ListTreasuryReceivedDebitsResourceReceivedDebit<'a> {
    /// Returns a list of ReceivedDebits.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryReceivedDebitsResourceReceivedDebit>> {
        client.get_query("/treasury/received_debits", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_treasury::TreasuryReceivedDebitsResourceReceivedDebit> {
        stripe::ListPaginator::from_params("/treasury/received_debits", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryReceivedDebitsResourceReceivedDebit<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryReceivedDebitsResourceReceivedDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryReceivedDebitsResourceReceivedDebit<'a> {
    /// Retrieves the details of an existing ReceivedDebit by passing the unique ReceivedDebit ID from the ReceivedDebit list.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_received_debits_resource_received_debit::TreasuryReceivedDebitId) -> stripe::Response<stripe_treasury::TreasuryReceivedDebitsResourceReceivedDebit> {
        client.get_query(&format!("/treasury/received_debits/{id}", id = id), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedDebitsResourceReceivedDebit<'a> {
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
    /// The FinancialAccount to pull funds from.
    pub financial_account: &'a str,
    /// Initiating payment method details for the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiating_payment_method_details: Option<CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetails<'a>>,
    /// The rails used for the object.
    pub network: CreateTreasuryReceivedDebitsResourceReceivedDebitNetwork,
}
impl<'a> CreateTreasuryReceivedDebitsResourceReceivedDebit<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency, financial_account: &'a str, network: CreateTreasuryReceivedDebitsResourceReceivedDebitNetwork) -> Self {
        Self { amount, currency, description: Default::default(), expand: Default::default(), financial_account, initiating_payment_method_details: Default::default(), network }
    }
}
/// Initiating payment method details for the object.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetails<'a> {
    /// The source type.
    #[serde(rename = "type")]
    pub type_: CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsType,
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount<'a>>,
}
impl<'a> CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetails<'a> {
    pub fn new(type_: CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsType) -> Self {
        Self { type_, us_bank_account: Default::default() }
    }
}
/// The source type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsType {
    UsBankAccount,
}

impl CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsType::*;
        match self {
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount<'a> {
    /// The bank account holder's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<&'a str>,
    /// The bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// The bank account's routing number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> CreateTreasuryReceivedDebitsResourceReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The rails used for the object.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryReceivedDebitsResourceReceivedDebitNetwork {
    Ach,
}

impl CreateTreasuryReceivedDebitsResourceReceivedDebitNetwork {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryReceivedDebitsResourceReceivedDebitNetwork::*;
        match self {
            Ach => "ach",
        }
    }
}

impl std::str::FromStr for CreateTreasuryReceivedDebitsResourceReceivedDebitNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedDebitsResourceReceivedDebitNetwork::*;
        match s {
            "ach" => Ok(Ach),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTreasuryReceivedDebitsResourceReceivedDebitNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTreasuryReceivedDebitsResourceReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryReceivedDebitsResourceReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryReceivedDebitsResourceReceivedDebitNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateTreasuryReceivedDebitsResourceReceivedDebit<'a> {
    /// Use this endpoint to simulate a test mode ReceivedDebit initiated by a third party.
    ///
    /// In live mode, you can’t directly create ReceivedDebits initiated by third parties.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_treasury::TreasuryReceivedDebitsResourceReceivedDebit> {
        client.send_form("/test_helpers/treasury/received_debits", self, http_types::Method::Post)
    }
}
