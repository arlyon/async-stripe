/// A SetupAttempt describes one attempted confirmation of a SetupIntent,
/// whether that confirmation is successful or unsuccessful. You can use
/// SetupAttempts to inspect details of a specific attempt at setting up a
/// payment method using a SetupIntent.
///
/// For more details see <<https://stripe.com/docs/api/setup_attempts/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupAttempt {
    /// The value of [application](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-application) on the SetupIntent at the time of this confirmation.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
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
#[doc(hidden)]
pub struct SetupAttemptBuilder {
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    attach_to_self: Option<Option<bool>>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: SetupAttemptBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupAttemptBuilder {
        type Out = SetupAttempt;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "application" => Deserialize::begin(&mut self.application),
                "attach_to_self" => Deserialize::begin(&mut self.attach_to_self),
                "created" => Deserialize::begin(&mut self.created),
                "customer" => Deserialize::begin(&mut self.customer),
                "flow_directions" => Deserialize::begin(&mut self.flow_directions),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "on_behalf_of" => Deserialize::begin(&mut self.on_behalf_of),
                "payment_method" => Deserialize::begin(&mut self.payment_method),
                "payment_method_details" => Deserialize::begin(&mut self.payment_method_details),
                "setup_error" => Deserialize::begin(&mut self.setup_error),
                "setup_intent" => Deserialize::begin(&mut self.setup_intent),
                "status" => Deserialize::begin(&mut self.status),
                "usage" => Deserialize::begin(&mut self.usage),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                application: Deserialize::default(),
                attach_to_self: Deserialize::default(),
                created: Deserialize::default(),
                customer: Deserialize::default(),
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(application),
                Some(attach_to_self),
                Some(created),
                Some(customer),
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
                self.application.take(),
                self.attach_to_self,
                self.created,
                self.customer.take(),
                self.flow_directions.take(),
                self.id.take(),
                self.livemode,
                self.on_behalf_of.take(),
                self.payment_method.take(),
                self.payment_method_details.take(),
                self.setup_error.take(),
                self.setup_intent.take(),
                self.status.take(),
                self.usage.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                application,
                attach_to_self,
                created,
                customer,
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
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SetupAttempt {
        type Builder = SetupAttemptBuilder;
    }

    impl FromValueOpt for SetupAttempt {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupAttemptBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "application" => b.application = FromValueOpt::from_value(v),
                    "attach_to_self" => b.attach_to_self = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "flow_directions" => b.flow_directions = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "on_behalf_of" => b.on_behalf_of = FromValueOpt::from_value(v),
                    "payment_method" => b.payment_method = FromValueOpt::from_value(v),
                    "payment_method_details" => {
                        b.payment_method_details = FromValueOpt::from_value(v)
                    }
                    "setup_error" => b.setup_error = FromValueOpt::from_value(v),
                    "setup_intent" => b.setup_intent = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "usage" => b.usage = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupAttempt {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("SetupAttempt", 15)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("attach_to_self", &self.attach_to_self)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer", &self.customer)?;
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

impl std::fmt::Debug for SetupAttemptFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for SetupAttemptFlowDirections {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SetupAttemptFlowDirections> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupAttemptFlowDirections::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SetupAttemptFlowDirections);
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
