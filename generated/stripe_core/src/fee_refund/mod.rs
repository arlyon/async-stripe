/// `Application Fee Refund` objects allow you to refund an application fee that
/// has previously been created but not yet refunded.
///
/// Funds will be refunded to the Stripe account from which the fee was originally collected.  Related guide: [Refunding Application Fees](https://stripe.com/docs/connect/destination-charges#refunding-app-fee).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FeeRefund {
    /// Amount, in %s.
    pub amount: i64,
    /// Balance transaction that describes the impact on your account balance.
    pub balance_transaction:
        Option<stripe_types::Expandable<stripe_core::balance_transaction::BalanceTransaction>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the application fee that was refunded.
    pub fee: stripe_types::Expandable<stripe_core::application_fee::ApplicationFee>,
    /// Unique identifier for the object.
    pub id: stripe_core::fee_refund::FeeRefundId,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<stripe_types::Metadata>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: FeeRefundObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FeeRefund {
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
pub enum FeeRefundObject {
    FeeRefund,
}

impl FeeRefundObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FeeRefund => "fee_refund",
        }
    }
}

impl AsRef<str> for FeeRefundObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FeeRefundObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for FeeRefund {
    type Id = stripe_core::fee_refund::FeeRefundId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FeeRefundId);
pub mod requests;
