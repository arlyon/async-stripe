#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FraudDetails {
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FraudDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
