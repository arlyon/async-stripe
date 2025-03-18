// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{ChargeId, DisputeId, PaymentIntentId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{BalanceTransaction, Charge, Currency, File, PaymentIntent};
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
pub struct DisputeEvidenceDetails {
    /// Date by which evidence must be submitted in order to successfully challenge dispute.
    ///
    /// Will be 0 if the customer's bank or credit card company doesn't allow a response for this particular dispute.
    pub due_by: Option<Timestamp>,

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
pub struct DisputePaymentMethodDetails {
    /// Card specific dispute details.
    pub card: Option<DisputePaymentMethodDetailsCard>,

    /// Payment method type.
    #[serde(rename = "type")]
    pub type_: DisputePaymentMethodDetailsType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputePaymentMethodDetailsCard {
    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: String,

    /// The card network's specific dispute reason code, which maps to one of Stripe's primary dispute categories to simplify response guidance.
    ///
    /// The [Network code map](https://stripe.com/docs/disputes/categories#network-code-map) lists all available dispute reason codes by network.
    pub network_reason_code: Option<String>,
}

/// The parameters for `Dispute::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListDisputes<'a> {
    /// Only return disputes associated to the charge specified by this charge ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<ChargeId>,

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
    }
}
/// An enum representing the possible values of an `DisputePaymentMethodDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DisputePaymentMethodDetailsType {
    Card,
}

impl DisputePaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            DisputePaymentMethodDetailsType::Card => "card",
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
        Self::Card
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
