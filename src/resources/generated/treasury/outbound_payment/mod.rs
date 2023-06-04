/// Use OutboundPayments to send funds to another party's external bank account or [FinancialAccount](https://stripe.com/docs/api#financial_accounts).
///
/// To send money to an account belonging to the same user, use an [OutboundTransfer](https://stripe.com/docs/api#outbound_transfers).  Simulate OutboundPayment state changes with the `/v1/test_helpers/treasury/outbound_payments` endpoints.
/// These methods can only be called on test mode objects.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct OutboundPayment {
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
    /// ID of the [customer](https://stripe.com/docs/api/customers) to whom an OutboundPayment is sent.
pub customer: Option<String>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
pub description: Option<String>,
    /// The PaymentMethod via which an OutboundPayment is sent.
    ///
    /// This field can be empty if the OutboundPayment was created using `destination_payment_method_data`.
pub destination_payment_method: Option<String>,
    /// Details about the PaymentMethod for an OutboundPayment.
pub destination_payment_method_details: Option<crate::treasury::outbound_payment::destination_payment_method_details::DestinationPaymentMethodDetails>,
    /// Details about the end user.
pub end_user_details: Option<crate::treasury::outbound_payment::end_user_details::EndUserDetails>,
    /// The date when funds are expected to arrive in the destination account.
pub expected_arrival_date: crate::Timestamp,
    /// The FinancialAccount that funds were pulled from.
pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
pub id: crate::treasury::outbound_payment::TreasuryOutboundPaymentId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
pub metadata: crate::Metadata,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
pub object: OutboundPaymentObject,
    /// Details about a returned OutboundPayment.
    ///
    /// Only set when the status is `returned`.
pub returned_details: Option<crate::treasury::outbound_payment::returned_details::ReturnedDetails>,
    /// The description that appears on the receiving end for an OutboundPayment (for example, bank statement for external bank transfer).
pub statement_descriptor: String,
    /// Current status of the OutboundPayment: `processing`, `failed`, `posted`, `returned`, `canceled`.
    ///
    /// An OutboundPayment is `processing` if it has been created and is pending.
    /// The status changes to `posted` once the OutboundPayment has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
    /// If an OutboundPayment fails to arrive at its destination, its status will change to `returned`.
pub status: OutboundPaymentStatus,
pub status_transitions: crate::treasury::outbound_payment::status_transitions::StatusTransitions,
    /// The Transaction associated with this object.
pub transaction: crate::Expandable<crate::treasury::transaction::Transaction>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for OutboundPayment {
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
pub enum OutboundPaymentObject {
    #[serde(rename = "treasury.outbound_payment")]
    TreasuryOutboundPayment,
}

impl OutboundPaymentObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TreasuryOutboundPayment => "treasury.outbound_payment",
        }
    }
}

impl AsRef<str> for OutboundPaymentObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Current status of the OutboundPayment: `processing`, `failed`, `posted`, `returned`, `canceled`.
///
/// An OutboundPayment is `processing` if it has been created and is pending.
/// The status changes to `posted` once the OutboundPayment has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
/// If an OutboundPayment fails to arrive at its destination, its status will change to `returned`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl OutboundPaymentStatus {
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

impl AsRef<str> for OutboundPaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for OutboundPayment {
    type Id = crate::treasury::outbound_payment::TreasuryOutboundPaymentId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(TreasuryOutboundPaymentId);
pub mod destination_payment_method_details;
pub mod requests;
pub use destination_payment_method_details::DestinationPaymentMethodDetails;
pub mod end_user_details;
pub use end_user_details::EndUserDetails;
pub mod status_transitions;
pub use status_transitions::StatusTransitions;
pub mod returned_details;
pub use returned_details::ReturnedDetails;