/// A SetupAttempt describes one attempted confirmation of a SetupIntent,
/// whether that confirmation is successful or unsuccessful. You can use
/// SetupAttempts to inspect details of a specific attempt at setting up a
/// payment method using a SetupIntent.
///
/// For more details see <<https://stripe.com/docs/api/setup_attempts/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SetupAttempt {
    /// The value of [application](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-application) on the SetupIntent at the time of this confirmation.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The value of [customer](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-customer) on the SetupIntent at the time of this confirmation.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    pub flow_directions: Option<Vec<SetupAttemptFlowDirections>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::SetupAttemptId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The value of [on_behalf_of](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-on_behalf_of) on the SetupIntent at the time of this confirmation.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// ID of the payment method used with this SetupAttempt.
    pub payment_method: stripe_types::Expandable<stripe_shared::PaymentMethod>,
    pub payment_method_details: stripe_shared::SetupAttemptPaymentMethodDetails,
    /// The error encountered during this attempt to confirm the SetupIntent, if any.
    pub setup_error: Option<Box<stripe_shared::ApiErrors>>,
    /// ID of the SetupIntent that this attempt belongs to.
    pub setup_intent: stripe_types::Expandable<stripe_shared::SetupIntent>,
    /// Status of this SetupAttempt, one of `requires_confirmation`, `requires_action`, `processing`, `succeeded`, `failed`, or `abandoned`.
    pub status: String,
    /// The value of [usage](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-usage) on the SetupIntent at the time of this confirmation, one of `off_session` or `on_session`.
    pub usage: String,
}
/// Indicates the directions of money movement for which this payment method is intended to be used.
///
/// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
/// Include `outbound` if you intend to use the payment method as the destination to send funds to.
/// You can include both if you intend to use the payment method for both purposes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupAttemptFlowDirections {
    Inbound,
    Outbound,
}
impl SetupAttemptFlowDirections {
    pub fn as_str(self) -> &'static str {
        use SetupAttemptFlowDirections::*;
        match self {
            Inbound => "inbound",
            Outbound => "outbound",
        }
    }
}

impl std::str::FromStr for SetupAttemptFlowDirections {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupAttemptFlowDirections::*;
        match s {
            "inbound" => Ok(Inbound),
            "outbound" => Ok(Outbound),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for SetupAttemptFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupAttemptFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupAttemptFlowDirections {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupAttemptFlowDirections {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SetupAttemptFlowDirections"))
    }
}
impl stripe_types::Object for SetupAttempt {
    type Id = stripe_shared::SetupAttemptId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(SetupAttemptId, "setatt_");
