/// A customer's `Cash balance` represents real funds.
///
/// Customers can add funds to their cash balance by sending a bank transfer.
/// These funds can be used for payment and can eventually be paid out to your bank account.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CashBalance {
    /// A hash of all cash balances available to this customer.
    ///
    /// You cannot delete a customer with any cash balances, even if the balance is 0.
    /// Amounts are represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub available: Option<std::collections::HashMap<String, i64>>,
    /// The ID of the customer whose cash balance this object represents.
    pub customer: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub settings: stripe_types::CustomerBalanceCustomerBalanceSettings,
}
