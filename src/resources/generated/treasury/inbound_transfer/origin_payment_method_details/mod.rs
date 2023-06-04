#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct OriginPaymentMethodDetails {
pub billing_details: crate::treasury::received_credit::initiating_payment_method_details::billing_details::BillingDetails,
    /// The type of the payment method used in the InboundTransfer.
#[serde(rename = "type")]
pub type_: OriginPaymentMethodDetailsType,
#[serde(skip_serializing_if = "Option::is_none")]
pub us_bank_account: Option<crate::treasury::inbound_transfer::origin_payment_method_details::us_bank_account::UsBankAccount>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for OriginPaymentMethodDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The type of the payment method used in the InboundTransfer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum OriginPaymentMethodDetailsType {
    UsBankAccount,
}

impl OriginPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for OriginPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OriginPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
