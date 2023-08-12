/// Use OutboundPayments to send funds to another party's external bank account or [FinancialAccount](https://stripe.com/docs/api#financial_accounts).
///
/// To send money to an account belonging to the same user, use an [OutboundTransfer](https://stripe.com/docs/api#outbound_transfers).  Simulate OutboundPayment state changes with the `/v1/test_helpers/treasury/outbound_payments` endpoints.
/// These methods can only be called on test mode objects.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryOutboundPaymentsResourceOutboundPayment {
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
    pub destination_payment_method_details: Option<stripe_treasury::OutboundPaymentsPaymentMethodDetails>,
    /// Details about the end user.
    pub end_user_details: Option<stripe_treasury::TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails>,
    /// The date when funds are expected to arrive in the destination account.
    pub expected_arrival_date: stripe_types::Timestamp,
    /// The FinancialAccount that funds were pulled from.
    pub financial_account: String,
    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_treasury::treasury_outbound_payments_resource_outbound_payment::TreasuryOutboundPaymentId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Details about a returned OutboundPayment.
    ///
    /// Only set when the status is `returned`.
    pub returned_details: Option<stripe_treasury::TreasuryOutboundPaymentsResourceReturnedStatus>,
    /// The description that appears on the receiving end for an OutboundPayment (for example, bank statement for external bank transfer).
    pub statement_descriptor: String,
    /// Current status of the OutboundPayment: `processing`, `failed`, `posted`, `returned`, `canceled`.
    ///
    /// An OutboundPayment is `processing` if it has been created and is pending.
    /// The status changes to `posted` once the OutboundPayment has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
    /// If an OutboundPayment fails to arrive at its destination, its status will change to `returned`.
    pub status: TreasuryOutboundPaymentsResourceOutboundPaymentStatus,
    pub status_transitions: stripe_treasury::TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions,
    /// The Transaction associated with this object.
    pub transaction: stripe_types::Expandable<stripe_treasury::TreasuryTransactionsResourceTransaction>,
}
/// Current status of the OutboundPayment: `processing`, `failed`, `posted`, `returned`, `canceled`.
///
/// An OutboundPayment is `processing` if it has been created and is pending.
/// The status changes to `posted` once the OutboundPayment has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
/// If an OutboundPayment fails to arrive at its destination, its status will change to `returned`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl TreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryOutboundPaymentsResourceOutboundPaymentStatus::*;
        match self {
            Canceled => "canceled",
            Failed => "failed",
            Posted => "posted",
            Processing => "processing",
            Returned => "returned",
        }
    }
}

impl std::str::FromStr for TreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryOutboundPaymentsResourceOutboundPaymentStatus::*;
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

impl AsRef<str> for TreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TreasuryOutboundPaymentsResourceOutboundPaymentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryOutboundPaymentsResourceOutboundPaymentStatus"))
    }
}
impl stripe_types::Object for TreasuryOutboundPaymentsResourceOutboundPayment {
    type Id = stripe_treasury::treasury_outbound_payments_resource_outbound_payment::TreasuryOutboundPaymentId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(TreasuryOutboundPaymentId);
#[cfg(feature = "treasury_outbound_payments_resource_outbound_payment")]
mod requests;
#[cfg(feature = "treasury_outbound_payments_resource_outbound_payment")]
pub use requests::*;
