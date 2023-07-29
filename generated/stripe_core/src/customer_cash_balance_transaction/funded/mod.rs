#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Funded {
    pub bank_transfer:
        stripe_core::customer_cash_balance_transaction::funded::bank_transfer::BankTransfer,
}
pub mod bank_transfer;
pub use bank_transfer::BankTransfer;
