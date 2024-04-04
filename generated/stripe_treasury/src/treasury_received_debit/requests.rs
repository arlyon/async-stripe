#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListTreasuryReceivedDebit<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The FinancialAccount that funds were pulled from.
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
    /// Only return ReceivedDebits that have the given status: `succeeded` or `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_treasury::TreasuryReceivedDebitStatus>,
}
impl<'a> ListTreasuryReceivedDebit<'a> {
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
impl<'a> ListTreasuryReceivedDebit<'a> {
    /// Returns a list of ReceivedDebits.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_treasury::TreasuryReceivedDebit>> {
        client.get_query("/treasury/received_debits", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_treasury::TreasuryReceivedDebit>> {
        stripe::ListPaginator::from_list_params("/treasury/received_debits", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTreasuryReceivedDebit<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryReceivedDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTreasuryReceivedDebit<'a> {
    /// Retrieves the details of an existing ReceivedDebit by passing the unique ReceivedDebit ID from the ReceivedDebit list.
    pub fn send(
        &self,
        client: &stripe::Client,
        id: &stripe_treasury::TreasuryReceivedDebitId,
    ) -> stripe::Response<stripe_treasury::TreasuryReceivedDebit> {
        client.get_query(&format!("/treasury/received_debits/{id}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedDebit<'a> {
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
    /// The FinancialAccount to pull funds from.
    pub financial_account: &'a str,
    /// Initiating payment method details for the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiating_payment_method_details:
        Option<CreateTreasuryReceivedDebitInitiatingPaymentMethodDetails<'a>>,
    /// The rails used for the object.
    pub network: CreateTreasuryReceivedDebitNetwork,
}
impl<'a> CreateTreasuryReceivedDebit<'a> {
    pub fn new(
        amount: i64,
        currency: stripe_types::Currency,
        financial_account: &'a str,
        network: CreateTreasuryReceivedDebitNetwork,
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
pub struct CreateTreasuryReceivedDebitInitiatingPaymentMethodDetails<'a> {
    /// The source type.
    #[serde(rename = "type")]
    pub type_: CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType,
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount<'a>>,
}
impl<'a> CreateTreasuryReceivedDebitInitiatingPaymentMethodDetails<'a> {
    pub fn new(type_: CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType) -> Self {
        Self { type_, us_bank_account: None }
    }
}
/// The source type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType {
    UsBankAccount,
}
impl CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType::*;
        match self {
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount<'a> {
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
impl<'a> CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The rails used for the object.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryReceivedDebitNetwork {
    Ach,
}
impl CreateTreasuryReceivedDebitNetwork {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryReceivedDebitNetwork::*;
        match self {
            Ach => "ach",
        }
    }
}

impl std::str::FromStr for CreateTreasuryReceivedDebitNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedDebitNetwork::*;
        match s {
            "ach" => Ok(Ach),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateTreasuryReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryReceivedDebitNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateTreasuryReceivedDebit<'a> {
    /// Use this endpoint to simulate a test mode ReceivedDebit initiated by a third party.
    /// In live mode, you can’t directly create ReceivedDebits initiated by third parties.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_treasury::TreasuryReceivedDebit> {
        client.send_form("/test_helpers/treasury/received_debits", self, http_types::Method::Post)
    }
}
