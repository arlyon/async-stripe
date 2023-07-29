#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Blik {
#[serde(skip_serializing_if = "Option::is_none")]
pub mandate_options: Option<stripe_types::setup_intent::payment_method_options::blik_mandate_options::BlikMandateOptions>,

}
