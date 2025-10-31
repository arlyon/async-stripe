use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListInvoicePaymentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment: Option<ListInvoicePaymentPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<ListInvoicePaymentStatus>,
}
impl ListInvoicePaymentBuilder {
    fn new() -> Self {
        Self {
            ending_before: None,
            expand: None,
            invoice: None,
            limit: None,
            payment: None,
            starting_after: None,
            status: None,
        }
    }
}
/// The payment details of the invoice payments to return.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListInvoicePaymentPayment {
    /// Only return invoice payments associated by this payment intent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<String>,
    /// Only return invoice payments associated by this payment record ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_record: Option<String>,
    /// Only return invoice payments associated by this payment type.
    #[serde(rename = "type")]
    pub type_: ListInvoicePaymentPaymentType,
}
impl ListInvoicePaymentPayment {
    pub fn new(type_: impl Into<ListInvoicePaymentPaymentType>) -> Self {
        Self { payment_intent: None, payment_record: None, type_: type_.into() }
    }
}
/// Only return invoice payments associated by this payment type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListInvoicePaymentPaymentType {
    PaymentIntent,
    PaymentRecord,
}
impl ListInvoicePaymentPaymentType {
    pub fn as_str(self) -> &'static str {
        use ListInvoicePaymentPaymentType::*;
        match self {
            PaymentIntent => "payment_intent",
            PaymentRecord => "payment_record",
        }
    }
}

impl std::str::FromStr for ListInvoicePaymentPaymentType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListInvoicePaymentPaymentType::*;
        match s {
            "payment_intent" => Ok(PaymentIntent),
            "payment_record" => Ok(PaymentRecord),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ListInvoicePaymentPaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListInvoicePaymentPaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListInvoicePaymentPaymentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListInvoicePaymentPaymentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ListInvoicePaymentPaymentType")
        })
    }
}
/// The status of the invoice payments to return.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListInvoicePaymentStatus {
    Canceled,
    Open,
    Paid,
}
impl ListInvoicePaymentStatus {
    pub fn as_str(self) -> &'static str {
        use ListInvoicePaymentStatus::*;
        match self {
            Canceled => "canceled",
            Open => "open",
            Paid => "paid",
        }
    }
}

impl std::str::FromStr for ListInvoicePaymentStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListInvoicePaymentStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "open" => Ok(Open),
            "paid" => Ok(Paid),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ListInvoicePaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListInvoicePaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListInvoicePaymentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListInvoicePaymentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ListInvoicePaymentStatus"))
    }
}
/// When retrieving an invoice, there is an includable payments property containing the first handful of those items.
/// There is also a URL where you can retrieve the full (paginated) list of payments.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListInvoicePayment {
    inner: ListInvoicePaymentBuilder,
}
impl ListInvoicePayment {
    /// Construct a new `ListInvoicePayment`.
    pub fn new() -> Self {
        Self { inner: ListInvoicePaymentBuilder::new() }
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
    /// The identifier of the invoice whose payments to return.
    pub fn invoice(mut self, invoice: impl Into<String>) -> Self {
        self.inner.invoice = Some(invoice.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// The payment details of the invoice payments to return.
    pub fn payment(mut self, payment: impl Into<ListInvoicePaymentPayment>) -> Self {
        self.inner.payment = Some(payment.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// The status of the invoice payments to return.
    pub fn status(mut self, status: impl Into<ListInvoicePaymentStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListInvoicePayment {
    fn default() -> Self {
        Self::new()
    }
}
impl ListInvoicePayment {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::InvoicePayment>> {
        stripe_client_core::ListPaginator::new_list("/invoice_payments", &self.inner)
    }
}

impl StripeRequest for ListInvoicePayment {
    type Output = stripe_types::List<stripe_shared::InvoicePayment>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/invoice_payments").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveInvoicePaymentBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveInvoicePaymentBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the invoice payment with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveInvoicePayment {
    inner: RetrieveInvoicePaymentBuilder,
    invoice_payment: stripe_shared::InvoicePaymentId,
}
impl RetrieveInvoicePayment {
    /// Construct a new `RetrieveInvoicePayment`.
    pub fn new(invoice_payment: impl Into<stripe_shared::InvoicePaymentId>) -> Self {
        Self {
            invoice_payment: invoice_payment.into(),
            inner: RetrieveInvoicePaymentBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveInvoicePayment {
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

impl StripeRequest for RetrieveInvoicePayment {
    type Output = stripe_shared::InvoicePayment;

    fn build(&self) -> RequestBuilder {
        let invoice_payment = &self.invoice_payment;
        RequestBuilder::new(StripeMethod::Get, format!("/invoice_payments/{invoice_payment}"))
            .query(&self.inner)
    }
}
