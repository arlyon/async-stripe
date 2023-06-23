#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DestinationPaymentMethodDetails {
pub billing_details: stripe_treasury::treasury::received_credit::initiating_payment_method_details::billing_details::BillingDetails,
    /// The type of the payment method used in the OutboundTransfer.
#[serde(rename = "type")]
pub type_: DestinationPaymentMethodDetailsType,
#[serde(skip_serializing_if = "Option::is_none")]
pub us_bank_account: Option<stripe_treasury::treasury::outbound_transfer::destination_payment_method_details::us_bank_account::UsBankAccount>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DestinationPaymentMethodDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The type of the payment method used in the OutboundTransfer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DestinationPaymentMethodDetailsType {
    UsBankAccount,
}

impl DestinationPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for DestinationPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DestinationPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
