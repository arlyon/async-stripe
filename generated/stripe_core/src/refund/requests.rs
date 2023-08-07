#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
    pub ending_before: Option<&'a str>,
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
    pub starting_after: Option<&'a str>,
}
impl<'a> ListRefund<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> stripe::PaginationParams for ListRefund<'a> {}
impl<'a> ListRefund<'a> {
    /// Returns a list of all refunds you’ve previously created.
    ///
    /// The refunds are returned in sorted order, with the most recent refunds appearing first.
    /// For convenience, the 10 most recent refunds are always available by default on the charge object.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::Refund>> {
        client.get_query("/refunds", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::Refund> {
        stripe::ListPaginator::from_params("/refunds", self)
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
    /// For payment methods without native refund support (e.g., Konbini, PromptPay), use this email from the customer to receive refund instructions.
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateRefundOrigin {
    CustomerBalance,
}

impl CreateRefundOrigin {
    pub fn as_str(self) -> &'static str {
        use CreateRefundOrigin::*;
        match self {
            CustomerBalance => "customer_balance",
        }
    }
}

impl std::str::FromStr for CreateRefundOrigin {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateRefundOrigin::*;
        match s {
            "customer_balance" => Ok(CustomerBalance),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateRefundOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateRefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}

impl CreateRefundReason {
    pub fn as_str(self) -> &'static str {
        use CreateRefundReason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl std::str::FromStr for CreateRefundReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateRefundReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "requested_by_customer" => Ok(RequestedByCustomer),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateRefundReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl<'a> CreateRefund<'a> {
    /// Create a refund.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::Refund> {
        client.send_form("/refunds", self, http_types::Method::Post)
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
impl<'a> RetrieveRefund<'a> {
    /// Retrieves the details of an existing refund.
    pub fn send(
        &self,
        client: &stripe::Client,
        refund: &stripe_types::refund::RefundId,
    ) -> stripe::Response<stripe_types::Refund> {
        client.get_query(&format!("/refunds/{refund}", refund = refund), self)
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
impl<'a> UpdateRefund<'a> {
    /// Updates the specified refund by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.  This request only accepts `metadata` as an argument.
    pub fn send(
        &self,
        client: &stripe::Client,
        refund: &stripe_types::refund::RefundId,
    ) -> stripe::Response<stripe_types::Refund> {
        client.send_form(
            &format!("/refunds/{refund}", refund = refund),
            self,
            http_types::Method::Post,
        )
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
impl<'a> CancelRefund<'a> {
    /// Cancels a refund with a status of `requires_action`.
    ///
    /// Refunds in other states cannot be canceled, and only refunds for payment methods that require customer action will enter the `requires_action` state.
    pub fn send(
        &self,
        client: &stripe::Client,
        refund: &stripe_types::refund::RefundId,
    ) -> stripe::Response<stripe_types::Refund> {
        client.send_form(
            &format!("/refunds/{refund}/cancel", refund = refund),
            self,
            http_types::Method::Post,
        )
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
impl<'a> ExpireRefund<'a> {
    /// Expire a refund with a status of `requires_action`.
    pub fn send(
        &self,
        client: &stripe::Client,
        refund: &stripe_types::refund::RefundId,
    ) -> stripe::Response<stripe_types::Refund> {
        client.send_form(
            &format!("/test_helpers/refunds/{refund}/expire", refund = refund),
            self,
            http_types::Method::Post,
        )
    }
}
