#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ChargeFraudDetails {
    /// Assessments from Stripe.
    ///
    /// If set, the value is `fraudulent`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_report: Option<String>,
    /// Assessments reported by you.
    ///
    /// If set, possible values of are `safe` and `fraudulent`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_report: Option<String>,
}
