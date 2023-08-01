/// Use [InboundTransfers](https://stripe.com/docs/treasury/moving-money/financial-accounts/into/inbound-transfers) to add funds to your [FinancialAccount](https://stripe.com/docs/api#financial_accounts) via a PaymentMethod that is owned by you.
///
/// The funds will be transferred via an ACH debit.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InboundTransfer {
    /// Amount (in cents) transferred.
pub amount: i64,
    /// Returns `true` if the InboundTransfer is able to be canceled.
pub cancelable: bool,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
pub description: Option<String>,
    /// Details about this InboundTransfer's failure.
    ///
    /// Only set when status is `failed`.
pub failure_details: Option<stripe_treasury::treasury::inbound_transfer::failure_details::FailureDetails>,
    /// The FinancialAccount that received the funds.
pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
pub id: stripe_treasury::treasury::inbound_transfer::TreasuryInboundTransferId,
pub linked_flows: stripe_treasury::treasury::inbound_transfer::linked_flows::LinkedFlows,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
pub object: InboundTransferObject,
    /// The origin payment method to be debited for an InboundTransfer.
pub origin_payment_method: String,
    /// Details about the PaymentMethod for an InboundTransfer.
pub origin_payment_method_details: Option<stripe_treasury::treasury::inbound_transfer::origin_payment_method_details::OriginPaymentMethodDetails>,
    /// Returns `true` if the funds for an InboundTransfer were returned after the InboundTransfer went to the `succeeded` state.
pub returned: Option<bool>,
    /// Statement descriptor shown when funds are debited from the source.
    ///
    /// Not all payment networks support `statement_descriptor`.
pub statement_descriptor: String,
    /// Status of the InboundTransfer: `processing`, `succeeded`, `failed`, and `canceled`.
    ///
    /// An InboundTransfer is `processing` if it is created and pending.
    /// The status changes to `succeeded` once the funds have been "confirmed" and a `transaction` is created and posted.
    /// The status changes to `failed` if the transfer fails.
pub status: InboundTransferStatus,
pub status_transitions: stripe_treasury::treasury::inbound_transfer::status_transitions::StatusTransitions,
    /// The Transaction associated with this object.
pub transaction: Option<stripe_types::Expandable<stripe_treasury::treasury::transaction::Transaction>>,

}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InboundTransferObject {
    TreasuryInboundTransfer,
}

impl InboundTransferObject {
    pub fn as_str(self) -> &'static str {
        use InboundTransferObject::*;
        match self {
            TreasuryInboundTransfer => "treasury.inbound_transfer",
        }
    }
}

impl std::str::FromStr for InboundTransferObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransferObject::*;
        match s {
            "treasury.inbound_transfer" => Ok(TreasuryInboundTransfer),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for InboundTransferObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InboundTransferObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for InboundTransferObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InboundTransferObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InboundTransferObject"))
    }
}
/// Status of the InboundTransfer: `processing`, `succeeded`, `failed`, and `canceled`.
///
/// An InboundTransfer is `processing` if it is created and pending.
/// The status changes to `succeeded` once the funds have been "confirmed" and a `transaction` is created and posted.
/// The status changes to `failed` if the transfer fails.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InboundTransferStatus {
    Canceled,
    Failed,
    Processing,
    Succeeded,
}

impl InboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        use InboundTransferStatus::*;
        match self {
            Canceled => "canceled",
            Failed => "failed",
            Processing => "processing",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for InboundTransferStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransferStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "failed" => Ok(Failed),
            "processing" => Ok(Processing),
            "succeeded" => Ok(Succeeded),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for InboundTransferStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for InboundTransferStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InboundTransferStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InboundTransferStatus"))
    }
}
impl stripe_types::Object for InboundTransfer {
    type Id = stripe_treasury::treasury::inbound_transfer::TreasuryInboundTransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryInboundTransferId);
pub mod origin_payment_method_details;
pub use origin_payment_method_details::OriginPaymentMethodDetails;
pub mod failure_details;
pub use failure_details::FailureDetails;
pub mod linked_flows;
pub use linked_flows::LinkedFlows;
pub mod status_transitions;
pub use status_transitions::StatusTransitions;
pub mod requests;
