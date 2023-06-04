#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    pub description: Option<String>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Option<u32>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
