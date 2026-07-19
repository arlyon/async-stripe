/// A SetupAttempt describes one attempted confirmation of a SetupIntent,
/// whether that confirmation is successful or unsuccessful. You can use
/// SetupAttempts to inspect details of a specific attempt at setting up a
/// payment method using a SetupIntent.
///
/// For more details see <<https://stripe.com/docs/api/setup_attempts/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupAttempt {
    /// The value of [application](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-application) on the SetupIntent at the time of this confirmation.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    pub attach_to_self: Option<bool>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The value of [customer](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-customer) on the SetupIntent at the time of this confirmation.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// The value of [customer_account](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-customer_account) on the SetupIntent at the time of this confirmation.
    pub customer_account: Option<String>,
    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    pub flow_directions: Option<Vec<SetupAttemptFlowDirections>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::SetupAttemptId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The value of [on_behalf_of](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-on_behalf_of) on the SetupIntent at the time of this confirmation.
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
    /// The value of [usage](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-usage) on the SetupIntent at the time of this confirmation, one of `off_session` or `on_session`.
    pub usage: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupAttempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupAttempt").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupAttemptBuilder {
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    attach_to_self: Option<Option<bool>>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    customer_account: Option<Option<String>>,
    flow_directions: Option<Option<Vec<SetupAttemptFlowDirections>>>,
    id: Option<stripe_shared::SetupAttemptId>,
    livemode: Option<bool>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    payment_method_details: Option<stripe_shared::SetupAttemptPaymentMethodDetails>,
    setup_error: Option<Option<Box<stripe_shared::ApiErrors>>>,
    setup_intent: Option<stripe_types::Expandable<stripe_shared::SetupIntent>>,
    status: Option<String>,
    usage: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for SetupAttempt {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupAttempt>,
        builder: SetupAttemptBuilder,
    }

    impl Visitor for Place<SetupAttempt> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupAttemptBuilder {
                    application: Deserialize::default(),
                    attach_to_self: Deserialize::default(),
                    created: Deserialize::default(),
                    customer: Deserialize::default(),
                    customer_account: Deserialize::default(),
                    flow_directions: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    on_behalf_of: Deserialize::default(),
                    payment_method: Deserialize::default(),
                    payment_method_details: Deserialize::default(),
                    setup_error: Deserialize::default(),
                    setup_intent: Deserialize::default(),
                    status: Deserialize::default(),
                    usage: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "application" => Deserialize::begin(&mut self.builder.application),
                "attach_to_self" => Deserialize::begin(&mut self.builder.attach_to_self),
                "created" => Deserialize::begin(&mut self.builder.created),
                "customer" => Deserialize::begin(&mut self.builder.customer),
                "customer_account" => Deserialize::begin(&mut self.builder.customer_account),
                "flow_directions" => Deserialize::begin(&mut self.builder.flow_directions),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "on_behalf_of" => Deserialize::begin(&mut self.builder.on_behalf_of),
                "payment_method" => Deserialize::begin(&mut self.builder.payment_method),
                "payment_method_details" => {
                    Deserialize::begin(&mut self.builder.payment_method_details)
                }
                "setup_error" => Deserialize::begin(&mut self.builder.setup_error),
                "setup_intent" => Deserialize::begin(&mut self.builder.setup_intent),
                "status" => Deserialize::begin(&mut self.builder.status),
                "usage" => Deserialize::begin(&mut self.builder.usage),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(application),
                Some(attach_to_self),
                Some(created),
                Some(customer),
                Some(customer_account),
                Some(flow_directions),
                Some(id),
                Some(livemode),
                Some(on_behalf_of),
                Some(payment_method),
                Some(payment_method_details),
                Some(setup_error),
                Some(setup_intent),
                Some(status),
                Some(usage),
            ) = (
                self.builder.application.take(),
                self.builder.attach_to_self,
                self.builder.created,
                self.builder.customer.take(),
                self.builder.customer_account.take(),
                self.builder.flow_directions.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.on_behalf_of.take(),
                self.builder.payment_method.take(),
                self.builder.payment_method_details.take(),
                self.builder.setup_error.take(),
                self.builder.setup_intent.take(),
                self.builder.status.take(),
                self.builder.usage.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SetupAttempt {
                application,
                attach_to_self,
                created,
                customer,
                customer_account,
                flow_directions,
                id,
                livemode,
                on_behalf_of,
                payment_method,
                payment_method_details,
                setup_error,
                setup_intent,
                status,
                usage,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupAttempt {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("SetupAttempt", 16)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("attach_to_self", &self.attach_to_self)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_account", &self.customer_account)?;
        s.serialize_field("flow_directions", &self.flow_directions)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("on_behalf_of", &self.on_behalf_of)?;
        s.serialize_field("payment_method", &self.payment_method)?;
        s.serialize_field("payment_method_details", &self.payment_method_details)?;
        s.serialize_field("setup_error", &self.setup_error)?;
        s.serialize_field("setup_intent", &self.setup_intent)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("usage", &self.usage)?;

        s.serialize_field("object", "setup_attempt")?;
        s.end()
    }
}
/// Indicates the directions of money movement for which this payment method is intended to be used.
///
/// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
/// Include `outbound` if you intend to use the payment method as the destination to send funds to.
/// You can include both if you intend to use the payment method for both purposes.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SetupAttemptFlowDirections {
    Inbound,
    Outbound,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SetupAttemptFlowDirections {
    pub fn as_str(&self) -> &str {
        use SetupAttemptFlowDirections::*;
        match self {
            Inbound => "inbound",
            Outbound => "outbound",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SetupAttemptFlowDirections {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupAttemptFlowDirections::*;
        match s {
            "inbound" => Ok(Inbound),
            "outbound" => Ok(Outbound),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "SetupAttemptFlowDirections");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SetupAttemptFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SetupAttemptFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupAttemptFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SetupAttemptFlowDirections)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupAttemptFlowDirections {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SetupAttemptFlowDirections {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<SetupAttemptFlowDirections> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupAttemptFlowDirections::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SetupAttemptFlowDirections {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for SetupAttempt {
    type Id = stripe_shared::SetupAttemptId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(SetupAttemptId);
