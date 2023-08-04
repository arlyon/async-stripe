#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer:
        Option<stripe_types::InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
