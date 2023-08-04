#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListDispute<'a> {
    /// Only return disputes associated to the charge specified by this charge ID.
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
    /// Only return disputes associated to the PaymentIntent specified by this PaymentIntent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListDispute<'a> {
    /// Returns a list of your disputes.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::Dispute>> {
        client.get_query("/disputes", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveDispute<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveDispute<'a> {
    /// Retrieves the dispute with the given ID.
    pub fn send(
        &self,
        client: &stripe::Client,
        dispute: &stripe_types::dispute::DisputeId,
    ) -> stripe::Response<stripe_types::Dispute> {
        client.get_query(&format!("/disputes/{dispute}", dispute = dispute), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateDispute<'a> {
    /// Evidence to upload, to respond to a dispute.
    ///
    /// Updating any field in the hash will submit all fields in the hash for review.
    /// The combined character count of all fields is limited to 150,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<UpdateDisputeEvidence<'a>>,
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
    /// Whether to immediately submit evidence to the bank.
    ///
    /// If `false`, evidence is staged on the dispute.
    /// Staged evidence is visible in the API and Dashboard, and can be submitted to the bank by making another request with this attribute set to `true` (the default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<bool>,
}
impl<'a> UpdateDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Evidence to upload, to respond to a dispute.
///
/// Updating any field in the hash will submit all fields in the hash for review.
/// The combined character count of all fields is limited to 150,000.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateDisputeEvidence<'a> {
    /// Any server or activity logs showing proof that the customer accessed or downloaded the purchased digital product.
    ///
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
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_disclosure: Option<&'a str>,
    /// A justification for why the customer's subscription was not canceled.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_rebuttal: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any communication with the customer that you feel is relevant to your case.
    ///
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
    ///
    /// This document should be paired with a similar document from the disputed payment that proves the two payments are separate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_documentation: Option<&'a str>,
    /// An explanation of the difference between the disputed charge versus the prior charge that appears to be a duplicate.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_explanation: Option<&'a str>,
    /// The Stripe ID for the prior charge which appears to be a duplicate of the disputed charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_id: Option<&'a str>,
    /// A description of the product or service that was sold.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any receipt or message sent to the customer notifying them of the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your refund policy, as shown to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy: Option<&'a str>,
    /// Documentation demonstrating that the customer was shown your refund policy prior to purchase.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_disclosure: Option<&'a str>,
    /// A justification for why the customer is not entitled to a refund.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_explanation: Option<&'a str>,
    /// The date on which the customer received or began receiving the purchased service, in a clear human-readable format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_date: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a service was provided to the customer.
    ///
    /// This could include a copy of a signed contract, work order, or other form of written agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_documentation: Option<&'a str>,
    /// The address to which a physical product was shipped.
    ///
    /// You should try to include as complete address information as possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<&'a str>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    ///
    /// If multiple carriers were used for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_carrier: Option<&'a str>,
    /// The date on which a physical product began its route to the shipping address, in a clear human-readable format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_date: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a product was shipped to the customer at the same address the customer provided to you.
    ///
    /// This could include a copy of the shipment receipt, shipping label, etc.
    /// It should show the customer's full shipping address, if possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_documentation: Option<&'a str>,
    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_tracking_number: Option<&'a str>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any additional evidence or statements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_file: Option<&'a str>,
    /// Any additional evidence or statements.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_text: Option<&'a str>,
}
impl<'a> UpdateDisputeEvidence<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateDispute<'a> {
    /// When you get a dispute, contacting your customer is always the best first step.
    ///
    /// If that doesn’t work, you can submit evidence to help us resolve the dispute in your favor.
    /// You can do this in your [dashboard](https://dashboard.stripe.com/disputes), but if you prefer, you can use the API to submit evidence programmatically.  Depending on your dispute type, different evidence fields will give you a better chance of winning your dispute.
    /// To figure out which evidence fields to provide, see our [guide to dispute types](https://stripe.com/docs/disputes/categories).
    pub fn send(
        &self,
        client: &stripe::Client,
        dispute: &stripe_types::dispute::DisputeId,
    ) -> stripe::Response<stripe_types::Dispute> {
        client.send_form(
            &format!("/disputes/{dispute}", dispute = dispute),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CloseDispute<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CloseDispute<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CloseDispute<'a> {
    /// Closing the dispute for a charge indicates that you do not have any evidence to submit and are essentially dismissing the dispute, acknowledging it as lost.
    ///
    /// The status of the dispute will change from `needs_response` to `lost`.
    ///
    /// _Closing a dispute is irreversible_.
    pub fn send(
        &self,
        client: &stripe::Client,
        dispute: &stripe_types::dispute::DisputeId,
    ) -> stripe::Response<stripe_types::Dispute> {
        client.send_form(
            &format!("/disputes/{dispute}/close", dispute = dispute),
            self,
            http_types::Method::Post,
        )
    }
}
