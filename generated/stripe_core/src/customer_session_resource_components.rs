/// Configuration for the components supported by this customer session.
#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomerSessionResourceComponents {
    pub buy_button: stripe_core::CustomerSessionResourceComponentsResourceBuyButton,
    pub pricing_table: stripe_core::CustomerSessionResourceComponentsResourcePricingTable,
}
