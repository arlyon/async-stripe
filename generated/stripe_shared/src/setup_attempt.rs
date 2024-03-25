/// A SetupAttempt describes one attempted confirmation of a SetupIntent,
/// whether that confirmation is successful or unsuccessful. You can use
/// SetupAttempts to inspect details of a specific attempt at setting up a
/// payment method using a SetupIntent.
///
/// For more details see <<https://stripe.com/docs/api/setup_attempts/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
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

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
            Ok(Box::new(Builder { out: &mut self.out, builder: SetupAttemptBuilder::deser_default() }))
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
            let application = self.application.take()?;
            let attach_to_self = self.attach_to_self.take()?;
            let created = self.created.take()?;
            let customer = self.customer.take()?;
            let flow_directions = self.flow_directions.take()?;
            let id = self.id.take()?;
            let livemode = self.livemode.take()?;
            let on_behalf_of = self.on_behalf_of.take()?;
            let payment_method = self.payment_method.take()?;
            let payment_method_details = self.payment_method_details.take()?;
            let setup_error = self.setup_error.take()?;
            let setup_intent = self.setup_intent.take()?;
            let status = self.status.take()?;
            let usage = self.usage.take()?;

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

    impl<'a> Map for Builder<'a> {
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
                    "application" => b.application = Some(FromValueOpt::from_value(v)?),
                    "attach_to_self" => b.attach_to_self = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "customer" => b.customer = Some(FromValueOpt::from_value(v)?),
                    "flow_directions" => b.flow_directions = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "on_behalf_of" => b.on_behalf_of = Some(FromValueOpt::from_value(v)?),
                    "payment_method" => b.payment_method = Some(FromValueOpt::from_value(v)?),
                    "payment_method_details" => b.payment_method_details = Some(FromValueOpt::from_value(v)?),
                    "setup_error" => b.setup_error = Some(FromValueOpt::from_value(v)?),
                    "setup_intent" => b.setup_intent = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "usage" => b.usage = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
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
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SetupAttemptFlowDirections"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SetupAttemptFlowDirections {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SetupAttemptFlowDirections> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupAttemptFlowDirections::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(SetupAttemptFlowDirections);
impl stripe_types::Object for SetupAttempt {
    type Id = stripe_shared::SetupAttemptId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(SetupAttemptId, "setatt_");
