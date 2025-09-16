use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListDisputeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListDisputeBuilder {
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
pub struct ListDispute {
    inner: ListDisputeBuilder,
}
impl ListDispute {
    /// Construct a new `ListDispute`.
    pub fn new() -> Self {
        Self { inner: ListDisputeBuilder::new() }
    }
    /// Only return disputes associated to the charge specified by this charge ID.
    pub fn charge(mut self, charge: impl Into<String>) -> Self {
        self.inner.charge = Some(charge.into());
        self
    }
    /// Only return disputes that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
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
    /// Only return disputes associated to the PaymentIntent specified by this PaymentIntent ID.
    pub fn payment_intent(mut self, payment_intent: impl Into<String>) -> Self {
        self.inner.payment_intent = Some(payment_intent.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListDispute {
    fn default() -> Self {
        Self::new()
    }
}
impl ListDispute {
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
        stripe_client_core::ListPaginator::new_list("/disputes", &self.inner)
    }
}

impl StripeRequest for ListDispute {
    type Output = stripe_types::List<stripe_shared::Dispute>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/disputes").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveDisputeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveDisputeBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the dispute with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveDispute {
    inner: RetrieveDisputeBuilder,
    dispute: stripe_shared::DisputeId,
}
impl RetrieveDispute {
    /// Construct a new `RetrieveDispute`.
    pub fn new(dispute: impl Into<stripe_shared::DisputeId>) -> Self {
        Self { dispute: dispute.into(), inner: RetrieveDisputeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveDispute {
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

impl StripeRequest for RetrieveDispute {
    type Output = stripe_shared::Dispute;

    fn build(&self) -> RequestBuilder {
        let dispute = &self.dispute;
        RequestBuilder::new(StripeMethod::Get, format!("/disputes/{dispute}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateDisputeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence: Option<UpdateDisputeEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submit: Option<bool>,
}
impl UpdateDisputeBuilder {
    fn new() -> Self {
        Self { evidence: None, expand: None, metadata: None, submit: None }
    }
}
/// Evidence to upload, to respond to a dispute.
/// Updating any field in the hash will submit all fields in the hash for review.
/// The combined character count of all fields is limited to 150,000.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateDisputeEvidence {
    /// Any server or activity logs showing proof that the customer accessed or downloaded the purchased digital product.
    /// This information should include IP addresses, corresponding timestamps, and any detailed recorded activity.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_activity_log: Option<String>,
    /// The billing address provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your subscription cancellation policy, as shown to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy: Option<String>,
    /// An explanation of how and when the customer was shown your refund policy prior to purchase.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_disclosure: Option<String>,
    /// A justification for why the customer's subscription was not canceled.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_rebuttal: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any communication with the customer that you feel is relevant to your case.
    /// Examples include emails proving that the customer received the product or service, or demonstrating their use of or satisfaction with the product or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_communication: Option<String>,
    /// The email address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,
    /// The name of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// The IP address that the customer used when making the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_purchase_ip: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A relevant document or contract showing the customer's signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_signature: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation for the prior charge that can uniquely identify the charge, such as a receipt, shipping label, work order, etc.
    /// This document should be paired with a similar document from the disputed payment that proves the two payments are separate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_documentation: Option<String>,
    /// An explanation of the difference between the disputed charge versus the prior charge that appears to be a duplicate.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_explanation: Option<String>,
    /// The Stripe ID for the prior charge which appears to be a duplicate of the disputed charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_id: Option<String>,
    /// Additional evidence for qualifying evidence programs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_evidence: Option<UpdateDisputeEvidenceEnhancedEvidence>,
    /// A description of the product or service that was sold. Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any receipt or message sent to the customer notifying them of the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your refund policy, as shown to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy: Option<String>,
    /// Documentation demonstrating that the customer was shown your refund policy prior to purchase.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_disclosure: Option<String>,
    /// A justification for why the customer is not entitled to a refund.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_explanation: Option<String>,
    /// The date on which the customer received or began receiving the purchased service, in a clear human-readable format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_date: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a service was provided to the customer.
    /// This could include a copy of a signed contract, work order, or other form of written agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_documentation: Option<String>,
    /// The address to which a physical product was shipped.
    /// You should try to include as complete address information as possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<String>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    /// If multiple carriers were used for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_carrier: Option<String>,
    /// The date on which a physical product began its route to the shipping address, in a clear human-readable format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_date: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a product was shipped to the customer at the same address the customer provided to you.
    /// This could include a copy of the shipment receipt, shipping label, etc.
    /// It should show the customer's full shipping address, if possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_documentation: Option<String>,
    /// The tracking number for a physical product, obtained from the delivery service.
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_tracking_number: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any additional evidence or statements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_file: Option<String>,
    /// Any additional evidence or statements. Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_text: Option<String>,
}
impl UpdateDisputeEvidence {
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
            enhanced_evidence: None,
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
impl Default for UpdateDisputeEvidence {
    fn default() -> Self {
        Self::new()
    }
}
/// Additional evidence for qualifying evidence programs.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateDisputeEvidenceEnhancedEvidence {
    /// Evidence provided for Visa Compelling Evidence 3.0 evidence submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_compelling_evidence_3:
        Option<UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3>,
    /// Evidence provided for Visa compliance evidence submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_compliance: Option<UpdateDisputeEvidenceEnhancedEvidenceVisaCompliance>,
}
impl UpdateDisputeEvidenceEnhancedEvidence {
    pub fn new() -> Self {
        Self { visa_compelling_evidence_3: None, visa_compliance: None }
    }
}
impl Default for UpdateDisputeEvidenceEnhancedEvidence {
    fn default() -> Self {
        Self::new()
    }
}
/// Evidence provided for Visa Compelling Evidence 3.0 evidence submission.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3 {
    /// Disputed transaction details for Visa Compelling Evidence 3.0 evidence submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disputed_transaction:
        Option<UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransaction>,
    /// List of exactly two prior undisputed transaction objects for Visa Compelling Evidence 3.0 evidence submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_undisputed_transactions: Option<
        Vec<
            UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3PriorUndisputedTransactions,
        >,
    >,
}
impl UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3 {
    pub fn new() -> Self {
        Self { disputed_transaction: None, prior_undisputed_transactions: None }
    }
}
impl Default for UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3 {
    fn default() -> Self {
        Self::new()
    }
}
/// Disputed transaction details for Visa Compelling Evidence 3.0 evidence submission.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransaction {
    /// User Account ID used to log into business platform. Must be recognizable by the user.
#[serde(skip_serializing_if = "Option::is_none")]
pub customer_account_id: Option<String>,
        /// Unique identifier of the cardholder’s device derived from a combination of at least two hardware and software attributes.
    /// Must be at least 20 characters.
#[serde(skip_serializing_if = "Option::is_none")]
pub customer_device_fingerprint: Option<String>,
        /// Unique identifier of the cardholder’s device such as a device serial number (e.g., International Mobile Equipment Identity [IMEI]).
    /// Must be at least 15 characters.
#[serde(skip_serializing_if = "Option::is_none")]
pub customer_device_id: Option<String>,
    /// The email address of the customer.
#[serde(skip_serializing_if = "Option::is_none")]
pub customer_email_address: Option<String>,
    /// The IP address that the customer used when making the purchase.
#[serde(skip_serializing_if = "Option::is_none")]
pub customer_purchase_ip: Option<String>,
    /// Categorization of disputed payment.
#[serde(skip_serializing_if = "Option::is_none")]
pub merchandise_or_services: Option<UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices>,
    /// A description of the product or service that was sold.
#[serde(skip_serializing_if = "Option::is_none")]
pub product_description: Option<String>,
        /// The address to which a physical product was shipped.
    /// All fields are required for Visa Compelling Evidence 3.0 evidence submission.
#[serde(skip_serializing_if = "Option::is_none")]
pub shipping_address: Option<ShippingAddress>,

}
impl UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransaction {
    pub fn new() -> Self {
        Self {
            customer_account_id: None,
            customer_device_fingerprint: None,
            customer_device_id: None,
            customer_email_address: None,
            customer_purchase_ip: None,
            merchandise_or_services: None,
            product_description: None,
            shipping_address: None,
        }
    }
}
impl Default for UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransaction {
    fn default() -> Self {
        Self::new()
    }
}
/// Categorization of disputed payment.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices
{
    Merchandise,
    Services,
}
impl UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    pub fn as_str(self) -> &'static str {
        use UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::*;
        match self {
Merchandise => "merchandise",
Services => "services",

        }
    }
}

impl std::str::FromStr for UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::*;
        match s {
    "merchandise" => Ok(Merchandise),
"services" => Ok(Services),
_ => Err(stripe_types::StripeParseError)

        }
    }
}
impl std::fmt::Display for UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices"))
    }
}
/// List of exactly two prior undisputed transaction objects for Visa Compelling Evidence 3.0 evidence submission.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3PriorUndisputedTransactions {
    /// Stripe charge ID for the Visa Compelling Evidence 3.0 eligible prior charge.
    pub charge: String,
    /// User Account ID used to log into business platform. Must be recognizable by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_account_id: Option<String>,
    /// Unique identifier of the cardholder’s device derived from a combination of at least two hardware and software attributes.
    /// Must be at least 20 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_device_fingerprint: Option<String>,
    /// Unique identifier of the cardholder’s device such as a device serial number (e.g., International Mobile Equipment Identity [IMEI]).
    /// Must be at least 15 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_device_id: Option<String>,
    /// The email address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,
    /// The IP address that the customer used when making the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_purchase_ip: Option<String>,
    /// A description of the product or service that was sold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// The address to which a physical product was shipped.
    /// All fields are required for Visa Compelling Evidence 3.0 evidence submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}
impl UpdateDisputeEvidenceEnhancedEvidenceVisaCompellingEvidence3PriorUndisputedTransactions {
    pub fn new(charge: impl Into<String>) -> Self {
        Self {
            charge: charge.into(),
            customer_account_id: None,
            customer_device_fingerprint: None,
            customer_device_id: None,
            customer_email_address: None,
            customer_purchase_ip: None,
            product_description: None,
            shipping_address: None,
        }
    }
}
/// Evidence provided for Visa compliance evidence submission.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateDisputeEvidenceEnhancedEvidenceVisaCompliance {
    /// A field acknowledging the fee incurred when countering a Visa compliance dispute.
    /// If this field is set to true, evidence can be submitted for the compliance dispute.
    /// Stripe collects a 500 USD (or local equivalent) amount to cover the network costs associated with resolving compliance disputes.
    /// Stripe refunds the 500 USD network fee if you win the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_acknowledged: Option<bool>,
}
impl UpdateDisputeEvidenceEnhancedEvidenceVisaCompliance {
    pub fn new() -> Self {
        Self { fee_acknowledged: None }
    }
}
impl Default for UpdateDisputeEvidenceEnhancedEvidenceVisaCompliance {
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
pub struct UpdateDispute {
    inner: UpdateDisputeBuilder,
    dispute: stripe_shared::DisputeId,
}
impl UpdateDispute {
    /// Construct a new `UpdateDispute`.
    pub fn new(dispute: impl Into<stripe_shared::DisputeId>) -> Self {
        Self { dispute: dispute.into(), inner: UpdateDisputeBuilder::new() }
    }
    /// Evidence to upload, to respond to a dispute.
    /// Updating any field in the hash will submit all fields in the hash for review.
    /// The combined character count of all fields is limited to 150,000.
    pub fn evidence(mut self, evidence: impl Into<UpdateDisputeEvidence>) -> Self {
        self.inner.evidence = Some(evidence.into());
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
    /// Whether to immediately submit evidence to the bank.
    /// If `false`, evidence is staged on the dispute.
    /// Staged evidence is visible in the API and Dashboard, and can be submitted to the bank by making another request with this attribute set to `true` (the default).
    pub fn submit(mut self, submit: impl Into<bool>) -> Self {
        self.inner.submit = Some(submit.into());
        self
    }
}
impl UpdateDispute {
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

impl StripeRequest for UpdateDispute {
    type Output = stripe_shared::Dispute;

    fn build(&self) -> RequestBuilder {
        let dispute = &self.dispute;
        RequestBuilder::new(StripeMethod::Post, format!("/disputes/{dispute}")).form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CloseDisputeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CloseDisputeBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Closing the dispute for a charge indicates that you do not have any evidence to submit and are essentially dismissing the dispute, acknowledging it as lost.
///
/// The status of the dispute will change from `needs_response` to `lost`.
/// _Closing a dispute is irreversible_.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CloseDispute {
    inner: CloseDisputeBuilder,
    dispute: stripe_shared::DisputeId,
}
impl CloseDispute {
    /// Construct a new `CloseDispute`.
    pub fn new(dispute: impl Into<stripe_shared::DisputeId>) -> Self {
        Self { dispute: dispute.into(), inner: CloseDisputeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CloseDispute {
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

impl StripeRequest for CloseDispute {
    type Output = stripe_shared::Dispute;

    fn build(&self) -> RequestBuilder {
        let dispute = &self.dispute;
        RequestBuilder::new(StripeMethod::Post, format!("/disputes/{dispute}/close"))
            .form(&self.inner)
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct ShippingAddress {
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
impl ShippingAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for ShippingAddress {
    fn default() -> Self {
        Self::new()
    }
}
