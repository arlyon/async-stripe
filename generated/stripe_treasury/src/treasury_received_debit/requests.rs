use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListTreasuryReceivedDebitBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    financial_account: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_treasury::TreasuryReceivedDebitStatus>,
}
impl<'a> ListTreasuryReceivedDebitBuilder<'a> {
    fn new(financial_account: &'a str) -> Self {
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
/// Returns a list of ReceivedDebits.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryReceivedDebit<'a> {
    inner: ListTreasuryReceivedDebitBuilder<'a>,
}
impl<'a> ListTreasuryReceivedDebit<'a> {
    /// Construct a new `ListTreasuryReceivedDebit`.
    pub fn new(financial_account: &'a str) -> Self {
        Self { inner: ListTreasuryReceivedDebitBuilder::new(financial_account) }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// Only return ReceivedDebits that have the given status: `succeeded` or `failed`.
    pub fn status(mut self, status: stripe_treasury::TreasuryReceivedDebitStatus) -> Self {
        self.inner.status = Some(status);
        self
    }
}
impl ListTreasuryReceivedDebit<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_treasury::TreasuryReceivedDebit>>
    {
        stripe_client_core::ListPaginator::new_list("/treasury/received_debits", self.inner)
    }
}

impl StripeRequest for ListTreasuryReceivedDebit<'_> {
    type Output = stripe_types::List<stripe_treasury::TreasuryReceivedDebit>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/received_debits").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryReceivedDebitBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryReceivedDebitBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing ReceivedDebit by passing the unique ReceivedDebit ID from the ReceivedDebit list.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryReceivedDebit<'a> {
    inner: RetrieveTreasuryReceivedDebitBuilder<'a>,
    id: &'a stripe_treasury::TreasuryReceivedDebitId,
}
impl<'a> RetrieveTreasuryReceivedDebit<'a> {
    /// Construct a new `RetrieveTreasuryReceivedDebit`.
    pub fn new(id: &'a stripe_treasury::TreasuryReceivedDebitId) -> Self {
        Self { id, inner: RetrieveTreasuryReceivedDebitBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTreasuryReceivedDebit<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveTreasuryReceivedDebit<'_> {
    type Output = stripe_treasury::TreasuryReceivedDebit;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/treasury/received_debits/{id}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateTreasuryReceivedDebitBuilder<'a> {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    financial_account: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    initiating_payment_method_details:
        Option<CreateTreasuryReceivedDebitInitiatingPaymentMethodDetails<'a>>,
    network: CreateTreasuryReceivedDebitNetwork,
}
impl<'a> CreateTreasuryReceivedDebitBuilder<'a> {
    fn new(
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType",
            )
        })
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
        Self { account_holder_name: None, account_number: None, routing_number: None }
    }
}
impl<'a> Default for CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Specifies the network rails to be used.
/// If not set, will default to the PaymentMethod's preferred network.
/// See the [docs](https://stripe.com/docs/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedDebitNetwork::*;
        match s {
            "ach" => Ok(Ach),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTreasuryReceivedDebitNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTreasuryReceivedDebitNetwork")
        })
    }
}
/// Use this endpoint to simulate a test mode ReceivedDebit initiated by a third party.
/// In live mode, you can’t directly create ReceivedDebits initiated by third parties.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedDebit<'a> {
    inner: CreateTreasuryReceivedDebitBuilder<'a>,
}
impl<'a> CreateTreasuryReceivedDebit<'a> {
    /// Construct a new `CreateTreasuryReceivedDebit`.
    pub fn new(
        amount: i64,
        currency: stripe_types::Currency,
        financial_account: &'a str,
        network: CreateTreasuryReceivedDebitNetwork,
    ) -> Self {
        Self {
            inner: CreateTreasuryReceivedDebitBuilder::new(
                amount,
                currency,
                financial_account,
                network,
            ),
        }
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Initiating payment method details for the object.
    pub fn initiating_payment_method_details(
        mut self,
        initiating_payment_method_details: CreateTreasuryReceivedDebitInitiatingPaymentMethodDetails<'a>,
    ) -> Self {
        self.inner.initiating_payment_method_details = Some(initiating_payment_method_details);
        self
    }
}
impl CreateTreasuryReceivedDebit<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateTreasuryReceivedDebit<'_> {
    type Output = stripe_treasury::TreasuryReceivedDebit;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/test_helpers/treasury/received_debits")
            .form(&self.inner)
    }
}
