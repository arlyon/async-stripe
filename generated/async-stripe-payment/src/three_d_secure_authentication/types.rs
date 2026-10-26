/// The Standalone 3DS API allows you to run EMV 3D Secure (3DS) authentication using Stripe while authorizing the payment with any PSP.
///
/// Related guide: [Standalone 3DS](/payments/3d-secure/standalone-3d-secure)
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureAuthentication {
    pub acquirer_details: Option<stripe_payment::ThreeDSecureStandaloneResourceAcquirerDetails>,
    /// The amount for this 3DS Authentication.
    pub amount: Option<i64>,
    /// The URL for presenting a challenge to your cardholder, present if status is requires_challenge.
    pub challenge_url: Option<String>,
    pub channel: stripe_payment::ThreeDSecureStandaloneResourceChannel,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// The 3DS directory server with which this 3DS Authentication was processed.
    pub directory_server: stripe_payment::ThreeDSecureAuthenticationDirectoryServer,
    /// The URL for performing issuer fingerprinting, present if fingerprinting is supported for the given payment method.
    pub fingerprinting_url: Option<String>,
    pub flow_preference: Option<stripe_payment::ThreeDSecureStandaloneResourceFlowPreference>,
    pub future_usage: Option<stripe_payment::ThreeDSecureStandaloneResourceFutureUsage>,
    /// Unique identifier for the object.
    pub id: stripe_payment::ThreeDSecureAuthenticationId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Indicates whether this 3DS Authentication is being performed for a payment or non-payment use case.
    pub message_category: stripe_payment::ThreeDSecureAuthenticationMessageCategory,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The outcome of this 3DS Authentication.
    pub outcome: Option<ThreeDSecureAuthenticationOutcome>,
    pub outcome_details: Option<stripe_payment::ThreeDSecureStandaloneResourceOutcomeDetails>,
    /// ID of the payment method (a PaymentMethod object) to attach to this 3DS Authentication.
    pub payment_method: stripe_types::Expandable<stripe_shared::PaymentMethod>,
    /// The reason for invoking this 3DS Authentication.
    pub reason: Option<stripe_payment::ThreeDSecureAuthenticationReason>,
    pub shipping_address: Option<stripe_payment::ThreeDSecureStandaloneResourceShippingAddress>,
    /// Status of this Authentication.
    pub status: stripe_payment::ThreeDSecureAuthenticationStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureAuthentication {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThreeDSecureAuthentication").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThreeDSecureAuthenticationBuilder {
    acquirer_details: Option<Option<stripe_payment::ThreeDSecureStandaloneResourceAcquirerDetails>>,
    amount: Option<Option<i64>>,
    challenge_url: Option<Option<String>>,
    channel: Option<stripe_payment::ThreeDSecureStandaloneResourceChannel>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<Option<stripe_types::Currency>>,
    directory_server: Option<stripe_payment::ThreeDSecureAuthenticationDirectoryServer>,
    fingerprinting_url: Option<Option<String>>,
    flow_preference: Option<Option<stripe_payment::ThreeDSecureStandaloneResourceFlowPreference>>,
    future_usage: Option<Option<stripe_payment::ThreeDSecureStandaloneResourceFutureUsage>>,
    id: Option<stripe_payment::ThreeDSecureAuthenticationId>,
    livemode: Option<bool>,
    message_category: Option<stripe_payment::ThreeDSecureAuthenticationMessageCategory>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    outcome: Option<Option<ThreeDSecureAuthenticationOutcome>>,
    outcome_details: Option<Option<stripe_payment::ThreeDSecureStandaloneResourceOutcomeDetails>>,
    payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    reason: Option<Option<stripe_payment::ThreeDSecureAuthenticationReason>>,
    shipping_address: Option<Option<stripe_payment::ThreeDSecureStandaloneResourceShippingAddress>>,
    status: Option<stripe_payment::ThreeDSecureAuthenticationStatus>,
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

    impl Deserialize for ThreeDSecureAuthentication {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureAuthentication>,
        builder: ThreeDSecureAuthenticationBuilder,
    }

    impl Visitor for Place<ThreeDSecureAuthentication> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureAuthenticationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureAuthenticationBuilder {
        type Out = ThreeDSecureAuthentication;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acquirer_details" => Deserialize::begin(&mut self.acquirer_details),
                "amount" => Deserialize::begin(&mut self.amount),
                "challenge_url" => Deserialize::begin(&mut self.challenge_url),
                "channel" => Deserialize::begin(&mut self.channel),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "directory_server" => Deserialize::begin(&mut self.directory_server),
                "fingerprinting_url" => Deserialize::begin(&mut self.fingerprinting_url),
                "flow_preference" => Deserialize::begin(&mut self.flow_preference),
                "future_usage" => Deserialize::begin(&mut self.future_usage),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "message_category" => Deserialize::begin(&mut self.message_category),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "outcome" => Deserialize::begin(&mut self.outcome),
                "outcome_details" => Deserialize::begin(&mut self.outcome_details),
                "payment_method" => Deserialize::begin(&mut self.payment_method),
                "reason" => Deserialize::begin(&mut self.reason),
                "shipping_address" => Deserialize::begin(&mut self.shipping_address),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acquirer_details: Some(None),
                amount: Some(None),
                challenge_url: Some(None),
                channel: None,
                created: None,
                currency: Some(None),
                directory_server: None,
                fingerprinting_url: Some(None),
                flow_preference: Some(None),
                future_usage: Some(None),
                id: None,
                livemode: None,
                message_category: None,
                metadata: Some(None),
                outcome: Some(None),
                outcome_details: Some(None),
                payment_method: None,
                reason: Some(None),
                shipping_address: Some(None),
                status: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(acquirer_details),
                Some(amount),
                Some(challenge_url),
                Some(channel),
                Some(created),
                Some(currency),
                Some(directory_server),
                Some(fingerprinting_url),
                Some(flow_preference),
                Some(future_usage),
                Some(id),
                Some(livemode),
                Some(message_category),
                Some(metadata),
                Some(outcome),
                Some(outcome_details),
                Some(payment_method),
                Some(reason),
                Some(shipping_address),
                Some(status),
            ) = (
                self.acquirer_details.take(),
                self.amount,
                self.challenge_url.take(),
                self.channel.take(),
                self.created,
                self.currency.take(),
                self.directory_server.take(),
                self.fingerprinting_url.take(),
                self.flow_preference.take(),
                self.future_usage.take(),
                self.id.take(),
                self.livemode,
                self.message_category.take(),
                self.metadata.take(),
                self.outcome.take(),
                self.outcome_details.take(),
                self.payment_method.take(),
                self.reason.take(),
                self.shipping_address.take(),
                self.status.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                acquirer_details,
                amount,
                challenge_url,
                channel,
                created,
                currency,
                directory_server,
                fingerprinting_url,
                flow_preference,
                future_usage,
                id,
                livemode,
                message_category,
                metadata,
                outcome,
                outcome_details,
                payment_method,
                reason,
                shipping_address,
                status,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for ThreeDSecureAuthentication {
        type Builder = ThreeDSecureAuthenticationBuilder;
    }

    impl FromValueOpt for ThreeDSecureAuthentication {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureAuthenticationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acquirer_details" => b.acquirer_details = FromValueOpt::from_value(v),
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "challenge_url" => b.challenge_url = FromValueOpt::from_value(v),
                    "channel" => b.channel = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "directory_server" => b.directory_server = FromValueOpt::from_value(v),
                    "fingerprinting_url" => b.fingerprinting_url = FromValueOpt::from_value(v),
                    "flow_preference" => b.flow_preference = FromValueOpt::from_value(v),
                    "future_usage" => b.future_usage = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "message_category" => b.message_category = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "outcome" => b.outcome = FromValueOpt::from_value(v),
                    "outcome_details" => b.outcome_details = FromValueOpt::from_value(v),
                    "payment_method" => b.payment_method = FromValueOpt::from_value(v),
                    "reason" => b.reason = FromValueOpt::from_value(v),
                    "shipping_address" => b.shipping_address = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureAuthentication {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ThreeDSecureAuthentication", 21)?;
        s.serialize_field("acquirer_details", &self.acquirer_details)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("challenge_url", &self.challenge_url)?;
        s.serialize_field("channel", &self.channel)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("directory_server", &self.directory_server)?;
        s.serialize_field("fingerprinting_url", &self.fingerprinting_url)?;
        s.serialize_field("flow_preference", &self.flow_preference)?;
        s.serialize_field("future_usage", &self.future_usage)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("message_category", &self.message_category)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("outcome", &self.outcome)?;
        s.serialize_field("outcome_details", &self.outcome_details)?;
        s.serialize_field("payment_method", &self.payment_method)?;
        s.serialize_field("reason", &self.reason)?;
        s.serialize_field("shipping_address", &self.shipping_address)?;
        s.serialize_field("status", &self.status)?;

        s.serialize_field("object", "three_d_secure.authentication")?;
        s.end()
    }
}
/// The outcome of this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureAuthenticationOutcome {
    Abandoned,
    AttemptAcknowledged,
    Authenticated,
    Canceled,
    Denied,
    Informational,
    InternalError,
    NotSupported,
    NotTriggered,
    ProcessingError,
    Rejected,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureAuthenticationOutcome {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureAuthenticationOutcome::*;
        match self {
            Abandoned => "abandoned",
            AttemptAcknowledged => "attempt_acknowledged",
            Authenticated => "authenticated",
            Canceled => "canceled",
            Denied => "denied",
            Informational => "informational",
            InternalError => "internal_error",
            NotSupported => "not_supported",
            NotTriggered => "not_triggered",
            ProcessingError => "processing_error",
            Rejected => "rejected",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureAuthenticationOutcome {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureAuthenticationOutcome::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "attempt_acknowledged" => Ok(AttemptAcknowledged),
            "authenticated" => Ok(Authenticated),
            "canceled" => Ok(Canceled),
            "denied" => Ok(Denied),
            "informational" => Ok(Informational),
            "internal_error" => Ok(InternalError),
            "not_supported" => Ok(NotSupported),
            "not_triggered" => Ok(NotTriggered),
            "processing_error" => Ok(ProcessingError),
            "rejected" => Ok(Rejected),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureAuthenticationOutcome"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureAuthenticationOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureAuthenticationOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureAuthenticationOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureAuthenticationOutcome)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureAuthenticationOutcome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureAuthenticationOutcome {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureAuthenticationOutcome> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureAuthenticationOutcome::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureAuthenticationOutcome);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureAuthenticationOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for ThreeDSecureAuthentication {
    type Id = stripe_payment::ThreeDSecureAuthenticationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ThreeDSecureAuthenticationId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureAuthenticationDirectoryServer {
    AmericanExpress,
    CartesBancaires,
    Discover,
    Mastercard,
    Visa,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureAuthenticationDirectoryServer {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureAuthenticationDirectoryServer::*;
        match self {
            AmericanExpress => "american_express",
            CartesBancaires => "cartes_bancaires",
            Discover => "discover",
            Mastercard => "mastercard",
            Visa => "visa",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureAuthenticationDirectoryServer {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureAuthenticationDirectoryServer::*;
        match s {
            "american_express" => Ok(AmericanExpress),
            "cartes_bancaires" => Ok(CartesBancaires),
            "discover" => Ok(Discover),
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureAuthenticationDirectoryServer"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureAuthenticationDirectoryServer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureAuthenticationDirectoryServer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureAuthenticationDirectoryServer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureAuthenticationDirectoryServer))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for ThreeDSecureAuthenticationDirectoryServer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureAuthenticationDirectoryServer {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureAuthenticationDirectoryServer> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(ThreeDSecureAuthenticationDirectoryServer::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureAuthenticationDirectoryServer);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureAuthenticationDirectoryServer {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureAuthenticationMessageCategory {
    NonPaymentAuthentication,
    PaymentAuthentication,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureAuthenticationMessageCategory {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureAuthenticationMessageCategory::*;
        match self {
            NonPaymentAuthentication => "non_payment_authentication",
            PaymentAuthentication => "payment_authentication",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureAuthenticationMessageCategory {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureAuthenticationMessageCategory::*;
        match s {
            "non_payment_authentication" => Ok(NonPaymentAuthentication),
            "payment_authentication" => Ok(PaymentAuthentication),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureAuthenticationMessageCategory"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureAuthenticationMessageCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureAuthenticationMessageCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureAuthenticationMessageCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureAuthenticationMessageCategory))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for ThreeDSecureAuthenticationMessageCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureAuthenticationMessageCategory {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureAuthenticationMessageCategory> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(ThreeDSecureAuthenticationMessageCategory::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureAuthenticationMessageCategory);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureAuthenticationMessageCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureAuthenticationReason {
    CardholderAuthentication,
    IssuerRequested,
    LiabilityShift,
    ProcessingCosts,
    RegulatoryCompliance,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureAuthenticationReason {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureAuthenticationReason::*;
        match self {
            CardholderAuthentication => "cardholder_authentication",
            IssuerRequested => "issuer_requested",
            LiabilityShift => "liability_shift",
            ProcessingCosts => "processing_costs",
            RegulatoryCompliance => "regulatory_compliance",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureAuthenticationReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureAuthenticationReason::*;
        match s {
            "cardholder_authentication" => Ok(CardholderAuthentication),
            "issuer_requested" => Ok(IssuerRequested),
            "liability_shift" => Ok(LiabilityShift),
            "processing_costs" => Ok(ProcessingCosts),
            "regulatory_compliance" => Ok(RegulatoryCompliance),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureAuthenticationReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureAuthenticationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureAuthenticationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureAuthenticationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureAuthenticationReason)).finish_non_exhaustive()
    }
}
impl serde::Serialize for ThreeDSecureAuthenticationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureAuthenticationReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureAuthenticationReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureAuthenticationReason::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureAuthenticationReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureAuthenticationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureAuthenticationStatus {
    Canceled,
    Error,
    Failed,
    RequiresChallenge,
    RequiresSubmission,
    Succeeded,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureAuthenticationStatus {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureAuthenticationStatus::*;
        match self {
            Canceled => "canceled",
            Error => "error",
            Failed => "failed",
            RequiresChallenge => "requires_challenge",
            RequiresSubmission => "requires_submission",
            Succeeded => "succeeded",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureAuthenticationStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureAuthenticationStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "error" => Ok(Error),
            "failed" => Ok(Failed),
            "requires_challenge" => Ok(RequiresChallenge),
            "requires_submission" => Ok(RequiresSubmission),
            "succeeded" => Ok(Succeeded),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureAuthenticationStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureAuthenticationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureAuthenticationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureAuthenticationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureAuthenticationStatus)).finish_non_exhaustive()
    }
}
impl serde::Serialize for ThreeDSecureAuthenticationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureAuthenticationStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureAuthenticationStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureAuthenticationStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureAuthenticationStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureAuthenticationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
