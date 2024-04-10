#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListTreasuryReceivedCredit<'a> {
    /// A cursor for use in pagination.
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
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return ReceivedCredits described by the flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_flows: Option<ListTreasuryReceivedCreditLinkedFlows>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return ReceivedCredits that have the given status: `succeeded` or `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_treasury::TreasuryReceivedCreditStatus>,
}
impl<'a> ListTreasuryReceivedCredit<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self {
            ending_before: None,
            expand: None,
            financial_account,
            limit: None,
            linked_flows: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Only return ReceivedCredits described by the flow.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListTreasuryReceivedCreditLinkedFlows {
    /// The source flow type.
    pub source_flow_type: ListTreasuryReceivedCreditLinkedFlowsSourceFlowType,
}
impl ListTreasuryReceivedCreditLinkedFlows {
    pub fn new(source_flow_type: ListTreasuryReceivedCreditLinkedFlowsSourceFlowType) -> Self {
        Self { source_flow_type }
    }
}
/// The source flow type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    CreditReversal,
    Other,
    OutboundPayment,
    Payout,
}
impl ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryReceivedCreditLinkedFlowsSourceFlowType::*;
        match self {
            CreditReversal => "credit_reversal",
            Other => "other",
            OutboundPayment => "outbound_payment",
            Payout => "payout",
        }
    }
}

impl std::str::FromStr for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryReceivedCreditLinkedFlowsSourceFlowType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "payout" => Ok(Payout),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType",
            )
        })
    }
}
impl<'a> ListTreasuryReceivedCredit<'a> {
    /// Returns a list of ReceivedCredits.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryReceivedCredit>> {
        client.get_query("/treasury/received_credits", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_treasury::TreasuryReceivedCredit>> {
        stripe::ListPaginator::from_list_params("/treasury/received_credits", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryReceivedCredit<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryReceivedCredit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryReceivedCredit<'a> {
    /// Retrieves the details of an existing ReceivedCredit by passing the unique ReceivedCredit ID from the ReceivedCredit list.
    pub fn send(
        &self,
        client: &stripe::Client,
        id: &stripe_treasury::TreasuryReceivedCreditId,
    ) -> stripe::Response<stripe_treasury::TreasuryReceivedCredit> {
        client.get_query(&format!("/treasury/received_credits/{id}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedCredit<'a> {
    /// Amount (in cents) to be transferred.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The FinancialAccount to send funds to.
    pub financial_account: &'a str,
    /// Initiating payment method details for the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiating_payment_method_details:
        Option<CreateTreasuryReceivedCreditInitiatingPaymentMethodDetails<'a>>,
    /// The rails used for the object.
    pub network: CreateTreasuryReceivedCreditNetwork,
}
impl<'a> CreateTreasuryReceivedCredit<'a> {
    pub fn new(
        amount: i64,
        currency: stripe_types::Currency,
        financial_account: &'a str,
        network: CreateTreasuryReceivedCreditNetwork,
    ) -> Self {
        Self {
            amount,
            currency,
            description: None,
            expand: None,
            financial_account,
            initiating_payment_method_details: None,
            network,
        }
    }
}
/// Initiating payment method details for the object.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedCreditInitiatingPaymentMethodDetails<'a> {
    /// The source type.
    #[serde(rename = "type")]
    pub type_: CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType,
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount<'a>>,
}
impl<'a> CreateTreasuryReceivedCreditInitiatingPaymentMethodDetails<'a> {
    pub fn new(type_: CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType) -> Self {
        Self { type_, us_bank_account: None }
    }
}
/// The source type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType {
    UsBankAccount,
}
impl CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType::*;
        match self {
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType",
            )
        })
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount<'a> {
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
impl<'a> CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The rails used for the object.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryReceivedCreditNetwork {
    Ach,
    UsDomesticWire,
}
impl CreateTreasuryReceivedCreditNetwork {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryReceivedCreditNetwork::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for CreateTreasuryReceivedCreditNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedCreditNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateTreasuryReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryReceivedCreditNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTreasuryReceivedCreditNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTreasuryReceivedCreditNetwork")
        })
    }
}
impl<'a> CreateTreasuryReceivedCredit<'a> {
    /// Use this endpoint to simulate a test mode ReceivedCredit initiated by a third party.
    /// In live mode, you can’t directly create ReceivedCredits initiated by third parties.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_treasury::TreasuryReceivedCredit> {
        client.send_form("/test_helpers/treasury/received_credits", self, http_types::Method::Post)
    }
}
