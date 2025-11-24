use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListTreasuryReceivedDebitBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_treasury::TreasuryReceivedDebitStatus>,
}
impl ListTreasuryReceivedDebitBuilder {
    fn new(financial_account: impl Into<String>) -> Self {
        Self {
            ending_before: None,
            expand: None,
            financial_account: financial_account.into(),
            limit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Returns a list of ReceivedDebits.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryReceivedDebit {
    inner: ListTreasuryReceivedDebitBuilder,
}
impl ListTreasuryReceivedDebit {
    /// Construct a new `ListTreasuryReceivedDebit`.
    pub fn new(financial_account: impl Into<String>) -> Self {
        Self { inner: ListTreasuryReceivedDebitBuilder::new(financial_account.into()) }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return ReceivedDebits that have the given status: `succeeded` or `failed`.
    pub fn status(
        mut self,
        status: impl Into<stripe_treasury::TreasuryReceivedDebitStatus>,
    ) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl ListTreasuryReceivedDebit {
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
        stripe_client_core::ListPaginator::new_list("/treasury/received_debits", &self.inner)
    }
}

impl StripeRequest for ListTreasuryReceivedDebit {
    type Output = stripe_types::List<stripe_treasury::TreasuryReceivedDebit>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/received_debits").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryReceivedDebitBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTreasuryReceivedDebitBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing ReceivedDebit by passing the unique ReceivedDebit ID from the ReceivedDebit list.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryReceivedDebit {
    inner: RetrieveTreasuryReceivedDebitBuilder,
    id: stripe_treasury::TreasuryReceivedDebitId,
}
impl RetrieveTreasuryReceivedDebit {
    /// Construct a new `RetrieveTreasuryReceivedDebit`.
    pub fn new(id: impl Into<stripe_treasury::TreasuryReceivedDebitId>) -> Self {
        Self { id: id.into(), inner: RetrieveTreasuryReceivedDebitBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTreasuryReceivedDebit {
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

impl StripeRequest for RetrieveTreasuryReceivedDebit {
    type Output = stripe_treasury::TreasuryReceivedDebit;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/treasury/received_debits/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateTreasuryReceivedDebitBuilder {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    initiating_payment_method_details:
        Option<CreateTreasuryReceivedDebitInitiatingPaymentMethodDetails>,
    network: CreateTreasuryReceivedDebitNetwork,
}
impl CreateTreasuryReceivedDebitBuilder {
    fn new(
        amount: impl Into<i64>,
        currency: impl Into<stripe_types::Currency>,
        financial_account: impl Into<String>,
        network: impl Into<CreateTreasuryReceivedDebitNetwork>,
    ) -> Self {
        Self {
            amount: amount.into(),
            currency: currency.into(),
            description: None,
            expand: None,
            financial_account: financial_account.into(),
            initiating_payment_method_details: None,
            network: network.into(),
        }
    }
}
/// Initiating payment method details for the object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedDebitInitiatingPaymentMethodDetails {
    /// The source type.
    #[serde(rename = "type")]
    pub type_: CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType,
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount>,
}
impl CreateTreasuryReceivedDebitInitiatingPaymentMethodDetails {
    pub fn new(
        type_: impl Into<CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType>,
    ) -> Self {
        Self { type_: type_.into(), us_bank_account: None }
    }
}
/// The source type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType {
    UsBankAccount,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType {
    pub fn as_str(&self) -> &str {
        use CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType::*;
        match self {
            UsBankAccount => "us_bank_account",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount {
    /// The bank account holder's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    /// The bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// The bank account's routing number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
impl CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount {
    pub fn new() -> Self {
        Self { account_holder_name: None, account_number: None, routing_number: None }
    }
}
impl Default for CreateTreasuryReceivedDebitInitiatingPaymentMethodDetailsUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Specifies the network rails to be used.
/// If not set, will default to the PaymentMethod's preferred network.
/// See the [docs](https://stripe.com/docs/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTreasuryReceivedDebitNetwork {
    Ach,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTreasuryReceivedDebitNetwork {
    pub fn as_str(&self) -> &str {
        use CreateTreasuryReceivedDebitNetwork::*;
        match self {
            Ach => "ach",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTreasuryReceivedDebitNetwork {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedDebitNetwork::*;
        match s {
            "ach" => Ok(Ach),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTreasuryReceivedDebitNetwork"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Use this endpoint to simulate a test mode ReceivedDebit initiated by a third party.
/// In live mode, you can’t directly create ReceivedDebits initiated by third parties.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedDebit {
    inner: CreateTreasuryReceivedDebitBuilder,
}
impl CreateTreasuryReceivedDebit {
    /// Construct a new `CreateTreasuryReceivedDebit`.
    pub fn new(
        amount: impl Into<i64>,
        currency: impl Into<stripe_types::Currency>,
        financial_account: impl Into<String>,
        network: impl Into<CreateTreasuryReceivedDebitNetwork>,
    ) -> Self {
        Self {
            inner: CreateTreasuryReceivedDebitBuilder::new(
                amount.into(),
                currency.into(),
                financial_account.into(),
                network.into(),
            ),
        }
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Initiating payment method details for the object.
    pub fn initiating_payment_method_details(
        mut self,
        initiating_payment_method_details: impl Into<
            CreateTreasuryReceivedDebitInitiatingPaymentMethodDetails,
        >,
    ) -> Self {
        self.inner.initiating_payment_method_details =
            Some(initiating_payment_method_details.into());
        self
    }
}
impl CreateTreasuryReceivedDebit {
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

impl StripeRequest for CreateTreasuryReceivedDebit {
    type Output = stripe_treasury::TreasuryReceivedDebit;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/test_helpers/treasury/received_debits")
            .form(&self.inner)
    }
}
