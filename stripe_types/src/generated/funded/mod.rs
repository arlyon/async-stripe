#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Funded {
    pub bank_transfer: stripe_types::bank_transfer::BankTransfer,
}
