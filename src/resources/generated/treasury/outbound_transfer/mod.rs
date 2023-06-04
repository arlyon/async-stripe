/// Use OutboundTransfers to transfer funds from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) to a PaymentMethod belonging to the same entity.
///
/// To send funds to a different party, use [OutboundPayments](https://stripe.com/docs/api#outbound_payments) instead.
/// You can send funds over ACH rails or through a domestic wire transfer to a user's own external bank account.  Simulate OutboundTransfer state changes with the `/v1/test_helpers/treasury/outbound_transfers` endpoints.
/// These methods can only be called on test mode objects.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct OutboundTransfer {
    /// Amount (in cents) transferred.
pub amount: i64,
    /// Returns `true` if the object can be canceled, and `false` otherwise.
pub cancelable: bool,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
pub created: crate::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
pub currency: crate::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
pub description: Option<String>,
    /// The PaymentMethod used as the payment instrument for an OutboundTransfer.
pub destination_payment_method: Option<String>,
pub destination_payment_method_details: crate::treasury::outbound_transfer::destination_payment_method_details::DestinationPaymentMethodDetails,
    /// The date when funds are expected to arrive in the destination account.
pub expected_arrival_date: crate::Timestamp,
    /// The FinancialAccount that funds were pulled from.
pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
pub id: crate::treasury::outbound_transfer::TreasuryOutboundTransferId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
pub metadata: crate::Metadata,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
pub object: OutboundTransferObject,
    /// Details about a returned OutboundTransfer.
    ///
    /// Only set when the status is `returned`.
pub returned_details: Option<crate::treasury::outbound_transfer::returned_details::ReturnedDetails>,
    /// Information about the OutboundTransfer to be sent to the recipient account.
pub statement_descriptor: String,
    /// Current status of the OutboundTransfer: `processing`, `failed`, `canceled`, `posted`, `returned`.
    ///
    /// An OutboundTransfer is `processing` if it has been created and is pending.
    /// The status changes to `posted` once the OutboundTransfer has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
    /// If an OutboundTransfer fails to arrive at its destination, its status will change to `returned`.
pub status: OutboundTransferStatus,
pub status_transitions: crate::treasury::outbound_transfer::status_transitions::StatusTransitions,
    /// The Transaction associated with this object.
pub transaction: crate::Expandable<crate::treasury::transaction::Transaction>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for OutboundTransfer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum OutboundTransferObject {
    #[serde(rename = "treasury.outbound_transfer")]
    TreasuryOutboundTransfer,
}

impl OutboundTransferObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryOutboundTransfer => "treasury.outbound_transfer",
        }
    }
}

impl AsRef<str> for OutboundTransferObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundTransferObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Current status of the OutboundTransfer: `processing`, `failed`, `canceled`, `posted`, `returned`.
///
/// An OutboundTransfer is `processing` if it has been created and is pending.
/// The status changes to `posted` once the OutboundTransfer has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
/// If an OutboundTransfer fails to arrive at its destination, its status will change to `returned`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum OutboundTransferStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl OutboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Failed => "failed",
            Self::Posted => "posted",
            Self::Processing => "processing",
            Self::Returned => "returned",
        }
    }
}

impl AsRef<str> for OutboundTransferStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for OutboundTransfer {
    type Id = crate::treasury::outbound_transfer::TreasuryOutboundTransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(TreasuryOutboundTransferId);
pub mod destination_payment_method_details;
pub mod requests;
pub use destination_payment_method_details::DestinationPaymentMethodDetails;
pub mod returned_details;
pub use returned_details::ReturnedDetails;
pub mod status_transitions;
pub use status_transitions::StatusTransitions;
