/// A dispute occurs when a customer questions your charge with their card issuer.
/// When this happens, you're given the opportunity to respond to the dispute with
/// evidence that shows that the charge is legitimate.
///
/// You can find more information about the dispute process in our [Disputes and Fraud](/docs/disputes) documentation.  Related guide: [Disputes and fraud](https://stripe.com/docs/disputes).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Dispute {
    /// Disputed amount.
    ///
    /// Usually the amount of the charge, but can differ (usually because of currency fluctuation or because only part of the order is disputed).
    pub amount: i64,
    /// List of zero, one, or two balance transactions that show funds withdrawn and reinstated to your Stripe account as a result of this dispute.
    pub balance_transactions: Vec<stripe_types::balance_transaction::BalanceTransaction>,
    /// ID of the charge that was disputed.
    pub charge: stripe_types::Expandable<stripe_types::charge::Charge>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    pub evidence: stripe_types::evidence::Evidence,
    pub evidence_details: stripe_types::evidence_details::EvidenceDetails,
    /// Unique identifier for the object.
    pub id: stripe_types::dispute::DisputeId,
    /// If true, it is still possible to refund the disputed payment.
    ///
    /// Once the payment has been fully refunded, no further funds will be withdrawn from your Stripe account as a result of this dispute.
    pub is_charge_refundable: bool,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Network-dependent reason code for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_reason_code: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DisputeObject,
    /// ID of the PaymentIntent that was disputed.
    pub payment_intent:
        Option<stripe_types::Expandable<stripe_types::payment_intent::PaymentIntent>>,
    /// Reason given by cardholder for dispute.
    ///
    /// Possible values are `bank_cannot_process`, `check_returned`, `credit_not_processed`, `customer_initiated`, `debit_not_authorized`, `duplicate`, `fraudulent`, `general`, `incorrect_account_details`, `insufficient_funds`, `product_not_received`, `product_unacceptable`, `subscription_canceled`, or `unrecognized`.
    /// Read more about [dispute reasons](https://stripe.com/docs/disputes/categories).
    pub reason: String,
    /// Current status of dispute.
    ///
    /// Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `charge_refunded`, `won`, or `lost`.
    pub status: DisputeStatus,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DisputeObject {
    Dispute,
}

impl DisputeObject {
    pub fn as_str(self) -> &'static str {
        use DisputeObject::*;
        match self {
            Dispute => "dispute",
        }
    }
}

impl std::str::FromStr for DisputeObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputeObject::*;
        match s {
            "dispute" => Ok(Dispute),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for DisputeObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DisputeObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DisputeObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for DisputeObject"))
    }
}
/// Current status of dispute.
///
/// Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `charge_refunded`, `won`, or `lost`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DisputeStatus {
    ChargeRefunded,
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
            ChargeRefunded => "charge_refunded",
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
            "charge_refunded" => Ok(ChargeRefunded),
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for DisputeStatus"))
    }
}
impl stripe_types::Object for Dispute {
    type Id = stripe_types::dispute::DisputeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(DisputeId, "dp_" | "du_");
