#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StatusDetails {
    /// Details related to the closure of this FinancialAccount.
    pub closed:
        Option<stripe_treasury::treasury::financial_account::status_details::closed::Closed>,
}
pub mod closed;
pub use closed::Closed;
