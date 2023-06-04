#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DestinationPaymentMethodDetails {
pub billing_details: crate::treasury::received_credit::initiating_payment_method_details::billing_details::BillingDetails,
#[serde(skip_serializing_if = "Option::is_none")]
pub financial_account: Option<crate::treasury::outbound_payment::destination_payment_method_details::financial_account::FinancialAccount>,
    /// The type of the payment method used in the OutboundPayment.
#[serde(rename = "type")]
pub type_: DestinationPaymentMethodDetailsType,
#[serde(skip_serializing_if = "Option::is_none")]
pub us_bank_account: Option<crate::treasury::outbound_payment::destination_payment_method_details::us_bank_account::UsBankAccount>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DestinationPaymentMethodDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The type of the payment method used in the OutboundPayment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum DestinationPaymentMethodDetailsType {
    FinancialAccount,
    UsBankAccount,
}

impl DestinationPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinancialAccount => "financial_account",
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
pub mod financial_account;
pub use financial_account::FinancialAccount;
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;