#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListTreasuryReceivedCreditsResourceReceivedCredit<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The FinancialAccount that received the funds.
    pub financial_account: &'a str,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return ReceivedCredits described by the flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_flows: Option<ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlows>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return ReceivedCredits that have the given status: `succeeded` or `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTreasuryReceivedCreditsResourceReceivedCreditStatus>,
}
impl<'a> ListTreasuryReceivedCreditsResourceReceivedCredit<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self { ending_before: Default::default(), expand: Default::default(), financial_account, limit: Default::default(), linked_flows: Default::default(), starting_after: Default::default(), status: Default::default() }
    }
}
/// Only return ReceivedCredits described by the flow.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlows {
    /// The source flow type.
    pub source_flow_type: ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlowsSourceFlowType,
}
impl ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlows {
    pub fn new(source_flow_type: ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlowsSourceFlowType) -> Self {
        Self { source_flow_type }
    }
}
/// The source flow type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlowsSourceFlowType {
    CreditReversal,
    Other,
    OutboundPayment,
    Payout,
}

impl ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlowsSourceFlowType {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlowsSourceFlowType::*;
        match self {
            CreditReversal => "credit_reversal",
            Other => "other",
            OutboundPayment => "outbound_payment",
            Payout => "payout",
        }
    }
}

impl std::str::FromStr for ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlowsSourceFlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlowsSourceFlowType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "payout" => Ok(Payout),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlowsSourceFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlowsSourceFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlowsSourceFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryReceivedCreditsResourceReceivedCreditLinkedFlowsSourceFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Only return ReceivedCredits that have the given status: `succeeded` or `failed`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryReceivedCreditsResourceReceivedCreditStatus {
    Failed,
    Succeeded,
}

impl ListTreasuryReceivedCreditsResourceReceivedCreditStatus {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryReceivedCreditsResourceReceivedCreditStatus::*;
        match self {
            Failed => "failed",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for ListTreasuryReceivedCreditsResourceReceivedCreditStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryReceivedCreditsResourceReceivedCreditStatus::*;
        match s {
            "failed" => Ok(Failed),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListTreasuryReceivedCreditsResourceReceivedCreditStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTreasuryReceivedCreditsResourceReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryReceivedCreditsResourceReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryReceivedCreditsResourceReceivedCreditStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> stripe::PaginationParams for ListTreasuryReceivedCreditsResourceReceivedCredit<'a> {}
impl<'a> ListTreasuryReceivedCreditsResourceReceivedCredit<'a> {
    /// Returns a list of ReceivedCredits.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryReceivedCreditsResourceReceivedCredit>> {
        client.get_query("/treasury/received_credits", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_treasury::TreasuryReceivedCreditsResourceReceivedCredit> {
        stripe::ListPaginator::from_params("/treasury/received_credits", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryReceivedCreditsResourceReceivedCredit<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryReceivedCreditsResourceReceivedCredit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryReceivedCreditsResourceReceivedCredit<'a> {
    /// Retrieves the details of an existing ReceivedCredit by passing the unique ReceivedCredit ID from the ReceivedCredit list.
    pub fn send(&self, client: &stripe::Client, id: &stripe_treasury::treasury_received_credits_resource_received_credit::TreasuryReceivedCreditId) -> stripe::Response<stripe_treasury::TreasuryReceivedCreditsResourceReceivedCredit> {
        client.get_query(&format!("/treasury/received_credits/{id}", id = id), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedCreditsResourceReceivedCredit<'a> {
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
    /// Initiating payment method details for the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiating_payment_method_details: Option<CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetails<'a>>,
    /// The rails used for the object.
    pub network: CreateTreasuryReceivedCreditsResourceReceivedCreditNetwork,
}
impl<'a> CreateTreasuryReceivedCreditsResourceReceivedCredit<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency, financial_account: &'a str, network: CreateTreasuryReceivedCreditsResourceReceivedCreditNetwork) -> Self {
        Self { amount, currency, description: Default::default(), expand: Default::default(), financial_account, initiating_payment_method_details: Default::default(), network }
    }
}
/// Initiating payment method details for the object.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetails<'a> {
    /// The source type.
    #[serde(rename = "type")]
    pub type_: CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsType,
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount<'a>>,
}
impl<'a> CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetails<'a> {
    pub fn new(type_: CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsType) -> Self {
        Self { type_, us_bank_account: Default::default() }
    }
}
/// The source type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsType {
    UsBankAccount,
}

impl CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsType::*;
        match self {
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount<'a> {
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
impl<'a> CreateTreasuryReceivedCreditsResourceReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The rails used for the object.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryReceivedCreditsResourceReceivedCreditNetwork {
    Ach,
    UsDomesticWire,
}

impl CreateTreasuryReceivedCreditsResourceReceivedCreditNetwork {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryReceivedCreditsResourceReceivedCreditNetwork::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for CreateTreasuryReceivedCreditsResourceReceivedCreditNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedCreditsResourceReceivedCreditNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTreasuryReceivedCreditsResourceReceivedCreditNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTreasuryReceivedCreditsResourceReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryReceivedCreditsResourceReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryReceivedCreditsResourceReceivedCreditNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateTreasuryReceivedCreditsResourceReceivedCredit<'a> {
    /// Use this endpoint to simulate a test mode ReceivedCredit initiated by a third party.
    ///
    /// In live mode, you can’t directly create ReceivedCredits initiated by third parties.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_treasury::TreasuryReceivedCreditsResourceReceivedCredit> {
        client.send_form("/test_helpers/treasury/received_credits", self, http_types::Method::Post)
    }
}
