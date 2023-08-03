#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentFlowsAmountDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<stripe_types::PaymentFlowsAmountDetailsResourceTip>,
}
