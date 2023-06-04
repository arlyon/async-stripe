#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Installments {
    /// Installment plans that may be selected for this PaymentIntent.
    pub available_plans:
        Option<Vec<crate::payment_intent::payment_method_options::card::installments::plan::Plan>>,
    /// Whether Installments are enabled for this PaymentIntent.
    pub enabled: bool,
    /// Installment plan selected for this PaymentIntent.
    pub plan: Option<crate::payment_intent::payment_method_options::card::installments::plan::Plan>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Installments {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod plan;
pub use plan::Plan;
