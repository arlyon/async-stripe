/// Use OutboundTransfers to transfer funds from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) to a PaymentMethod belonging to the same entity.
///
/// To send funds to a different party, use [OutboundPayments](https://stripe.com/docs/api#outbound_payments) instead.
/// You can send funds over ACH rails or through a domestic wire transfer to a user's own external bank account.  Simulate OutboundTransfer state changes with the `/v1/test_helpers/treasury/outbound_transfers` endpoints.
/// These methods can only be called on test mode objects.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryOutboundTransfersResourceOutboundTransfer {
    /// Amount (in cents) transferred.
    pub amount: i64,
    /// Returns `true` if the object can be canceled, and `false` otherwise.
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
    /// The PaymentMethod used as the payment instrument for an OutboundTransfer.
    pub destination_payment_method: Option<String>,
    pub destination_payment_method_details: stripe_treasury::OutboundTransfersPaymentMethodDetails,
    /// The date when funds are expected to arrive in the destination account.
    pub expected_arrival_date: stripe_types::Timestamp,
    /// The FinancialAccount that funds were pulled from.
    pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::treasury_outbound_transfers_resource_outbound_transfer::TreasuryOutboundTransferId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TreasuryOutboundTransfersResourceOutboundTransferObject,
    /// Details about a returned OutboundTransfer.
    ///
    /// Only set when the status is `returned`.
    pub returned_details: Option<stripe_treasury::TreasuryOutboundTransfersResourceReturnedDetails>,
    /// Information about the OutboundTransfer to be sent to the recipient account.
    pub statement_descriptor: String,
    /// Current status of the OutboundTransfer: `processing`, `failed`, `canceled`, `posted`, `returned`.
    ///
    /// An OutboundTransfer is `processing` if it has been created and is pending.
    /// The status changes to `posted` once the OutboundTransfer has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
    /// If an OutboundTransfer fails to arrive at its destination, its status will change to `returned`.
    pub status: TreasuryOutboundTransfersResourceOutboundTransferStatus,
    pub status_transitions: stripe_treasury::TreasuryOutboundTransfersResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: stripe_types::Expandable<stripe_treasury::TreasuryTransactionsResourceTransaction>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryOutboundTransfersResourceOutboundTransferObject {
    TreasuryOutboundTransfer,
}

impl TreasuryOutboundTransfersResourceOutboundTransferObject {
    pub fn as_str(self) -> &'static str {
        use TreasuryOutboundTransfersResourceOutboundTransferObject::*;
        match self {
            TreasuryOutboundTransfer => "treasury.outbound_transfer",
        }
    }
}

impl std::str::FromStr for TreasuryOutboundTransfersResourceOutboundTransferObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryOutboundTransfersResourceOutboundTransferObject::*;
        match s {
            "treasury.outbound_transfer" => Ok(TreasuryOutboundTransfer),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryOutboundTransfersResourceOutboundTransferObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryOutboundTransfersResourceOutboundTransferObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryOutboundTransfersResourceOutboundTransferObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryOutboundTransfersResourceOutboundTransferObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryOutboundTransfersResourceOutboundTransferObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryOutboundTransfersResourceOutboundTransferObject"))
    }
}
/// Current status of the OutboundTransfer: `processing`, `failed`, `canceled`, `posted`, `returned`.
///
/// An OutboundTransfer is `processing` if it has been created and is pending.
/// The status changes to `posted` once the OutboundTransfer has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
/// If an OutboundTransfer fails to arrive at its destination, its status will change to `returned`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryOutboundTransfersResourceOutboundTransferStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl TreasuryOutboundTransfersResourceOutboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryOutboundTransfersResourceOutboundTransferStatus::*;
        match self {
            Canceled => "canceled",
            Failed => "failed",
            Posted => "posted",
            Processing => "processing",
            Returned => "returned",
        }
    }
}

impl std::str::FromStr for TreasuryOutboundTransfersResourceOutboundTransferStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryOutboundTransfersResourceOutboundTransferStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "failed" => Ok(Failed),
            "posted" => Ok(Posted),
            "processing" => Ok(Processing),
            "returned" => Ok(Returned),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TreasuryOutboundTransfersResourceOutboundTransferStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryOutboundTransfersResourceOutboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryOutboundTransfersResourceOutboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryOutboundTransfersResourceOutboundTransferStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryOutboundTransfersResourceOutboundTransferStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryOutboundTransfersResourceOutboundTransferStatus"))
    }
}
impl stripe_types::Object for TreasuryOutboundTransfersResourceOutboundTransfer {
    type Id = stripe_treasury::treasury_outbound_transfers_resource_outbound_transfer::TreasuryOutboundTransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TreasuryOutboundTransferId);
pub mod requests;
