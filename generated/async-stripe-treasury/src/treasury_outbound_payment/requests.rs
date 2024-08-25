use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListTreasuryOutboundPaymentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
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
    status: Option<stripe_treasury::TreasuryOutboundPaymentStatus>,
}
impl ListTreasuryOutboundPaymentBuilder {
    fn new(financial_account: impl Into<String>) -> Self {
        Self {
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            financial_account: financial_account.into(),
            limit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Returns a list of OutboundPayments sent from the specified FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListTreasuryOutboundPayment {
    inner: ListTreasuryOutboundPaymentBuilder,
}
impl ListTreasuryOutboundPayment {
    /// Construct a new `ListTreasuryOutboundPayment`.
    pub fn new(financial_account: impl Into<String>) -> Self {
        Self { inner: ListTreasuryOutboundPaymentBuilder::new(financial_account.into()) }
    }
    /// Only return OutboundPayments that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return OutboundPayments sent to this customer.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
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
    /// Only return OutboundPayments that have the given status: `processing`, `failed`, `posted`, `returned`, or `canceled`.
    pub fn status(
        mut self,
        status: impl Into<stripe_treasury::TreasuryOutboundPaymentStatus>,
    ) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl ListTreasuryOutboundPayment {
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
        stripe_types::List<stripe_treasury::TreasuryOutboundPayment>,
    > {
        stripe_client_core::ListPaginator::new_list("/treasury/outbound_payments", &self.inner)
    }
}

impl StripeRequest for ListTreasuryOutboundPayment {
    type Output = stripe_types::List<stripe_treasury::TreasuryOutboundPayment>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/outbound_payments").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveTreasuryOutboundPaymentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTreasuryOutboundPaymentBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing OutboundPayment by passing the unique OutboundPayment ID from either the OutboundPayment creation request or OutboundPayment list.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveTreasuryOutboundPayment {
    inner: RetrieveTreasuryOutboundPaymentBuilder,
    id: stripe_treasury::TreasuryOutboundPaymentId,
}
impl RetrieveTreasuryOutboundPayment {
    /// Construct a new `RetrieveTreasuryOutboundPayment`.
    pub fn new(id: impl Into<stripe_treasury::TreasuryOutboundPaymentId>) -> Self {
        Self { id: id.into(), inner: RetrieveTreasuryOutboundPaymentBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTreasuryOutboundPayment {
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

impl StripeRequest for RetrieveTreasuryOutboundPayment {
    type Output = stripe_treasury::TreasuryOutboundPayment;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/treasury/outbound_payments/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct FailTreasuryOutboundPaymentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl FailTreasuryOutboundPaymentBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Transitions a test mode created OutboundPayment to the `failed` status.
/// The OutboundPayment must already be in the `processing` state.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct FailTreasuryOutboundPayment {
    inner: FailTreasuryOutboundPaymentBuilder,
    id: String,
}
impl FailTreasuryOutboundPayment {
    /// Construct a new `FailTreasuryOutboundPayment`.
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into(), inner: FailTreasuryOutboundPaymentBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl FailTreasuryOutboundPayment {
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

impl StripeRequest for FailTreasuryOutboundPayment {
    type Output = stripe_treasury::TreasuryOutboundPayment;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/outbound_payments/{id}/fail"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct PostTreasuryOutboundPaymentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl PostTreasuryOutboundPaymentBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Transitions a test mode created OutboundPayment to the `posted` status.
/// The OutboundPayment must already be in the `processing` state.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct PostTreasuryOutboundPayment {
    inner: PostTreasuryOutboundPaymentBuilder,
    id: String,
}
impl PostTreasuryOutboundPayment {
    /// Construct a new `PostTreasuryOutboundPayment`.
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into(), inner: PostTreasuryOutboundPaymentBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl PostTreasuryOutboundPayment {
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

impl StripeRequest for PostTreasuryOutboundPayment {
    type Output = stripe_treasury::TreasuryOutboundPayment;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/outbound_payments/{id}/post"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ReturnOutboundPaymentTreasuryOutboundPaymentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    returned_details: Option<ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetails>,
}
impl ReturnOutboundPaymentTreasuryOutboundPaymentBuilder {
    fn new() -> Self {
        Self { expand: None, returned_details: None }
    }
}
/// Optional hash to set the the return code.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetails {
    /// The return code to be set on the OutboundPayment object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetailsCode>,
}
impl ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetails {
    pub fn new() -> Self {
        Self { code: None }
    }
}
impl Default for ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The return code to be set on the OutboundPayment object.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    Declined,
    IncorrectAccountHolderName,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
}
impl ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetailsCode {
    pub fn as_str(self) -> &'static str {
        use ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetailsCode::*;
        match self {
            AccountClosed => "account_closed",
            AccountFrozen => "account_frozen",
            BankAccountRestricted => "bank_account_restricted",
            BankOwnershipChanged => "bank_ownership_changed",
            Declined => "declined",
            IncorrectAccountHolderName => "incorrect_account_holder_name",
            InvalidAccountNumber => "invalid_account_number",
            InvalidCurrency => "invalid_currency",
            NoAccount => "no_account",
            Other => "other",
        }
    }
}

impl std::str::FromStr for ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetailsCode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetailsCode::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "account_frozen" => Ok(AccountFrozen),
            "bank_account_restricted" => Ok(BankAccountRestricted),
            "bank_ownership_changed" => Ok(BankOwnershipChanged),
            "declined" => Ok(Declined),
            "incorrect_account_holder_name" => Ok(IncorrectAccountHolderName),
            "invalid_account_number" => Ok(InvalidAccountNumber),
            "invalid_currency" => Ok(InvalidCurrency),
            "no_account" => Ok(NoAccount),
            "other" => Ok(Other),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetailsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetailsCode
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetailsCode",
            )
        })
    }
}
/// Transitions a test mode created OutboundPayment to the `returned` status.
/// The OutboundPayment must already be in the `processing` state.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ReturnOutboundPaymentTreasuryOutboundPayment {
    inner: ReturnOutboundPaymentTreasuryOutboundPaymentBuilder,
    id: String,
}
impl ReturnOutboundPaymentTreasuryOutboundPayment {
    /// Construct a new `ReturnOutboundPaymentTreasuryOutboundPayment`.
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into(), inner: ReturnOutboundPaymentTreasuryOutboundPaymentBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Optional hash to set the the return code.
    pub fn returned_details(
        mut self,
        returned_details: impl Into<ReturnOutboundPaymentTreasuryOutboundPaymentReturnedDetails>,
    ) -> Self {
        self.inner.returned_details = Some(returned_details.into());
        self
    }
}
impl ReturnOutboundPaymentTreasuryOutboundPayment {
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

impl StripeRequest for ReturnOutboundPaymentTreasuryOutboundPayment {
    type Output = stripe_treasury::TreasuryOutboundPayment;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/treasury/outbound_payments/{id}/return"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateTreasuryOutboundPaymentBuilder {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_payment_method_data:
        Option<CreateTreasuryOutboundPaymentDestinationPaymentMethodData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_payment_method_options:
        Option<CreateTreasuryOutboundPaymentDestinationPaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_user_details: Option<CreateTreasuryOutboundPaymentEndUserDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<String>,
}
impl CreateTreasuryOutboundPaymentBuilder {
    fn new(
        amount: impl Into<i64>,
        currency: impl Into<stripe_types::Currency>,
        financial_account: impl Into<String>,
    ) -> Self {
        Self {
            amount: amount.into(),
            currency: currency.into(),
            customer: None,
            description: None,
            destination_payment_method: None,
            destination_payment_method_data: None,
            destination_payment_method_options: None,
            end_user_details: None,
            expand: None,
            financial_account: financial_account.into(),
            metadata: None,
            statement_descriptor: None,
        }
    }
}
/// Hash used to generate the PaymentMethod to be used for this OutboundPayment.
/// Exclusive with `destination_payment_method`.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTreasuryOutboundPaymentDestinationPaymentMethodData {
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details:
        Option<CreateTreasuryOutboundPaymentDestinationPaymentMethodDataBillingDetails>,
    /// Required if type is set to `financial_account`. The FinancialAccount ID to send funds to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: CreateTreasuryOutboundPaymentDestinationPaymentMethodDataType,
    /// Required hash if type is set to `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccount>,
}
impl CreateTreasuryOutboundPaymentDestinationPaymentMethodData {
    pub fn new(
        type_: impl Into<CreateTreasuryOutboundPaymentDestinationPaymentMethodDataType>,
    ) -> Self {
        Self {
            billing_details: None,
            financial_account: None,
            metadata: None,
            type_: type_.into(),
            us_bank_account: None,
        }
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTreasuryOutboundPaymentDestinationPaymentMethodDataBillingDetails {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address:
        Option<CreateTreasuryOutboundPaymentDestinationPaymentMethodDataBillingDetailsAddress>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl CreateTreasuryOutboundPaymentDestinationPaymentMethodDataBillingDetails {
    pub fn new() -> Self {
        Self { address: None, email: None, name: None, phone: None }
    }
}
impl Default for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataBillingDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Billing address.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTreasuryOutboundPaymentDestinationPaymentMethodDataBillingDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl CreateTreasuryOutboundPaymentDestinationPaymentMethodDataBillingDetailsAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataBillingDetailsAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryOutboundPaymentDestinationPaymentMethodDataType {
    FinancialAccount,
    UsBankAccount,
}
impl CreateTreasuryOutboundPaymentDestinationPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryOutboundPaymentDestinationPaymentMethodDataType::*;
        match self {
            FinancialAccount => "financial_account",
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryOutboundPaymentDestinationPaymentMethodDataType::*;
        match s {
            "financial_account" => Ok(FinancialAccount),
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataType",
            )
        })
    }
}
/// Required hash if type is set to `us_bank_account`.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccount {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<
        CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType,
    >,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type:
        Option<CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
impl CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccount {
    pub fn new() -> Self {
        Self {
            account_holder_type: None,
            account_number: None,
            account_type: None,
            financial_connections_account: None,
            routing_number: None,
        }
    }
}
impl Default for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}
impl CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountHolderType"))
    }
}
/// Account type: checkings or savings. Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}
impl CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTreasuryOutboundPaymentDestinationPaymentMethodDataUsBankAccountAccountType"))
    }
}
/// Payment method-specific configuration for this OutboundPayment.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTreasuryOutboundPaymentDestinationPaymentMethodOptions {
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccount>,
}
impl CreateTreasuryOutboundPaymentDestinationPaymentMethodOptions {
    pub fn new() -> Self {
        Self { us_bank_account: None }
    }
}
impl Default for CreateTreasuryOutboundPaymentDestinationPaymentMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccount {
    /// Specifies the network rails to be used.
    /// If not set, will default to the PaymentMethod's preferred network.
    /// See the [docs](https://stripe.com/docs/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network:
        Option<CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork>,
}
impl CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccount {
    pub fn new() -> Self {
        Self { network: None }
    }
}
impl Default for CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Specifies the network rails to be used.
/// If not set, will default to the PaymentMethod's preferred network.
/// See the [docs](https://stripe.com/docs/treasury/money-movement/timelines) to learn more about money movement timelines for each network type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}
impl CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTreasuryOutboundPaymentDestinationPaymentMethodOptionsUsBankAccountNetwork"))
    }
}
/// End user details.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTreasuryOutboundPaymentEndUserDetails {
    /// IP address of the user initiating the OutboundPayment.
    /// Must be supplied if `present` is set to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// `True` if the OutboundPayment creation request is being made on behalf of an end user by a platform.
    /// Otherwise, `false`.
    pub present: bool,
}
impl CreateTreasuryOutboundPaymentEndUserDetails {
    pub fn new(present: impl Into<bool>) -> Self {
        Self { ip_address: None, present: present.into() }
    }
}
/// Creates an OutboundPayment.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTreasuryOutboundPayment {
    inner: CreateTreasuryOutboundPaymentBuilder,
}
impl CreateTreasuryOutboundPayment {
    /// Construct a new `CreateTreasuryOutboundPayment`.
    pub fn new(
        amount: impl Into<i64>,
        currency: impl Into<stripe_types::Currency>,
        financial_account: impl Into<String>,
    ) -> Self {
        Self {
            inner: CreateTreasuryOutboundPaymentBuilder::new(
                amount.into(),
                currency.into(),
                financial_account.into(),
            ),
        }
    }
    /// ID of the customer to whom the OutboundPayment is sent.
    /// Must match the Customer attached to the `destination_payment_method` passed in.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// The PaymentMethod to use as the payment instrument for the OutboundPayment.
    /// Exclusive with `destination_payment_method_data`.
    pub fn destination_payment_method(
        mut self,
        destination_payment_method: impl Into<String>,
    ) -> Self {
        self.inner.destination_payment_method = Some(destination_payment_method.into());
        self
    }
    /// Hash used to generate the PaymentMethod to be used for this OutboundPayment.
    /// Exclusive with `destination_payment_method`.
    pub fn destination_payment_method_data(
        mut self,
        destination_payment_method_data: impl Into<
            CreateTreasuryOutboundPaymentDestinationPaymentMethodData,
        >,
    ) -> Self {
        self.inner.destination_payment_method_data = Some(destination_payment_method_data.into());
        self
    }
    /// Payment method-specific configuration for this OutboundPayment.
    pub fn destination_payment_method_options(
        mut self,
        destination_payment_method_options: impl Into<
            CreateTreasuryOutboundPaymentDestinationPaymentMethodOptions,
        >,
    ) -> Self {
        self.inner.destination_payment_method_options =
            Some(destination_payment_method_options.into());
        self
    }
    /// End user details.
    pub fn end_user_details(
        mut self,
        end_user_details: impl Into<CreateTreasuryOutboundPaymentEndUserDetails>,
    ) -> Self {
        self.inner.end_user_details = Some(end_user_details.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The description that appears on the receiving end for this OutboundPayment (for example, bank statement for external bank transfer).
    /// Maximum 10 characters for `ach` payments, 140 characters for `us_domestic_wire` payments, or 500 characters for `stripe` network transfers.
    /// The default value is "payment".
    pub fn statement_descriptor(mut self, statement_descriptor: impl Into<String>) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor.into());
        self
    }
}
impl CreateTreasuryOutboundPayment {
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

impl StripeRequest for CreateTreasuryOutboundPayment {
    type Output = stripe_treasury::TreasuryOutboundPayment;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/treasury/outbound_payments").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CancelTreasuryOutboundPaymentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CancelTreasuryOutboundPaymentBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Cancel an OutboundPayment.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CancelTreasuryOutboundPayment {
    inner: CancelTreasuryOutboundPaymentBuilder,
    id: stripe_treasury::TreasuryOutboundPaymentId,
}
impl CancelTreasuryOutboundPayment {
    /// Construct a new `CancelTreasuryOutboundPayment`.
    pub fn new(id: impl Into<stripe_treasury::TreasuryOutboundPaymentId>) -> Self {
        Self { id: id.into(), inner: CancelTreasuryOutboundPaymentBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CancelTreasuryOutboundPayment {
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

impl StripeRequest for CancelTreasuryOutboundPayment {
    type Output = stripe_treasury::TreasuryOutboundPayment;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/treasury/outbound_payments/{id}/cancel"))
            .form(&self.inner)
    }
}
