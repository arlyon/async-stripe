#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ApplicationFee {
    /// ID of the Stripe account this fee was taken from.
    pub account: stripe_types::Expandable<stripe_core::account::Account>,
    /// Amount earned, in %s.
    pub amount: i64,
    /// Amount in %s refunded (can be less than the amount attribute on the fee if a partial refund was issued).
    pub amount_refunded: i64,
    /// ID of the Connect application that earned the fee.
    pub application: stripe_types::Expandable<stripe_types::application::Application>,
    /// Balance transaction that describes the impact of this collected application fee on your account balance (not including refunds).
    pub balance_transaction:
        Option<stripe_types::Expandable<stripe_core::balance_transaction::BalanceTransaction>>,
    /// ID of the charge that the application fee was taken from.
    pub charge: stripe_types::Expandable<stripe_core::charge::Charge>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_core::application_fee::ApplicationFeeId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ApplicationFeeObject,
    /// ID of the corresponding charge on the platform account, if this fee was the result of a charge using the `destination` parameter.
    pub originating_transaction: Option<stripe_types::Expandable<stripe_core::charge::Charge>>,
    /// Whether the fee has been fully refunded.
    ///
    /// If the fee is only partially refunded, this attribute will still be false.
    pub refunded: bool,
    /// A list of refunds that have been applied to the fee.
    pub refunds: stripe_types::List<stripe_core::fee_refund::FeeRefund>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ApplicationFee {
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
pub enum ApplicationFeeObject {
    ApplicationFee,
}

impl ApplicationFeeObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ApplicationFee => "application_fee",
        }
    }
}

impl AsRef<str> for ApplicationFeeObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ApplicationFeeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for ApplicationFee {
    type Id = stripe_core::application_fee::ApplicationFeeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ApplicationFeeId, "fee_");
pub mod requests;
