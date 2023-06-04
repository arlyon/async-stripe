/// An early fraud warning indicates that the card issuer has notified us that a
/// charge may be fraudulent.
///
/// Related guide: [Early Fraud Warnings](https://stripe.com/docs/disputes/measuring#early-fraud-warnings).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct EarlyFraudWarning {
    /// An EFW is actionable if it has not received a dispute and has not been fully refunded.
    ///
    /// You may wish to proactively refund a charge that receives an EFW, in order to avoid receiving a dispute later.
    pub actionable: bool,
    /// ID of the charge this early fraud warning is for, optionally expanded.
    pub charge: crate::Expandable<crate::charge::Charge>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// The type of fraud labelled by the issuer.
    ///
    /// One of `card_never_received`, `fraudulent_card_application`, `made_with_counterfeit_card`, `made_with_lost_card`, `made_with_stolen_card`, `misc`, `unauthorized_use_of_card`.
    pub fraud_type: String,
    /// Unique identifier for the object.
    pub id: crate::radar::early_fraud_warning::RadarEarlyFraudWarningId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: EarlyFraudWarningObject,
    /// ID of the Payment Intent this early fraud warning is for, optionally expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<crate::payment_intent::PaymentIntent>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for EarlyFraudWarning {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum EarlyFraudWarningObject {
    #[serde(rename = "radar.early_fraud_warning")]
    RadarEarlyFraudWarning,
}

impl EarlyFraudWarningObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RadarEarlyFraudWarning => "radar.early_fraud_warning",
        }
    }
}

impl AsRef<str> for EarlyFraudWarningObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for EarlyFraudWarningObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for EarlyFraudWarning {
    type Id = crate::radar::early_fraud_warning::RadarEarlyFraudWarningId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(RadarEarlyFraudWarningId);
pub mod requests;
