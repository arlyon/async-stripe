#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct GeneratedFrom {
    /// The ID of the Charge that generated this PaymentMethod, if any.
    pub charge: Option<stripe_types::Expandable<stripe_core::charge::Charge>>,
    /// The ID of the SetupAttempt that generated this PaymentMethod, if any.
    pub setup_attempt: Option<stripe_types::Expandable<stripe_core::setup_attempt::SetupAttempt>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for GeneratedFrom {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
