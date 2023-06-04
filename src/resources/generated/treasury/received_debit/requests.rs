use crate::{Client, Response};

impl crate::treasury::received_debit::ReceivedDebit {
    /// Returns a list of ReceivedDebits.
    pub fn list(
        client: &Client,
        params: ListReceivedDebit,
    ) -> Response<crate::List<crate::treasury::received_debit::ReceivedDebit>> {
        client.get_query("/treasury/received_debits", params)
    }
    /// Retrieves the details of an existing ReceivedDebit by passing the unique ReceivedDebit ID from the ReceivedDebit list.
    pub fn retrieve(
        client: &Client,
        id: &str,
        params: RetrieveReceivedDebit,
    ) -> Response<crate::treasury::received_debit::ReceivedDebit> {
        client.get_query(&format!("/treasury/received_debits/{id}", id = id), params)
    }
    /// Use this endpoint to simulate a test mode ReceivedDebit initiated by a third party.
    ///
    /// In live mode, you can’t directly create ReceivedDebits initiated by third parties.
    pub fn create(
        client: &Client,
        params: CreateReceivedDebit,
    ) -> Response<crate::treasury::received_debit::ReceivedDebit> {
        client.send_form("/test_helpers/treasury/received_debits", params, http_types::Method::Post)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListReceivedDebit<'a> {
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
    pub status: Option<ListReceivedDebitStatus>,
}
impl<'a> ListReceivedDebit<'a> {
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
/// Only return ReceivedDebits that have the given status: `succeeded` or `failed`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListReceivedDebitStatus {
    Failed,
    Succeeded,
}

impl ListReceivedDebitStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for ListReceivedDebitStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListReceivedDebitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveReceivedDebit<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveReceivedDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateReceivedDebit<'a> {
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
    /// The FinancialAccount to pull funds from.
    pub financial_account: &'a str,
    /// Initiating payment method details for the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiating_payment_method_details:
        Option<CreateReceivedDebitInitiatingPaymentMethodDetails<'a>>,
    /// The rails used for the object.
    pub network: CreateReceivedDebitNetwork,
}
impl<'a> CreateReceivedDebit<'a> {
    pub fn new(
        amount: i64,
        currency: crate::Currency,
        financial_account: &'a str,
        network: CreateReceivedDebitNetwork,
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
pub struct CreateReceivedDebitInitiatingPaymentMethodDetails<'a> {
    /// The source type.
    #[serde(rename = "type")]
    pub type_: CreateReceivedDebitInitiatingPaymentMethodDetailsType,
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount<'a>>,
}
impl<'a> CreateReceivedDebitInitiatingPaymentMethodDetails<'a> {
    pub fn new(type_: CreateReceivedDebitInitiatingPaymentMethodDetailsType) -> Self {
        Self { type_, us_bank_account: Default::default() }
    }
}
/// The source type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateReceivedDebitInitiatingPaymentMethodDetailsType {
    UsBankAccount,
}

impl CreateReceivedDebitInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for CreateReceivedDebitInitiatingPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateReceivedDebitInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount<'a> {
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
impl<'a> CreateReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The rails used for the object.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateReceivedDebitNetwork {
    Ach,
}

impl CreateReceivedDebitNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
        }
    }
}

impl AsRef<str> for CreateReceivedDebitNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
