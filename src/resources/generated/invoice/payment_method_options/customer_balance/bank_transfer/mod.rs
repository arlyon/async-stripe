#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BankTransfer {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<crate::invoice::payment_method_options::customer_balance::bank_transfer::eu_bank_transfer::EuBankTransfer>,
    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
pub type_: Option<String>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BankTransfer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod eu_bank_transfer;
pub use eu_bank_transfer::EuBankTransfer;
