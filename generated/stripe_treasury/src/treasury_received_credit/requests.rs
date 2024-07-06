use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListTreasuryReceivedCreditBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    financial_account: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linked_flows: Option<ListTreasuryReceivedCreditLinkedFlows>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_treasury::TreasuryReceivedCreditStatus>,
}
impl<'a> ListTreasuryReceivedCreditBuilder<'a> {
    fn new(financial_account: &'a str) -> Self {
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryReceivedCreditLinkedFlowsSourceFlowType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "payout" => Ok(Payout),
            _ => Err(stripe_types::StripeParseError),
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
/// Returns a list of ReceivedCredits.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryReceivedCredit<'a> {
    inner: ListTreasuryReceivedCreditBuilder<'a>,
}
impl<'a> ListTreasuryReceivedCredit<'a> {
    /// Construct a new `ListTreasuryReceivedCredit`.
    pub fn new(financial_account: &'a str) -> Self {
        Self { inner: ListTreasuryReceivedCreditBuilder::new(financial_account) }
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
    /// Only return ReceivedCredits described by the flow.
    pub fn linked_flows(mut self, linked_flows: ListTreasuryReceivedCreditLinkedFlows) -> Self {
        self.inner.linked_flows = Some(linked_flows);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// Only return ReceivedCredits that have the given status: `succeeded` or `failed`.
    pub fn status(mut self, status: stripe_treasury::TreasuryReceivedCreditStatus) -> Self {
        self.inner.status = Some(status);
        self
    }
}
impl ListTreasuryReceivedCredit<'_> {
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
        stripe_client_core::ListPaginator::new_list("/treasury/received_credits", self.inner)
    }
}

impl StripeRequest for ListTreasuryReceivedCredit<'_> {
    type Output = stripe_types::List<stripe_treasury::TreasuryReceivedCredit>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/received_credits").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryReceivedCreditBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTreasuryReceivedCreditBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing ReceivedCredit by passing the unique ReceivedCredit ID from the ReceivedCredit list.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryReceivedCredit<'a> {
    inner: RetrieveTreasuryReceivedCreditBuilder<'a>,
    id: &'a stripe_treasury::TreasuryReceivedCreditId,
}
impl<'a> RetrieveTreasuryReceivedCredit<'a> {
    /// Construct a new `RetrieveTreasuryReceivedCredit`.
    pub fn new(id: &'a stripe_treasury::TreasuryReceivedCreditId) -> Self {
        Self { id, inner: RetrieveTreasuryReceivedCreditBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveTreasuryReceivedCredit<'_> {
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

impl StripeRequest for RetrieveTreasuryReceivedCredit<'_> {
    type Output = stripe_treasury::TreasuryReceivedCredit;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/treasury/received_credits/{id}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateTreasuryReceivedCreditBuilder<'a> {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    financial_account: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    initiating_payment_method_details:
        Option<CreateTreasuryReceivedCreditInitiatingPaymentMethodDetails<'a>>,
    network: CreateTreasuryReceivedCreditNetwork,
}
impl<'a> CreateTreasuryReceivedCreditBuilder<'a> {
    fn new(
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(stripe_types::StripeParseError),
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
        Self { account_holder_name: None, account_number: None, routing_number: None }
    }
}
impl<'a> Default for CreateTreasuryReceivedCreditInitiatingPaymentMethodDetailsUsBankAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Specifies the network rails to be used.
/// If not set, will default to the PaymentMethod's preferred network.
/// See the [docs](https://stripe.com/docs/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryReceivedCreditNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(stripe_types::StripeParseError),
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
/// Use this endpoint to simulate a test mode ReceivedCredit initiated by a third party.
/// In live mode, you can’t directly create ReceivedCredits initiated by third parties.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryReceivedCredit<'a> {
    inner: CreateTreasuryReceivedCreditBuilder<'a>,
}
impl<'a> CreateTreasuryReceivedCredit<'a> {
    /// Construct a new `CreateTreasuryReceivedCredit`.
    pub fn new(
        amount: i64,
        currency: stripe_types::Currency,
        financial_account: &'a str,
        network: CreateTreasuryReceivedCreditNetwork,
    ) -> Self {
        Self {
            inner: CreateTreasuryReceivedCreditBuilder::new(
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
        initiating_payment_method_details: CreateTreasuryReceivedCreditInitiatingPaymentMethodDetails<'a>,
    ) -> Self {
        self.inner.initiating_payment_method_details = Some(initiating_payment_method_details);
        self
    }
}
impl CreateTreasuryReceivedCredit<'_> {
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

impl StripeRequest for CreateTreasuryReceivedCredit<'_> {
    type Output = stripe_treasury::TreasuryReceivedCredit;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/test_helpers/treasury/received_credits")
            .form(&self.inner)
    }
}
