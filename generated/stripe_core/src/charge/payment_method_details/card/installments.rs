#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Installments {
    /// Installment plan selected for the payment.
    pub plan:
        Option<stripe_core::payment_intent::payment_method_options::card::installments::plan::Plan>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Installments {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
