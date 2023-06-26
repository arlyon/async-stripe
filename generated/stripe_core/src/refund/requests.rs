impl stripe_core::refund::Refund {
    /// Returns a list of all refunds you’ve previously created.
    ///
    /// The refunds are returned in sorted order, with the most recent refunds appearing first.
    /// For convenience, the 10 most recent refunds are always available by default on the charge object.
    pub fn list(
        client: &stripe::Client,
        params: ListRefund,
    ) -> stripe::Response<stripe_types::List<stripe_core::refund::Refund>> {
        client.get_query("/refunds", params)
    }
    /// Create a refund.
    pub fn create(
        client: &stripe::Client,
        params: CreateRefund,
    ) -> stripe::Response<stripe_core::refund::Refund> {
        client.send_form("/refunds", params, http_types::Method::Post)
    }
    /// Retrieves the details of an existing refund.
    pub fn retrieve(
        client: &stripe::Client,
        refund: &stripe_core::refund::RefundId,
        params: RetrieveRefund,
    ) -> stripe::Response<stripe_core::refund::Refund> {
        client.get_query(&format!("/refunds/{refund}", refund = refund), params)
    }
    /// Updates the specified refund by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.  This request only accepts `metadata` as an argument.
    pub fn update(
        client: &stripe::Client,
        refund: &stripe_core::refund::RefundId,
        params: UpdateRefund,
    ) -> stripe::Response<stripe_core::refund::Refund> {
        client.send_form(
            &format!("/refunds/{refund}", refund = refund),
            params,
            http_types::Method::Post,
        )
    }
    /// Cancels a refund with a status of `requires_action`.
    ///
    /// Refunds in other states cannot be canceled, and only refunds for payment methods that require customer action will enter the `requires_action` state.
    pub fn cancel(
        client: &stripe::Client,
        refund: &stripe_core::refund::RefundId,
        params: CancelRefund,
    ) -> stripe::Response<stripe_core::refund::Refund> {
        client.send_form(
            &format!("/refunds/{refund}/cancel", refund = refund),
            params,
            http_types::Method::Post,
        )
    }
    /// Expire a refund with a status of `requires_action`.
    pub fn expire(
        client: &stripe::Client,
        refund: &stripe_core::refund::RefundId,
        params: ExpireRefund,
    ) -> stripe::Response<stripe_core::refund::Refund> {
        client.send_form(
            &format!("/test_helpers/refunds/{refund}/expire", refund = refund),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListRefund<'a> {
    /// Only return refunds for the charge specified by this charge ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return refunds for the PaymentIntent specified by this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListRefund<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateRefund<'a> {
    /// A positive integer representing how much to refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<&'a str>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Customer whose customer balance to refund from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Address to send refund email, use customer email if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_email: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Origin of the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<CreateRefundOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<CreateRefundReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_transfer: Option<bool>,
}
impl<'a> CreateRefund<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Origin of the refund.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateRefundOrigin {
    CustomerBalance,
}

impl CreateRefundOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomerBalance => "customer_balance",
        }
    }
}

impl std::str::FromStr for CreateRefundOrigin {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "customer_balance" => Ok(Self::CustomerBalance),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateRefundOrigin {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateRefundOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateRefundOrigin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateRefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}

impl CreateRefundReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl std::str::FromStr for CreateRefundReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "duplicate" => Ok(Self::Duplicate),
            "fraudulent" => Ok(Self::Fraudulent),
            "requested_by_customer" => Ok(Self::RequestedByCustomer),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateRefundReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateRefundReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateRefundReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveRefund<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveRefund<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateRefund<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateRefund<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelRefund<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelRefund<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ExpireRefund<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ExpireRefund<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
