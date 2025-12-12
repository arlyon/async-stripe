use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListTreasuryReceivedCreditBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linked_flows: Option<ListTreasuryReceivedCreditLinkedFlows>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_treasury::TreasuryReceivedCreditStatus>,
}
impl ListTreasuryReceivedCreditBuilder {
    fn new(financial_account: impl Into<String>) -> Self {
        Self {
            ending_before: None,
            expand: None,
            financial_account: financial_account.into(),
            limit: None,
            linked_flows: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Only return ReceivedCredits described by the flow.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryReceivedCreditLinkedFlows {
    /// The source flow type.
    pub source_flow_type: ListTreasuryReceivedCreditLinkedFlowsSourceFlowType,
}
impl ListTreasuryReceivedCreditLinkedFlows {
    pub fn new(
        source_flow_type: impl Into<ListTreasuryReceivedCreditLinkedFlowsSourceFlowType>,
    ) -> Self {
        Self { source_flow_type: source_flow_type.into() }
    }
}
/// The source flow type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    CreditReversal,
    Other,
    OutboundPayment,
    OutboundTransfer,
    Payout,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    pub fn as_str(&self) -> &str {
        use ListTreasuryReceivedCreditLinkedFlowsSourceFlowType::*;
        match self {
            CreditReversal => "credit_reversal",
            Other => "other",
            OutboundPayment => "outbound_payment",
            OutboundTransfer => "outbound_transfer",
            Payout => "payout",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ListTreasuryReceivedCreditLinkedFlowsSourceFlowType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryReceivedCreditLinkedFlowsSourceFlowType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "outbound_transfer" => Ok(OutboundTransfer),
            "payout" => Ok(Payout),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ListTreasuryReceivedCreditLinkedFlowsSourceFlowType"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Returns a list of ReceivedCredits.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryReceivedCredit {
    inner: ListTreasuryReceivedCreditBuilder,
}
impl ListTreasuryReceivedCredit {
    /// Construct a new `ListTreasuryReceivedCredit`.
    pub fn new(financial_account: impl Into<String>) -> Self {
        Self { inner: ListTreasuryReceivedCreditBuilder::new(financial_account.into()) }
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
    /// Only return ReceivedCredits described by the flow.
    pub fn linked_flows(
        mut self,
        linked_flows: impl Into<ListTreasuryReceivedCreditLinkedFlows>,
    ) -> Self {
        self.inner.linked_flows = Some(linked_flows.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return ReceivedCredits that have the given status: `succeeded` or `failed`.
    pub fn status(
        mut self,
        status: impl Into<stripe_treasury::TreasuryReceivedCreditStatus>,
    ) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl ListTreasuryReceivedCredit {
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
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_treasury::TreasuryReceivedCredit>,
    > {
        stripe_client_core::ListPaginator::new_list("/treasury/received_credits", &self.inner)
    }
}

impl StripeRequest for ListTreasuryReceivedCredit {
    type Output = stripe_types::List<stripe_treasury::TreasuryReceivedCredit>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/received_credits").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryReceivedCreditBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTreasuryReceivedCreditBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing ReceivedCredit by passing the unique ReceivedCredit ID from the ReceivedCredit list.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryReceivedCredit {
    inner: RetrieveTreasuryReceivedCreditBuilder,
    id: stripe_treasury::TreasuryReceivedCreditId,
}
impl RetrieveTreasuryReceivedCredit {
    /// Construct a new `RetrieveTreasuryReceivedCredit`.
    pub fn new(id: impl Into<stripe_treasury::TreasuryReceivedCreditId>) -> Self {
        Self { id: id.into(), inner: RetrieveTreasuryReceivedCreditBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTreasuryReceivedCredit {
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

impl StripeRequest for RetrieveTreasuryReceivedCredit {
    type Output = stripe_treasury::TreasuryReceivedCredit;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/treasury/received_credits/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateTreasuryReceivedCreditBuilder {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    initiating_payment_method_details:
        Option<CreateTreasuryReceivedCreditInitiatingPaymentMethodDetails>,
    network: CreateTreasuryReceivedCreditNetwork,
}
impl CreateTreasuryReceivedCreditBuilder {
    fn new(
        amount: impl Into<i64>,
        currency: impl Into<stripe_types::Currency>,
        financial_account: impl Into<String>,
        network: impl Into<CreateTreasuryReceivedCreditNetwork>,
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
pub struct CreateTreasuryReceivedCreditInitiatingPaymentMethodDetails {
    /// The source type.
    #[serde(rename = "type")]
    pub type_: CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType,
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount>,
}
impl CreateTreasuryReceivedCreditInitiatingPaymentMethodDetails {
    pub fn new(
        type_: impl Into<CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType>,
    ) -> Self {
        Self { type_: type_.into(), us_bank_account: None }
    }
}
/// The source type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType {
    UsBankAccount,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType {
    pub fn as_str(&self) -> &str {
        use CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType::*;
        match self {
            UsBankAccount => "us_bank_account",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount {
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
impl CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount {
    pub fn new() -> Self {
        Self { account_holder_name: None, account_number: None, routing_number: None }
    }
}
impl Default for CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Specifies the network rails to be used.
/// If not set, will default to the PaymentMethod's preferred network.
/// See the [docs](https://docs.stripe.com/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTreasuryReceivedCreditNetwork {
    Ach,
    UsDomesticWire,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTreasuryReceivedCreditNetwork {
    pub fn as_str(&self) -> &str {
        use CreateTreasuryReceivedCreditNetwork::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTreasuryReceivedCreditNetwork {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedCreditNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTreasuryReceivedCreditNetwork"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Use this endpoint to simulate a test mode ReceivedCredit initiated by a third party.
/// In live mode, you can’t directly create ReceivedCredits initiated by third parties.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedCredit {
    inner: CreateTreasuryReceivedCreditBuilder,
}
impl CreateTreasuryReceivedCredit {
    /// Construct a new `CreateTreasuryReceivedCredit`.
    pub fn new(
        amount: impl Into<i64>,
        currency: impl Into<stripe_types::Currency>,
        financial_account: impl Into<String>,
        network: impl Into<CreateTreasuryReceivedCreditNetwork>,
    ) -> Self {
        Self {
            inner: CreateTreasuryReceivedCreditBuilder::new(
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
            CreateTreasuryReceivedCreditInitiatingPaymentMethodDetails,
        >,
    ) -> Self {
        self.inner.initiating_payment_method_details =
            Some(initiating_payment_method_details.into());
        self
    }
}
impl CreateTreasuryReceivedCredit {
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

impl StripeRequest for CreateTreasuryReceivedCredit {
    type Output = stripe_treasury::TreasuryReceivedCredit;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/test_helpers/treasury/received_credits")
            .form(&self.inner)
    }
}
