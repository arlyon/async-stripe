#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
    /// The [Balance Transaction](https://stripe.com/docs/api/balance_transactions/object) that corresponds to funds taken out of your Stripe balance.
    pub balance_transaction: stripe_types::Expandable<stripe_shared::BalanceTransaction>,
    /// The [Cash Balance Transaction](https://stripe.com/docs/api/cash_balance_transactions/object) that brought the customer balance negative, triggering the clawback of funds.
    pub linked_transaction: stripe_types::Expandable<stripe_shared::CustomerCashBalanceTransaction>,
}
