#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Funded {
    pub bank_transfer:
        crate::customer_cash_balance_transaction::funded::bank_transfer::BankTransfer,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Funded {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod bank_transfer;
pub use bank_transfer::BankTransfer;
