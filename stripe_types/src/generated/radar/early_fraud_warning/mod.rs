/// An early fraud warning indicates that the card issuer has notified us that a
/// charge may be fraudulent.
///
/// Related guide: [Early Fraud Warnings](https://stripe.com/docs/disputes/measuring#early-fraud-warnings).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EarlyFraudWarning {
    /// An EFW is actionable if it has not received a dispute and has not been fully refunded.
    ///
    /// You may wish to proactively refund a charge that receives an EFW, in order to avoid receiving a dispute later.
    pub actionable: bool,
    /// ID of the charge this early fraud warning is for, optionally expanded.
    pub charge: stripe_types::Expandable<stripe_types::charge::Charge>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The type of fraud labelled by the issuer.
    ///
    /// One of `card_never_received`, `fraudulent_card_application`, `made_with_counterfeit_card`, `made_with_lost_card`, `made_with_stolen_card`, `misc`, `unauthorized_use_of_card`.
    pub fraud_type: String,
    /// Unique identifier for the object.
    pub id: stripe_types::radar::early_fraud_warning::RadarEarlyFraudWarningId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: EarlyFraudWarningObject,
    /// ID of the Payment Intent this early fraud warning is for, optionally expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent:
        Option<stripe_types::Expandable<stripe_types::payment_intent::PaymentIntent>>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EarlyFraudWarningObject {
    RadarEarlyFraudWarning,
}

impl EarlyFraudWarningObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RadarEarlyFraudWarning => "radar.early_fraud_warning",
        }
    }
}

impl std::str::FromStr for EarlyFraudWarningObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "radar.early_fraud_warning" => Ok(Self::RadarEarlyFraudWarning),

            _ => Err(()),
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
impl serde::Serialize for EarlyFraudWarningObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EarlyFraudWarningObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for EarlyFraudWarningObject"))
    }
}
impl stripe_types::Object for EarlyFraudWarning {
    type Id = stripe_types::radar::early_fraud_warning::RadarEarlyFraudWarningId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(RadarEarlyFraudWarningId);
