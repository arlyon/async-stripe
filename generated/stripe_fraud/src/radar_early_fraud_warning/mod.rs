/// An early fraud warning indicates that the card issuer has notified us that a
/// charge may be fraudulent.
///
/// Related guide: [Early fraud warnings](https://stripe.com/docs/disputes/measuring#early-fraud-warnings).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RadarEarlyFraudWarning {
    /// An EFW is actionable if it has not received a dispute and has not been fully refunded.
    ///
    /// You may wish to proactively refund a charge that receives an EFW, in order to avoid receiving a dispute later.
    pub actionable: bool,
    /// ID of the charge this early fraud warning is for, optionally expanded.
    pub charge: stripe_types::Expandable<stripe_types::Charge>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The type of fraud labelled by the issuer.
    ///
    /// One of `card_never_received`, `fraudulent_card_application`, `made_with_counterfeit_card`, `made_with_lost_card`, `made_with_stolen_card`, `misc`, `unauthorized_use_of_card`.
    pub fraud_type: String,
    /// Unique identifier for the object.
    pub id: stripe_fraud::radar_early_fraud_warning::RadarEarlyFraudWarningId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// ID of the Payment Intent this early fraud warning is for, optionally expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<stripe_types::Expandable<stripe_types::PaymentIntent>>,
}
impl stripe_types::Object for RadarEarlyFraudWarning {
    type Id = stripe_fraud::radar_early_fraud_warning::RadarEarlyFraudWarningId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(RadarEarlyFraudWarningId);
#[cfg(feature = "radar_early_fraud_warning")]
mod requests;
#[cfg(feature = "radar_early_fraud_warning")]
pub use requests::*;
