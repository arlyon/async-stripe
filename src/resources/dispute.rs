use crate::params::{Identifiable, Metadata, Timestamp};
use crate::resources::{BalanceTransaction, Currency};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EvidenceDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_by: Option<Timestamp>,
    pub has_evidence: bool,
    pub past_due: bool,
    pub submission_count: u64,
}

/// The resource representing the evidence used to support a dispute.
///
/// For more details see https://stripe.com/docs/api#dispute_evidence_object.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisputeEvidenceObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_activity_log: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_disclosure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_rebuttal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_communication: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_purchase_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_disclosure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_carrier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_tracking_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_text: Option<String>,
}

/// The resource representing a Stripe dispute.
///
/// For more details see https://stripe.com/docs/api#disputes.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dispute {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub balance_transactions: Vec<BalanceTransaction>,
    pub charge: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub evidence: DisputeEvidenceObject,
    pub evidence_details: EvidenceDetails,
    pub is_charge_refundable: bool,
    pub livemode: bool,
    pub metadata: Metadata,
    pub reason: Option<String>,
    pub status: Option<String>,
}

impl Identifiable for Dispute {
    fn id(&self) -> &str {
        &self.id
    }
}
