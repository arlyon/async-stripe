use crate::{Client, Response};

impl crate::treasury::received_credit::ReceivedCredit {
    /// Returns a list of ReceivedCredits.
    pub fn list(
        client: &Client,
        params: ListReceivedCredit,
    ) -> Response<crate::List<crate::treasury::received_credit::ReceivedCredit>> {
        client.get_query("/treasury/received_credits", params)
    }
    /// Retrieves the details of an existing ReceivedCredit by passing the unique ReceivedCredit ID from the ReceivedCredit list.
    pub fn retrieve(
        client: &Client,
        id: &str,
        params: RetrieveReceivedCredit,
    ) -> Response<crate::treasury::received_credit::ReceivedCredit> {
        client.get_query(&format!("/treasury/received_credits/{id}", id = id), params)
    }
    /// Use this endpoint to simulate a test mode ReceivedCredit initiated by a third party.
    ///
    /// In live mode, you can’t directly create ReceivedCredits initiated by third parties.
    pub fn create(
        client: &Client,
        params: CreateReceivedCredit,
    ) -> Response<crate::treasury::received_credit::ReceivedCredit> {
        client.send_form(
            "/test_helpers/treasury/received_credits",
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListReceivedCredit<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
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
    pub linked_flows: Option<ListReceivedCreditLinkedFlows>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Only return ReceivedCredits that have the given status: `succeeded` or `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListReceivedCreditStatus>,
}
impl<'a> ListReceivedCredit<'a> {
    pub fn new(financial_account: &'a str) -> Self {
        Self {
            ending_before: Default::default(),
            expand: Default::default(),
            financial_account,
            limit: Default::default(),
            linked_flows: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}
/// Only return ReceivedCredits described by the flow.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListReceivedCreditLinkedFlows {
    /// The source flow type.
    pub source_flow_type: ListReceivedCreditLinkedFlowsSourceFlowType,
}
impl ListReceivedCreditLinkedFlows {
    pub fn new(source_flow_type: ListReceivedCreditLinkedFlowsSourceFlowType) -> Self {
        Self { source_flow_type }
    }
}
/// The source flow type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListReceivedCreditLinkedFlowsSourceFlowType {
    CreditReversal,
    Other,
    OutboundPayment,
    Payout,
}

impl ListReceivedCreditLinkedFlowsSourceFlowType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditReversal => "credit_reversal",
            Self::Other => "other",
            Self::OutboundPayment => "outbound_payment",
            Self::Payout => "payout",
        }
    }
}

impl AsRef<str> for ListReceivedCreditLinkedFlowsSourceFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListReceivedCreditLinkedFlowsSourceFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Only return ReceivedCredits that have the given status: `succeeded` or `failed`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListReceivedCreditStatus {
    Failed,
    Succeeded,
}

impl ListReceivedCreditStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for ListReceivedCreditStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveReceivedCredit<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveReceivedCredit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateReceivedCredit<'a> {
    /// Amount (in cents) to be transferred.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
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
    pub initiating_payment_method_details:
        Option<CreateReceivedCreditInitiatingPaymentMethodDetails<'a>>,
    /// The rails used for the object.
    pub network: CreateReceivedCreditNetwork,
}
impl<'a> CreateReceivedCredit<'a> {
    pub fn new(
        amount: i64,
        currency: crate::Currency,
        financial_account: &'a str,
        network: CreateReceivedCreditNetwork,
    ) -> Self {
        Self {
            amount,
            currency,
            description: Default::default(),
            expand: Default::default(),
            financial_account,
            initiating_payment_method_details: Default::default(),
            network,
        }
    }
}
/// Initiating payment method details for the object.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateReceivedCreditInitiatingPaymentMethodDetails<'a> {
    /// The source type.
    #[serde(rename = "type")]
    pub type_: CreateReceivedCreditInitiatingPaymentMethodDetailsType,
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<CreateReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount<'a>>,
}
impl<'a> CreateReceivedCreditInitiatingPaymentMethodDetails<'a> {
    pub fn new(type_: CreateReceivedCreditInitiatingPaymentMethodDetailsType) -> Self {
        Self { type_, us_bank_account: Default::default() }
    }
}
/// The source type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateReceivedCreditInitiatingPaymentMethodDetailsType {
    UsBankAccount,
}

impl CreateReceivedCreditInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for CreateReceivedCreditInitiatingPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateReceivedCreditInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount<'a> {
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
impl<'a> CreateReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The rails used for the object.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateReceivedCreditNetwork {
    Ach,
    UsDomesticWire,
}

impl CreateReceivedCreditNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for CreateReceivedCreditNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
