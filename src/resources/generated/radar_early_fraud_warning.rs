// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{RadarEarlyFraudWarningId};
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{Charge, PaymentIntent};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "RadarEarlyFraudWarning".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RadarEarlyFraudWarning {
    /// Unique identifier for the object.
    pub id: RadarEarlyFraudWarningId,

    /// An EFW is actionable if it has not received a dispute and has not been fully refunded.
    ///
    /// You may wish to proactively refund a charge that receives an EFW, in order to avoid receiving a dispute later.
    pub actionable: bool,

    /// ID of the charge this early fraud warning is for, optionally expanded.
    pub charge: Expandable<Charge>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The type of fraud labelled by the issuer.
    ///
    /// One of `card_never_received`, `fraudulent_card_application`, `made_with_counterfeit_card`, `made_with_lost_card`, `made_with_stolen_card`, `misc`, `unauthorized_use_of_card`.
    pub fraud_type: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// ID of the Payment Intent this early fraud warning is for, optionally expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Expandable<PaymentIntent>>,
}

impl Object for RadarEarlyFraudWarning {
    type Id = RadarEarlyFraudWarningId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "radar.early_fraud_warning"
    }
}
