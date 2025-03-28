// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{ChargeId, DisputeId, PaymentIntentId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{BalanceTransaction, Charge, Currency, DisputeTransactionShippingAddress, File, PaymentIntent};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Dispute".
///
/// For more details see <https://stripe.com/docs/api/disputes/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Dispute {
    /// Unique identifier for the object.
    pub id: DisputeId,

    /// Disputed amount.
    ///
    /// Usually the amount of the charge, but it can differ (usually because of currency fluctuation or because only part of the order is disputed).
    pub amount: i64,

    /// List of zero, one, or two balance transactions that show funds withdrawn and reinstated to your Stripe account as a result of this dispute.
    pub balance_transactions: Vec<BalanceTransaction>,

    /// ID of the charge that's disputed.
    pub charge: Expandable<Charge>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// List of eligibility types that are included in `enhanced_evidence`.
    pub enhanced_eligibility_types: Vec<DisputeEnhancedEligibilityTypes>,

    pub evidence: DisputeEvidence,

    pub evidence_details: DisputeEvidenceDetails,

    /// If true, it's still possible to refund the disputed payment.
    ///
    /// After the payment has been fully refunded, no further funds are withdrawn from your Stripe account as a result of this dispute.
    pub is_charge_refundable: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Network-dependent reason code for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_reason_code: Option<String>,

    /// ID of the PaymentIntent that's disputed.
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<DisputePaymentMethodDetails>,

    /// Reason given by cardholder for dispute.
    ///
    /// Possible values are `bank_cannot_process`, `check_returned`, `credit_not_processed`, `customer_initiated`, `debit_not_authorized`, `duplicate`, `fraudulent`, `general`, `incorrect_account_details`, `insufficient_funds`, `product_not_received`, `product_unacceptable`, `subscription_canceled`, or `unrecognized`.
    /// Learn more about [dispute reasons](https://stripe.com/docs/disputes/categories).
    pub reason: String,

    /// Current status of dispute.
    ///
    /// Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `won`, or `lost`.
    pub status: DisputeStatus,
}

impl Dispute {

    /// Returns a list of your disputes.
pub fn list(client: &Client, params: &ListDisputes<'_>) -> Response<List<Dispute>> {
   client.get_query("/disputes", params)
}


    /// Retrieves the dispute with the given ID.
    pub fn retrieve(client: &Client, id: &DisputeId, expand: &[&str]) -> Response<Dispute> {
        client.get_query(&format!("/disputes/{}", id), Expand { expand })
    }
}

impl Object for Dispute {
    type Id = DisputeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "dispute"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputeEvidence {

    /// Any server or activity logs showing proof that the customer accessed or downloaded the purchased digital product.
    ///
    /// This information should include IP addresses, corresponding timestamps, and any detailed recorded activity.
    pub access_activity_log: Option<String>,

    /// The billing address provided by the customer.
    pub billing_address: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your subscription cancellation policy, as shown to the customer.
    pub cancellation_policy: Option<Expandable<File>>,

    /// An explanation of how and when the customer was shown your refund policy prior to purchase.
    pub cancellation_policy_disclosure: Option<String>,

    /// A justification for why the customer's subscription was not canceled.
    pub cancellation_rebuttal: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any communication with the customer that you feel is relevant to your case.
    ///
    /// Examples include emails proving that the customer received the product or service, or demonstrating their use of or satisfaction with the product or service.
    pub customer_communication: Option<Expandable<File>>,

    /// The email address of the customer.
    pub customer_email_address: Option<String>,

    /// The name of the customer.
    pub customer_name: Option<String>,

    /// The IP address that the customer used when making the purchase.
    pub customer_purchase_ip: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A relevant document or contract showing the customer's signature.
    pub customer_signature: Option<Expandable<File>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation for the prior charge that can uniquely identify the charge, such as a receipt, shipping label, work order, etc.
    ///
    /// This document should be paired with a similar document from the disputed payment that proves the two payments are separate.
    pub duplicate_charge_documentation: Option<Expandable<File>>,

    /// An explanation of the difference between the disputed charge versus the prior charge that appears to be a duplicate.
    pub duplicate_charge_explanation: Option<String>,

    /// The Stripe ID for the prior charge which appears to be a duplicate of the disputed charge.
    pub duplicate_charge_id: Option<String>,

    pub enhanced_evidence: DisputeEnhancedEvidence,

    /// A description of the product or service that was sold.
    pub product_description: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any receipt or message sent to the customer notifying them of the charge.
    pub receipt: Option<Expandable<File>>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your refund policy, as shown to the customer.
    pub refund_policy: Option<Expandable<File>>,

    /// Documentation demonstrating that the customer was shown your refund policy prior to purchase.
    pub refund_policy_disclosure: Option<String>,

    /// A justification for why the customer is not entitled to a refund.
    pub refund_refusal_explanation: Option<String>,

    /// The date on which the customer received or began receiving the purchased service, in a clear human-readable format.
    pub service_date: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a service was provided to the customer.
    ///
    /// This could include a copy of a signed contract, work order, or other form of written agreement.
    pub service_documentation: Option<Expandable<File>>,

    /// The address to which a physical product was shipped.
    ///
    /// You should try to include as complete address information as possible.
    pub shipping_address: Option<String>,

    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    ///
    /// If multiple carriers were used for this purchase, please separate them with commas.
    pub shipping_carrier: Option<String>,

    /// The date on which a physical product began its route to the shipping address, in a clear human-readable format.
    pub shipping_date: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a product was shipped to the customer at the same address the customer provided to you.
    ///
    /// This could include a copy of the shipment receipt, shipping label, etc.
    /// It should show the customer's full shipping address, if possible.
    pub shipping_documentation: Option<Expandable<File>>,

    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    pub shipping_tracking_number: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any additional evidence or statements.
    pub uncategorized_file: Option<Expandable<File>>,

    /// Any additional evidence or statements.
    pub uncategorized_text: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputeEnhancedEvidence {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_compelling_evidence_3: Option<DisputeEnhancedEvidenceVisaCompellingEvidence3>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_compliance: Option<DisputeEnhancedEvidenceVisaCompliance>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputeEnhancedEvidenceVisaCompellingEvidence3 {

    /// Disputed transaction details for Visa Compelling Evidence 3.0 evidence submission.
    pub disputed_transaction: Option<DisputeVisaCompellingEvidence3DisputedTransaction>,

    /// List of exactly two prior undisputed transaction objects for Visa Compelling Evidence 3.0 evidence submission.
    pub prior_undisputed_transactions: Vec<DisputeVisaCompellingEvidence3PriorUndisputedTransaction>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputeEnhancedEvidenceVisaCompliance {

    /// A field acknowledging the fee incurred when countering a Visa compliance dispute.
    ///
    /// If this field is set to true, evidence can be submitted for the compliance dispute.
    /// Stripe collects a 500 USD (or local equivalent) amount to cover the network costs associated with resolving compliance disputes.
    /// Stripe refunds the 500 USD network fee if you win the dispute.
    pub fee_acknowledged: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputeEvidenceDetails {

    /// Date by which evidence must be submitted in order to successfully challenge dispute.
    ///
    /// Will be 0 if the customer's bank or credit card company doesn't allow a response for this particular dispute.
    pub due_by: Option<Timestamp>,

    pub enhanced_eligibility: DisputeEnhancedEligibility,

    /// Whether evidence has been staged for this dispute.
    pub has_evidence: bool,

    /// Whether the last evidence submission was submitted past the due date.
    ///
    /// Defaults to `false` if no evidence submissions have occurred.
    /// If `true`, then delivery of the latest evidence is *not* guaranteed.
    pub past_due: bool,

    /// The number of times evidence has been submitted.
    ///
    /// Typically, you may only submit evidence once.
    pub submission_count: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputeEnhancedEligibility {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_compelling_evidence_3: Option<DisputeEnhancedEligibilityVisaCompellingEvidence3>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_compliance: Option<DisputeEnhancedEligibilityVisaCompliance>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputeEnhancedEligibilityVisaCompellingEvidence3 {

    /// List of actions required to qualify dispute for Visa Compelling Evidence 3.0 evidence submission.
    pub required_actions: Vec<DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions>,

    /// Visa Compelling Evidence 3.0 eligibility status.
    pub status: DisputeEnhancedEligibilityVisaCompellingEvidence3Status,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputeEnhancedEligibilityVisaCompliance {

    /// Visa compliance eligibility status.
    pub status: DisputeEnhancedEligibilityVisaComplianceStatus,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputePaymentMethodDetails {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<DisputePaymentMethodDetailsAmazonPay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<DisputePaymentMethodDetailsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<DisputePaymentMethodDetailsKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<DisputePaymentMethodDetailsPaypal>,

    /// Payment method type.
    #[serde(rename = "type")]
    pub type_: DisputePaymentMethodDetailsType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputePaymentMethodDetailsAmazonPay {

    /// The AmazonPay dispute type, chargeback or claim.
    pub dispute_type: Option<DisputePaymentMethodDetailsAmazonPayDisputeType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputePaymentMethodDetailsCard {

    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `link`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: String,

    /// The type of dispute opened.
    ///
    /// Different case types may have varying fees and financial impact.
    pub case_type: DisputePaymentMethodDetailsCardCaseType,

    /// The card network's specific dispute reason code, which maps to one of Stripe's primary dispute categories to simplify response guidance.
    ///
    /// The [Network code map](https://stripe.com/docs/disputes/categories#network-code-map) lists all available dispute reason codes by network.
    pub network_reason_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputePaymentMethodDetailsKlarna {

    /// The reason for the dispute as defined by Klarna.
    pub reason_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputePaymentMethodDetailsPaypal {

    /// The ID of the dispute in PayPal.
    pub case_id: Option<String>,

    /// The reason for the dispute as defined by PayPal.
    pub reason_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputeVisaCompellingEvidence3DisputedTransaction {

    /// User Account ID used to log into business platform.
    ///
    /// Must be recognizable by the user.
    pub customer_account_id: Option<String>,

    /// Unique identifier of the cardholder’s device derived from a combination of at least two hardware and software attributes.
    ///
    /// Must be at least 20 characters.
    pub customer_device_fingerprint: Option<String>,

    /// Unique identifier of the cardholder’s device such as a device serial number (e.g., International Mobile Equipment Identity [IMEI]).
    ///
    /// Must be at least 15 characters.
    pub customer_device_id: Option<String>,

    /// The email address of the customer.
    pub customer_email_address: Option<String>,

    /// The IP address that the customer used when making the purchase.
    pub customer_purchase_ip: Option<String>,

    /// Categorization of disputed payment.
    pub merchandise_or_services: Option<DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices>,

    /// A description of the product or service that was sold.
    pub product_description: Option<String>,

    /// The address to which a physical product was shipped.
    ///
    /// All fields are required for Visa Compelling Evidence 3.0 evidence submission.
    pub shipping_address: Option<DisputeTransactionShippingAddress>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputeVisaCompellingEvidence3PriorUndisputedTransaction {

    /// Stripe charge ID for the Visa Compelling Evidence 3.0 eligible prior charge.
    pub charge: String,

    /// User Account ID used to log into business platform.
    ///
    /// Must be recognizable by the user.
    pub customer_account_id: Option<String>,

    /// Unique identifier of the cardholder’s device derived from a combination of at least two hardware and software attributes.
    ///
    /// Must be at least 20 characters.
    pub customer_device_fingerprint: Option<String>,

    /// Unique identifier of the cardholder’s device such as a device serial number (e.g., International Mobile Equipment Identity [IMEI]).
    ///
    /// Must be at least 15 characters.
    pub customer_device_id: Option<String>,

    /// The email address of the customer.
    pub customer_email_address: Option<String>,

    /// The IP address that the customer used when making the purchase.
    pub customer_purchase_ip: Option<String>,

    /// A description of the product or service that was sold.
    pub product_description: Option<String>,

    /// The address to which a physical product was shipped.
    ///
    /// All fields are required for Visa Compelling Evidence 3.0 evidence submission.
    pub shipping_address: Option<DisputeTransactionShippingAddress>,
}

/// The parameters for `Dispute::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListDisputes<'a> {

    /// Only return disputes associated to the charge specified by this charge ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<ChargeId>,

    /// Only return disputes that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<DisputeId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return disputes associated to the PaymentIntent specified by this PaymentIntent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<PaymentIntentId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<DisputeId>,
}

impl<'a> ListDisputes<'a> {
    pub fn new() -> Self {
        ListDisputes {
            charge: Default::default(),
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            payment_intent: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListDisputes<'_> {
    type O = Dispute;
    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }}
/// An enum representing the possible values of an `Dispute`'s `enhanced_eligibility_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DisputeEnhancedEligibilityTypes {
    #[serde(rename = "visa_compelling_evidence_3")]
    VisaCompellingEvidence3,
}

impl DisputeEnhancedEligibilityTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            DisputeEnhancedEligibilityTypes::VisaCompellingEvidence3 => "visa_compelling_evidence_3",
        }
    }
}

impl AsRef<str> for DisputeEnhancedEligibilityTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputeEnhancedEligibilityTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for DisputeEnhancedEligibilityTypes {
    fn default() -> Self {
        Self::VisaCompellingEvidence3
    }
}

/// An enum representing the possible values of an `DisputeEnhancedEligibilityVisaCompellingEvidence3`'s `required_actions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    MissingCustomerIdentifiers,
    MissingDisputedTransactionDescription,
    MissingMerchandiseOrServices,
    MissingPriorUndisputedTransactionDescription,
    MissingPriorUndisputedTransactions,
}

impl DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    pub fn as_str(self) -> &'static str {
        match self {
            DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions::MissingCustomerIdentifiers => "missing_customer_identifiers",
            DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions::MissingDisputedTransactionDescription => "missing_disputed_transaction_description",
            DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions::MissingMerchandiseOrServices => "missing_merchandise_or_services",
            DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions::MissingPriorUndisputedTransactionDescription => "missing_prior_undisputed_transaction_description",
            DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions::MissingPriorUndisputedTransactions => "missing_prior_undisputed_transactions",
        }
    }
}

impl AsRef<str> for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for DisputeEnhancedEligibilityVisaCompellingEvidence3RequiredActions {
    fn default() -> Self {
        Self::MissingCustomerIdentifiers
    }
}

/// An enum representing the possible values of an `DisputeEnhancedEligibilityVisaCompellingEvidence3`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    NotQualified,
    Qualified,
    RequiresAction,
}

impl DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    pub fn as_str(self) -> &'static str {
        match self {
            DisputeEnhancedEligibilityVisaCompellingEvidence3Status::NotQualified => "not_qualified",
            DisputeEnhancedEligibilityVisaCompellingEvidence3Status::Qualified => "qualified",
            DisputeEnhancedEligibilityVisaCompellingEvidence3Status::RequiresAction => "requires_action",
        }
    }
}

impl AsRef<str> for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for DisputeEnhancedEligibilityVisaCompellingEvidence3Status {
    fn default() -> Self {
        Self::NotQualified
    }
}

/// An enum representing the possible values of an `DisputeEnhancedEligibilityVisaCompliance`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DisputeEnhancedEligibilityVisaComplianceStatus {
    FeeAcknowledged,
    RequiresFeeAcknowledgement,
}

impl DisputeEnhancedEligibilityVisaComplianceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            DisputeEnhancedEligibilityVisaComplianceStatus::FeeAcknowledged => "fee_acknowledged",
            DisputeEnhancedEligibilityVisaComplianceStatus::RequiresFeeAcknowledgement => "requires_fee_acknowledgement",
        }
    }
}

impl AsRef<str> for DisputeEnhancedEligibilityVisaComplianceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputeEnhancedEligibilityVisaComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for DisputeEnhancedEligibilityVisaComplianceStatus {
    fn default() -> Self {
        Self::FeeAcknowledged
    }
}

/// An enum representing the possible values of an `DisputePaymentMethodDetailsAmazonPay`'s `dispute_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DisputePaymentMethodDetailsAmazonPayDisputeType {
    Chargeback,
    Claim,
}

impl DisputePaymentMethodDetailsAmazonPayDisputeType {
    pub fn as_str(self) -> &'static str {
        match self {
            DisputePaymentMethodDetailsAmazonPayDisputeType::Chargeback => "chargeback",
            DisputePaymentMethodDetailsAmazonPayDisputeType::Claim => "claim",
        }
    }
}

impl AsRef<str> for DisputePaymentMethodDetailsAmazonPayDisputeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputePaymentMethodDetailsAmazonPayDisputeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for DisputePaymentMethodDetailsAmazonPayDisputeType {
    fn default() -> Self {
        Self::Chargeback
    }
}

/// An enum representing the possible values of an `DisputePaymentMethodDetailsCard`'s `case_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DisputePaymentMethodDetailsCardCaseType {
    Chargeback,
    Inquiry,
}

impl DisputePaymentMethodDetailsCardCaseType {
    pub fn as_str(self) -> &'static str {
        match self {
            DisputePaymentMethodDetailsCardCaseType::Chargeback => "chargeback",
            DisputePaymentMethodDetailsCardCaseType::Inquiry => "inquiry",
        }
    }
}

impl AsRef<str> for DisputePaymentMethodDetailsCardCaseType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputePaymentMethodDetailsCardCaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for DisputePaymentMethodDetailsCardCaseType {
    fn default() -> Self {
        Self::Chargeback
    }
}

/// An enum representing the possible values of an `DisputePaymentMethodDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DisputePaymentMethodDetailsType {
    AmazonPay,
    Card,
    Klarna,
    Paypal,
}

impl DisputePaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            DisputePaymentMethodDetailsType::AmazonPay => "amazon_pay",
            DisputePaymentMethodDetailsType::Card => "card",
            DisputePaymentMethodDetailsType::Klarna => "klarna",
            DisputePaymentMethodDetailsType::Paypal => "paypal",
        }
    }
}

impl AsRef<str> for DisputePaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputePaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for DisputePaymentMethodDetailsType {
    fn default() -> Self {
        Self::AmazonPay
    }
}

/// An enum representing the possible values of an `Dispute`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DisputeStatus {
    Lost,
    NeedsResponse,
    UnderReview,
    WarningClosed,
    WarningNeedsResponse,
    WarningUnderReview,
    Won,
}

impl DisputeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            DisputeStatus::Lost => "lost",
            DisputeStatus::NeedsResponse => "needs_response",
            DisputeStatus::UnderReview => "under_review",
            DisputeStatus::WarningClosed => "warning_closed",
            DisputeStatus::WarningNeedsResponse => "warning_needs_response",
            DisputeStatus::WarningUnderReview => "warning_under_review",
            DisputeStatus::Won => "won",
        }
    }
}

impl AsRef<str> for DisputeStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for DisputeStatus {
    fn default() -> Self {
        Self::Lost
    }
}

/// An enum representing the possible values of an `DisputeVisaCompellingEvidence3DisputedTransaction`'s `merchandise_or_services` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    Merchandise,
    Services,
}

impl DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    pub fn as_str(self) -> &'static str {
        match self {
            DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::Merchandise => "merchandise",
            DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::Services => "services",
        }
    }
}

impl AsRef<str> for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for DisputeVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    fn default() -> Self {
        Self::Merchandise
    }
}
