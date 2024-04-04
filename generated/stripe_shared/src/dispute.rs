/// A dispute occurs when a customer questions your charge with their card issuer.
/// When this happens, you have the opportunity to respond to the dispute with
/// evidence that shows that the charge is legitimate.
///
/// Related guide: [Disputes and fraud](https://stripe.com/docs/disputes)
///
/// For more details see <<https://stripe.com/docs/api/disputes/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Dispute {
    /// Disputed amount.
    /// Usually the amount of the charge, but it can differ (usually because of currency fluctuation or because only part of the order is disputed).
    pub amount: i64,
    /// List of zero, one, or two balance transactions that show funds withdrawn and reinstated to your Stripe account as a result of this dispute.
    pub balance_transactions: Vec<stripe_shared::BalanceTransaction>,
    /// ID of the charge that's disputed.
    pub charge: stripe_types::Expandable<stripe_shared::Charge>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    pub evidence: stripe_shared::DisputeEvidence,
    pub evidence_details: stripe_shared::DisputeEvidenceDetails,
    /// Unique identifier for the object.
    pub id: stripe_shared::DisputeId,
    /// If true, it's still possible to refund the disputed payment.
    /// After the payment has been fully refunded, no further funds are withdrawn from your Stripe account as a result of this dispute.
    pub is_charge_refundable: bool,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Network-dependent reason code for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_reason_code: Option<String>,
    /// ID of the PaymentIntent that's disputed.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<stripe_shared::DisputePaymentMethodDetails>,
    /// Reason given by cardholder for dispute.
    /// Possible values are `bank_cannot_process`, `check_returned`, `credit_not_processed`, `customer_initiated`, `debit_not_authorized`, `duplicate`, `fraudulent`, `general`, `incorrect_account_details`, `insufficient_funds`, `product_not_received`, `product_unacceptable`, `subscription_canceled`, or `unrecognized`.
    /// Learn more about [dispute reasons](https://stripe.com/docs/disputes/categories).
    pub reason: String,
    /// Current status of dispute.
    /// Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `won`, or `lost`.
    pub status: DisputeStatus,
}
/// Current status of dispute.
/// Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `won`, or `lost`.
#[derive(Copy, Clone, Eq, PartialEq)]
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
        use DisputeStatus::*;
        match self {
            Lost => "lost",
            NeedsResponse => "needs_response",
            UnderReview => "under_review",
            WarningClosed => "warning_closed",
            WarningNeedsResponse => "warning_needs_response",
            WarningUnderReview => "warning_under_review",
            Won => "won",
        }
    }
}

impl std::str::FromStr for DisputeStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputeStatus::*;
        match s {
            "lost" => Ok(Lost),
            "needs_response" => Ok(NeedsResponse),
            "under_review" => Ok(UnderReview),
            "warning_closed" => Ok(WarningClosed),
            "warning_needs_response" => Ok(WarningNeedsResponse),
            "warning_under_review" => Ok(WarningUnderReview),
            "won" => Ok(Won),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for DisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for DisputeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DisputeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for DisputeStatus"))
    }
}
impl stripe_types::Object for Dispute {
    type Id = stripe_shared::DisputeId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(DisputeId, "dp_" | "du_" | "pdp_");
