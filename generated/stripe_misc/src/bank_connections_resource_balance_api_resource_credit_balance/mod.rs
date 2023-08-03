#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BankConnectionsResourceBalanceApiResourceCreditBalance {
    /// The credit that has been used by the account holder.
    ///
    /// Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Each value is a integer amount.
    ///
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub used: Option<std::collections::HashMap<String, i64>>,
}
