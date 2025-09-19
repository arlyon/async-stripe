/// A SetupIntent guides you through the process of setting up and saving a customer's payment credentials for future payments.
/// For example, you can use a SetupIntent to set up and save your customer's card without immediately collecting a payment.
/// Later, you can use [PaymentIntents](https://stripe.com/docs/api#payment_intents) to drive the payment flow.
///
/// Create a SetupIntent when you're ready to collect your customer's payment credentials.
/// Don't maintain long-lived, unconfirmed SetupIntents because they might not be valid.
/// The SetupIntent transitions through multiple [statuses](https://docs.stripe.com/payments/intents#intent-statuses) as it guides.
/// you through the setup process.
///
/// Successful SetupIntents result in payment credentials that are optimized for future payments.
/// For example, cardholders in [certain regions](https://stripe.com/guides/strong-customer-authentication) might need to be run through.
/// [Strong Customer Authentication](https://docs.stripe.com/strong-customer-authentication) during payment method collection.
/// to streamline later [off-session payments](https://docs.stripe.com/payments/setup-intents).
/// If you use the SetupIntent with a [Customer](https://stripe.com/docs/api#setup_intent_object-customer),.
/// it automatically attaches the resulting payment method to that Customer after successful setup.
/// We recommend using SetupIntents or [setup_future_usage](https://stripe.com/docs/api#payment_intent_object-setup_future_usage) on.
/// PaymentIntents to save payment methods to prevent saving invalid or unoptimized payment methods.
///
/// By using SetupIntents, you can reduce friction for your customers, even as regulations change over time.
///
/// Related guide: [Setup Intents API](https://docs.stripe.com/payments/setup-intents)
///
/// For more details see <<https://stripe.com/docs/api/setup_intents/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntent {
    /// ID of the Connect application that created the SetupIntent.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    pub attach_to_self: Option<bool>,
    /// Settings for dynamic payment methods compatible with this Setup Intent
    pub automatic_payment_methods:
        Option<stripe_shared::PaymentFlowsAutomaticPaymentMethodsSetupIntent>,
    /// Reason for cancellation of this SetupIntent, one of `abandoned`, `requested_by_customer`, or `duplicate`.
    pub cancellation_reason: Option<stripe_shared::SetupIntentCancellationReason>,
    /// The client secret of this SetupIntent. Used for client-side retrieval using a publishable key.
    ///
    /// The client secret can be used to complete payment setup from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    pub client_secret: Option<String>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    pub flow_directions: Option<Vec<stripe_shared::SetupIntentFlowDirections>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::SetupIntentId,
    /// The error encountered in the previous SetupIntent confirmation.
    pub last_setup_error: Option<Box<stripe_shared::ApiErrors>>,
    /// The most recent SetupAttempt for this SetupIntent.
    pub latest_attempt: Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// ID of the multi use Mandate generated by the SetupIntent.
    pub mandate: Option<stripe_types::Expandable<stripe_shared::Mandate>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// If present, this property tells you what actions you need to take in order for your customer to continue payment setup.
    pub next_action: Option<stripe_shared::SetupIntentNextAction>,
    /// The account (if any) for which the setup is intended.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// ID of the payment method used with this SetupIntent.
    /// If the payment method is `card_present` and isn't a digital wallet, then the [generated_card](https://docs.stripe.com/api/setup_attempts/object#setup_attempt_object-payment_method_details-card_present-generated_card) associated with the `latest_attempt` is attached to the Customer instead.
    pub payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// Information about the [payment method configuration](https://stripe.com/docs/api/payment_method_configurations) used for this Setup Intent.
    pub payment_method_configuration_details:
        Option<stripe_shared::PaymentMethodConfigBizPaymentMethodConfigurationDetails>,
    /// Payment method-specific configuration for this SetupIntent.
    pub payment_method_options: Option<stripe_shared::SetupIntentPaymentMethodOptions>,
    /// The list of payment method types (e.g.
    /// card) that this SetupIntent is allowed to set up.
    /// A list of valid payment method types can be found [here](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type).
    pub payment_method_types: Vec<String>,
    /// ID of the single_use Mandate generated by the SetupIntent.
    pub single_use_mandate: Option<stripe_types::Expandable<stripe_shared::Mandate>>,
    /// [Status](https://stripe.com/docs/payments/intents#intent-statuses) of this SetupIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `canceled`, or `succeeded`.
    pub status: SetupIntentStatus,
    /// Indicates how the payment method is intended to be used in the future.
    ///
    /// Use `on_session` if you intend to only reuse the payment method when the customer is in your checkout flow.
    /// Use `off_session` if your customer may or may not be in your checkout flow.
    /// If not provided, this value defaults to `off_session`.
    pub usage: String,
}
#[doc(hidden)]
pub struct SetupIntentBuilder {
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    attach_to_self: Option<Option<bool>>,
    automatic_payment_methods:
        Option<Option<stripe_shared::PaymentFlowsAutomaticPaymentMethodsSetupIntent>>,
    cancellation_reason: Option<Option<stripe_shared::SetupIntentCancellationReason>>,
    client_secret: Option<Option<String>>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    description: Option<Option<String>>,
    flow_directions: Option<Option<Vec<stripe_shared::SetupIntentFlowDirections>>>,
    id: Option<stripe_shared::SetupIntentId>,
    last_setup_error: Option<Option<Box<stripe_shared::ApiErrors>>>,
    latest_attempt: Option<Option<stripe_types::Expandable<stripe_shared::SetupAttempt>>>,
    livemode: Option<bool>,
    mandate: Option<Option<stripe_types::Expandable<stripe_shared::Mandate>>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    next_action: Option<Option<stripe_shared::SetupIntentNextAction>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    payment_method: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    payment_method_configuration_details:
        Option<Option<stripe_shared::PaymentMethodConfigBizPaymentMethodConfigurationDetails>>,
    payment_method_options: Option<Option<stripe_shared::SetupIntentPaymentMethodOptions>>,
    payment_method_types: Option<Vec<String>>,
    single_use_mandate: Option<Option<stripe_types::Expandable<stripe_shared::Mandate>>>,
    status: Option<SetupIntentStatus>,
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

    impl Deserialize for SetupIntent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntent>,
        builder: SetupIntentBuilder,
    }

    impl Visitor for Place<SetupIntent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentBuilder {
        type Out = SetupIntent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "application" => Deserialize::begin(&mut self.application),
                "attach_to_self" => Deserialize::begin(&mut self.attach_to_self),
                "automatic_payment_methods" => {
                    Deserialize::begin(&mut self.automatic_payment_methods)
                }
                "cancellation_reason" => Deserialize::begin(&mut self.cancellation_reason),
                "client_secret" => Deserialize::begin(&mut self.client_secret),
                "created" => Deserialize::begin(&mut self.created),
                "customer" => Deserialize::begin(&mut self.customer),
                "description" => Deserialize::begin(&mut self.description),
                "flow_directions" => Deserialize::begin(&mut self.flow_directions),
                "id" => Deserialize::begin(&mut self.id),
                "last_setup_error" => Deserialize::begin(&mut self.last_setup_error),
                "latest_attempt" => Deserialize::begin(&mut self.latest_attempt),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "mandate" => Deserialize::begin(&mut self.mandate),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "next_action" => Deserialize::begin(&mut self.next_action),
                "on_behalf_of" => Deserialize::begin(&mut self.on_behalf_of),
                "payment_method" => Deserialize::begin(&mut self.payment_method),
                "payment_method_configuration_details" => {
                    Deserialize::begin(&mut self.payment_method_configuration_details)
                }
                "payment_method_options" => Deserialize::begin(&mut self.payment_method_options),
                "payment_method_types" => Deserialize::begin(&mut self.payment_method_types),
                "single_use_mandate" => Deserialize::begin(&mut self.single_use_mandate),
                "status" => Deserialize::begin(&mut self.status),
                "usage" => Deserialize::begin(&mut self.usage),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                application: Deserialize::default(),
                attach_to_self: Deserialize::default(),
                automatic_payment_methods: Deserialize::default(),
                cancellation_reason: Deserialize::default(),
                client_secret: Deserialize::default(),
                created: Deserialize::default(),
                customer: Deserialize::default(),
                description: Deserialize::default(),
                flow_directions: Deserialize::default(),
                id: Deserialize::default(),
                last_setup_error: Deserialize::default(),
                latest_attempt: Deserialize::default(),
                livemode: Deserialize::default(),
                mandate: Deserialize::default(),
                metadata: Deserialize::default(),
                next_action: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                payment_method: Deserialize::default(),
                payment_method_configuration_details: Deserialize::default(),
                payment_method_options: Deserialize::default(),
                payment_method_types: Deserialize::default(),
                single_use_mandate: Deserialize::default(),
                status: Deserialize::default(),
                usage: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(application),
                Some(attach_to_self),
                Some(automatic_payment_methods),
                Some(cancellation_reason),
                Some(client_secret),
                Some(created),
                Some(customer),
                Some(description),
                Some(flow_directions),
                Some(id),
                Some(last_setup_error),
                Some(latest_attempt),
                Some(livemode),
                Some(mandate),
                Some(metadata),
                Some(next_action),
                Some(on_behalf_of),
                Some(payment_method),
                Some(payment_method_configuration_details),
                Some(payment_method_options),
                Some(payment_method_types),
                Some(single_use_mandate),
                Some(status),
                Some(usage),
            ) = (
                self.application.take(),
                self.attach_to_self,
                self.automatic_payment_methods,
                self.cancellation_reason,
                self.client_secret.take(),
                self.created,
                self.customer.take(),
                self.description.take(),
                self.flow_directions.take(),
                self.id.take(),
                self.last_setup_error.take(),
                self.latest_attempt.take(),
                self.livemode,
                self.mandate.take(),
                self.metadata.take(),
                self.next_action.take(),
                self.on_behalf_of.take(),
                self.payment_method.take(),
                self.payment_method_configuration_details.take(),
                self.payment_method_options.take(),
                self.payment_method_types.take(),
                self.single_use_mandate.take(),
                self.status,
                self.usage.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                application,
                attach_to_self,
                automatic_payment_methods,
                cancellation_reason,
                client_secret,
                created,
                customer,
                description,
                flow_directions,
                id,
                last_setup_error,
                latest_attempt,
                livemode,
                mandate,
                metadata,
                next_action,
                on_behalf_of,
                payment_method,
                payment_method_configuration_details,
                payment_method_options,
                payment_method_types,
                single_use_mandate,
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

    impl ObjectDeser for SetupIntent {
        type Builder = SetupIntentBuilder;
    }

    impl FromValueOpt for SetupIntent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "application" => b.application = FromValueOpt::from_value(v),
                    "attach_to_self" => b.attach_to_self = FromValueOpt::from_value(v),
                    "automatic_payment_methods" => {
                        b.automatic_payment_methods = FromValueOpt::from_value(v)
                    }
                    "cancellation_reason" => b.cancellation_reason = FromValueOpt::from_value(v),
                    "client_secret" => b.client_secret = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "flow_directions" => b.flow_directions = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "last_setup_error" => b.last_setup_error = FromValueOpt::from_value(v),
                    "latest_attempt" => b.latest_attempt = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "mandate" => b.mandate = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "next_action" => b.next_action = FromValueOpt::from_value(v),
                    "on_behalf_of" => b.on_behalf_of = FromValueOpt::from_value(v),
                    "payment_method" => b.payment_method = FromValueOpt::from_value(v),
                    "payment_method_configuration_details" => {
                        b.payment_method_configuration_details = FromValueOpt::from_value(v)
                    }
                    "payment_method_options" => {
                        b.payment_method_options = FromValueOpt::from_value(v)
                    }
                    "payment_method_types" => b.payment_method_types = FromValueOpt::from_value(v),
                    "single_use_mandate" => b.single_use_mandate = FromValueOpt::from_value(v),
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
impl serde::Serialize for SetupIntent {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("SetupIntent", 25)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("attach_to_self", &self.attach_to_self)?;
        s.serialize_field("automatic_payment_methods", &self.automatic_payment_methods)?;
        s.serialize_field("cancellation_reason", &self.cancellation_reason)?;
        s.serialize_field("client_secret", &self.client_secret)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("flow_directions", &self.flow_directions)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("last_setup_error", &self.last_setup_error)?;
        s.serialize_field("latest_attempt", &self.latest_attempt)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("mandate", &self.mandate)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("next_action", &self.next_action)?;
        s.serialize_field("on_behalf_of", &self.on_behalf_of)?;
        s.serialize_field("payment_method", &self.payment_method)?;
        s.serialize_field(
            "payment_method_configuration_details",
            &self.payment_method_configuration_details,
        )?;
        s.serialize_field("payment_method_options", &self.payment_method_options)?;
        s.serialize_field("payment_method_types", &self.payment_method_types)?;
        s.serialize_field("single_use_mandate", &self.single_use_mandate)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("usage", &self.usage)?;

        s.serialize_field("object", "setup_intent")?;
        s.end()
    }
}
/// [Status](https://stripe.com/docs/payments/intents#intent-statuses) of this SetupIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `canceled`, or `succeeded`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
}
impl SetupIntentStatus {
    pub fn as_str(self) -> &'static str {
        use SetupIntentStatus::*;
        match self {
            Canceled => "canceled",
            Processing => "processing",
            RequiresAction => "requires_action",
            RequiresConfirmation => "requires_confirmation",
            RequiresPaymentMethod => "requires_payment_method",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for SetupIntentStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "processing" => Ok(Processing),
            "requires_action" => Ok(RequiresAction),
            "requires_confirmation" => Ok(RequiresConfirmation),
            "requires_payment_method" => Ok(RequiresPaymentMethod),
            "succeeded" => Ok(Succeeded),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SetupIntentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupIntentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SetupIntentStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SetupIntentStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupIntentStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SetupIntentStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SetupIntentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentStatus"))
    }
}
impl stripe_types::Object for SetupIntent {
    type Id = stripe_shared::SetupIntentId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(SetupIntentId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentCancellationReason {
    Abandoned,
    Duplicate,
    RequestedByCustomer,
}
impl SetupIntentCancellationReason {
    pub fn as_str(self) -> &'static str {
        use SetupIntentCancellationReason::*;
        match self {
            Abandoned => "abandoned",
            Duplicate => "duplicate",
            RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl std::str::FromStr for SetupIntentCancellationReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentCancellationReason::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "duplicate" => Ok(Duplicate),
            "requested_by_customer" => Ok(RequestedByCustomer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SetupIntentCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentCancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SetupIntentCancellationReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SetupIntentCancellationReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupIntentCancellationReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SetupIntentCancellationReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SetupIntentCancellationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SetupIntentCancellationReason")
        })
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentFlowDirections {
    Inbound,
    Outbound,
}
impl SetupIntentFlowDirections {
    pub fn as_str(self) -> &'static str {
        use SetupIntentFlowDirections::*;
        match self {
            Inbound => "inbound",
            Outbound => "outbound",
        }
    }
}

impl std::str::FromStr for SetupIntentFlowDirections {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentFlowDirections::*;
        match s {
            "inbound" => Ok(Inbound),
            "outbound" => Ok(Outbound),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SetupIntentFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentFlowDirections {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SetupIntentFlowDirections {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SetupIntentFlowDirections> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SetupIntentFlowDirections::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SetupIntentFlowDirections);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SetupIntentFlowDirections {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentFlowDirections"))
    }
}
