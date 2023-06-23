#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SepaDebit {
#[serde(skip_serializing_if = "Option::is_none")]
pub mandate_options: Option<stripe_core::setup_intent::payment_method_options::sepa_debit_mandate_options::SepaDebitMandateOptions>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SepaDebit {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
