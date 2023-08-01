#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AmountDetails {
    /// The fee charged by the ATM for the cash withdrawal.
    pub atm_fee: Option<i64>,
}
