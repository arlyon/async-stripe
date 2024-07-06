use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListDisputeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListDisputeBuilder<'a> {
    fn new() -> Self {
        Self {
            charge: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_intent: None,
            starting_after: None,
        }
    }
}
/// Returns a list of your disputes.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListDispute<'a> {
    inner: ListDisputeBuilder<'a>,
}
impl<'a> ListDispute<'a> {
    /// Construct a new `ListDispute`.
    pub fn new() -> Self {
        Self { inner: ListDisputeBuilder::new() }
    }
    /// Only return disputes associated to the charge specified by this charge ID.
    pub fn charge(mut self, charge: &'a str) -> Self {
        self.inner.charge = Some(charge);
        self
    }
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
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
    /// Only return disputes associated to the PaymentIntent specified by this PaymentIntent ID.
    pub fn payment_intent(mut self, payment_intent: &'a str) -> Self {
        self.inner.payment_intent = Some(payment_intent);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListDispute<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListDispute<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Dispute>> {
        stripe_client_core::ListPaginator::new_list("/disputes", self.inner)
    }
}

impl StripeRequest for ListDispute<'_> {
    type Output = stripe_types::List<stripe_shared::Dispute>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/disputes").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveDisputeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveDisputeBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the dispute with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveDispute<'a> {
    inner: RetrieveDisputeBuilder<'a>,
    dispute: &'a stripe_shared::DisputeId,
}
impl<'a> RetrieveDispute<'a> {
    /// Construct a new `RetrieveDispute`.
    pub fn new(dispute: &'a stripe_shared::DisputeId) -> Self {
        Self { dispute, inner: RetrieveDisputeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveDispute<'_> {
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

impl StripeRequest for RetrieveDispute<'_> {
    type Output = stripe_shared::Dispute;

    fn build(&self) -> RequestBuilder {
        let dispute = self.dispute;
        RequestBuilder::new(StripeMethod::Get, format!("/disputes/{dispute}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateDisputeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence: Option<UpdateDisputeEvidence<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submit: Option<bool>,
}
impl<'a> UpdateDisputeBuilder<'a> {
    fn new() -> Self {
        Self { evidence: None, expand: None, metadata: None, submit: None }
    }
}
/// Evidence to upload, to respond to a dispute.
/// Updating any field in the hash will submit all fields in the hash for review.
/// The combined character count of all fields is limited to 150,000.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateDisputeEvidence<'a> {
    /// Any server or activity logs showing proof that the customer accessed or downloaded the purchased digital product.
    /// This information should include IP addresses, corresponding timestamps, and any detailed recorded activity.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_activity_log: Option<&'a str>,
    /// The billing address provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your subscription cancellation policy, as shown to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy: Option<&'a str>,
    /// An explanation of how and when the customer was shown your refund policy prior to purchase.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_disclosure: Option<&'a str>,
    /// A justification for why the customer's subscription was not canceled.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_rebuttal: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any communication with the customer that you feel is relevant to your case.
    /// Examples include emails proving that the customer received the product or service, or demonstrating their use of or satisfaction with the product or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_communication: Option<&'a str>,
    /// The email address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<&'a str>,
    /// The name of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<&'a str>,
    /// The IP address that the customer used when making the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_purchase_ip: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A relevant document or contract showing the customer's signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_signature: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation for the prior charge that can uniquely identify the charge, such as a receipt, shipping label, work order, etc.
    /// This document should be paired with a similar document from the disputed payment that proves the two payments are separate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_documentation: Option<&'a str>,
    /// An explanation of the difference between the disputed charge versus the prior charge that appears to be a duplicate.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_explanation: Option<&'a str>,
    /// The Stripe ID for the prior charge which appears to be a duplicate of the disputed charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_id: Option<&'a str>,
    /// A description of the product or service that was sold. Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any receipt or message sent to the customer notifying them of the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your refund policy, as shown to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy: Option<&'a str>,
    /// Documentation demonstrating that the customer was shown your refund policy prior to purchase.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_disclosure: Option<&'a str>,
    /// A justification for why the customer is not entitled to a refund.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_explanation: Option<&'a str>,
    /// The date on which the customer received or began receiving the purchased service, in a clear human-readable format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_date: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a service was provided to the customer.
    /// This could include a copy of a signed contract, work order, or other form of written agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_documentation: Option<&'a str>,
    /// The address to which a physical product was shipped.
    /// You should try to include as complete address information as possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<&'a str>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    /// If multiple carriers were used for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_carrier: Option<&'a str>,
    /// The date on which a physical product began its route to the shipping address, in a clear human-readable format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_date: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a product was shipped to the customer at the same address the customer provided to you.
    /// This could include a copy of the shipment receipt, shipping label, etc.
    /// It should show the customer's full shipping address, if possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_documentation: Option<&'a str>,
    /// The tracking number for a physical product, obtained from the delivery service.
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_tracking_number: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any additional evidence or statements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_file: Option<&'a str>,
    /// Any additional evidence or statements. Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_text: Option<&'a str>,
}
impl<'a> UpdateDisputeEvidence<'a> {
    pub fn new() -> Self {
        Self {
            access_activity_log: None,
            billing_address: None,
            cancellation_policy: None,
            cancellation_policy_disclosure: None,
            cancellation_rebuttal: None,
            customer_communication: None,
            customer_email_address: None,
            customer_name: None,
            customer_purchase_ip: None,
            customer_signature: None,
            duplicate_charge_documentation: None,
            duplicate_charge_explanation: None,
            duplicate_charge_id: None,
            product_description: None,
            receipt: None,
            refund_policy: None,
            refund_policy_disclosure: None,
            refund_refusal_explanation: None,
            service_date: None,
            service_documentation: None,
            shipping_address: None,
            shipping_carrier: None,
            shipping_date: None,
            shipping_documentation: None,
            shipping_tracking_number: None,
            uncategorized_file: None,
            uncategorized_text: None,
        }
    }
}
impl<'a> Default for UpdateDisputeEvidence<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// When you get a dispute, contacting your customer is always the best first step.
/// If that doesn’t work, you can submit evidence to help us resolve the dispute in your favor.
/// You can do this in your [dashboard](https://dashboard.stripe.com/disputes), but if you prefer, you can use the API to submit evidence programmatically.
///
/// Depending on your dispute type, different evidence fields will give you a better chance of winning your dispute.
/// To figure out which evidence fields to provide, see our [guide to dispute types](https://stripe.com/docs/disputes/categories).
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateDispute<'a> {
    inner: UpdateDisputeBuilder<'a>,
    dispute: &'a stripe_shared::DisputeId,
}
impl<'a> UpdateDispute<'a> {
    /// Construct a new `UpdateDispute`.
    pub fn new(dispute: &'a stripe_shared::DisputeId) -> Self {
        Self { dispute, inner: UpdateDisputeBuilder::new() }
    }
    /// Evidence to upload, to respond to a dispute.
    /// Updating any field in the hash will submit all fields in the hash for review.
    /// The combined character count of all fields is limited to 150,000.
    pub fn evidence(mut self, evidence: UpdateDisputeEvidence<'a>) -> Self {
        self.inner.evidence = Some(evidence);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// Whether to immediately submit evidence to the bank.
    /// If `false`, evidence is staged on the dispute.
    /// Staged evidence is visible in the API and Dashboard, and can be submitted to the bank by making another request with this attribute set to `true` (the default).
    pub fn submit(mut self, submit: bool) -> Self {
        self.inner.submit = Some(submit);
        self
    }
}
impl UpdateDispute<'_> {
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

impl StripeRequest for UpdateDispute<'_> {
    type Output = stripe_shared::Dispute;

    fn build(&self) -> RequestBuilder {
        let dispute = self.dispute;
        RequestBuilder::new(StripeMethod::Post, format!("/disputes/{dispute}")).form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CloseDisputeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> CloseDisputeBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Closing the dispute for a charge indicates that you do not have any evidence to submit and are essentially dismissing the dispute, acknowledging it as lost.
///
/// The status of the dispute will change from `needs_response` to `lost`.
/// _Closing a dispute is irreversible_.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CloseDispute<'a> {
    inner: CloseDisputeBuilder<'a>,
    dispute: &'a stripe_shared::DisputeId,
}
impl<'a> CloseDispute<'a> {
    /// Construct a new `CloseDispute`.
    pub fn new(dispute: &'a stripe_shared::DisputeId) -> Self {
        Self { dispute, inner: CloseDisputeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CloseDispute<'_> {
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

impl StripeRequest for CloseDispute<'_> {
    type Output = stripe_shared::Dispute;

    fn build(&self) -> RequestBuilder {
        let dispute = self.dispute;
        RequestBuilder::new(StripeMethod::Post, format!("/disputes/{dispute}/close"))
            .form(&self.inner)
    }
}
