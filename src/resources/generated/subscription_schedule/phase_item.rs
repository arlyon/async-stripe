/// A phase item describes the price and quantity of a phase.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PhaseItem {
    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    pub billing_thresholds: Option<crate::subscription_item::billing_thresholds::BillingThresholds>,
    /// ID of the plan to which the customer should be subscribed.
    pub plan: crate::Expandable<crate::plan::Plan>,
    /// ID of the price to which the customer should be subscribed.
    pub price: crate::Expandable<crate::price::Price>,
    /// Quantity of the plan to which the customer should be subscribed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to this `phase_item`.
    ///
    /// When set, the `default_tax_rates` on the phase do not apply to this `phase_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<crate::tax_rate::TaxRate>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PhaseItem {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
