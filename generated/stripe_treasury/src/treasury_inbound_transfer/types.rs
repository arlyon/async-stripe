/// Use [InboundTransfers](https://stripe.com/docs/treasury/moving-money/financial-accounts/into/inbound-transfers) to add funds to your [FinancialAccount](https://stripe.com/docs/api#financial_accounts) via a PaymentMethod that is owned by you.
/// The funds will be transferred via an ACH debit.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryInboundTransfer {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Returns `true` if the InboundTransfer is able to be canceled.
    pub cancelable: bool,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Details about this InboundTransfer's failure. Only set when status is `failed`.
    pub failure_details: Option<stripe_treasury::TreasuryInboundTransfersResourceFailureDetails>,
    /// The FinancialAccount that received the funds.
    pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::TreasuryInboundTransferId,
    pub linked_flows:
        stripe_treasury::TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The origin payment method to be debited for an InboundTransfer.
    pub origin_payment_method: String,
    /// Details about the PaymentMethod for an InboundTransfer.
    pub origin_payment_method_details: Option<stripe_treasury::InboundTransfers>,
    /// Returns `true` if the funds for an InboundTransfer were returned after the InboundTransfer went to the `succeeded` state.
    pub returned: Option<bool>,
    /// Statement descriptor shown when funds are debited from the source.
    /// Not all payment networks support `statement_descriptor`.
    pub statement_descriptor: String,
    /// Status of the InboundTransfer: `processing`, `succeeded`, `failed`, and `canceled`.
    /// An InboundTransfer is `processing` if it is created and pending.
    /// The status changes to `succeeded` once the funds have been "confirmed" and a `transaction` is created and posted.
    /// The status changes to `failed` if the transfer fails.
    pub status: stripe_treasury::TreasuryInboundTransferStatus,
    pub status_transitions:
        stripe_treasury::TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: Option<stripe_types::Expandable<stripe_treasury::TreasuryTransaction>>,
}
impl stripe_types::Object for TreasuryInboundTransfer {
    type Id = stripe_treasury::TreasuryInboundTransferId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TreasuryInboundTransferId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryInboundTransferStatus {
    Canceled,
    Failed,
    Processing,
    Succeeded,
}
impl TreasuryInboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryInboundTransferStatus::*;
        match self {
            Canceled => "canceled",
            Failed => "failed",
            Processing => "processing",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TreasuryInboundTransferStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryInboundTransferStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "failed" => Ok(Failed),
            "processing" => Ok(Processing),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryInboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryInboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryInboundTransferStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryInboundTransferStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TreasuryInboundTransferStatus")
        })
    }
}
