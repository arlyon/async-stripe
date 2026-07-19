#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsCard {
    /// Configuration options for setting up an eMandate for cards issued in India.
    pub mandate_options: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardMandateOptions>,
    /// Selected network to process this SetupIntent on.
    /// Depends on the available networks of the card attached to the setup intent.
    /// Can be only set confirm-time.
    pub network: Option<SetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<SetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptionsCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsCardBuilder {
    mandate_options:
        Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsCardMandateOptions>>,
    network: Option<Option<SetupIntentPaymentMethodOptionsCardNetwork>>,
    request_three_d_secure: Option<Option<SetupIntentPaymentMethodOptionsCardRequestThreeDSecure>>,
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

    impl Deserialize for SetupIntentPaymentMethodOptionsCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsCard>,
        builder: SetupIntentPaymentMethodOptionsCardBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsCardBuilder {
                    mandate_options: Deserialize::default(),
                    network: Deserialize::default(),
                    request_three_d_secure: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "mandate_options" => Deserialize::begin(&mut self.builder.mandate_options),
                "network" => Deserialize::begin(&mut self.builder.network),
                "request_three_d_secure" => {
                    Deserialize::begin(&mut self.builder.request_three_d_secure)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(mandate_options), Some(network), Some(request_three_d_secure)) = (
                self.builder.mandate_options.take(),
                self.builder.network.take(),
                self.builder.request_three_d_secure.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SetupIntentPaymentMethodOptionsCard {
                mandate_options,
                network,
                request_three_d_secure,
            });
            Ok(())
        }
    }
};
/// Selected network to process this SetupIntent on.
/// Depends on the available networks of the card attached to the setup intent.
/// Can be only set confirm-time.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SetupIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Girocard,
    Interac,
    Jcb,
    Link,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    /// This variant is prefixed with an underscore to avoid conflicts with Stripe's 'Unknown' variant.
    _Unknown(String),
}
impl SetupIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(&self) -> &str {
        use SetupIntentPaymentMethodOptionsCardNetwork::*;
        match self {
            Amex => "amex",
            CartesBancaires => "cartes_bancaires",
            Diners => "diners",
            Discover => "discover",
            EftposAu => "eftpos_au",
            Girocard => "girocard",
            Interac => "interac",
            Jcb => "jcb",
            Link => "link",
            Mastercard => "mastercard",
            Unionpay => "unionpay",
            Unknown => "unknown",
            Visa => "visa",
            _Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsCardNetwork {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsCardNetwork::*;
        match s {
            "amex" => Ok(Amex),
            "cartes_bancaires" => Ok(CartesBancaires),
            "diners" => Ok(Diners),
            "discover" => Ok(Discover),
            "eftpos_au" => Ok(EftposAu),
            "girocard" => Ok(Girocard),
            "interac" => Ok(Interac),
            "jcb" => Ok(Jcb),
            "link" => Ok(Link),
            "mastercard" => Ok(Mastercard),
            "unionpay" => Ok(Unionpay),
            "unknown" => Ok(Unknown),
            "visa" => Ok(Visa),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SetupIntentPaymentMethodOptionsCardNetwork"
                );
                Ok(_Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SetupIntentPaymentMethodOptionsCardNetwork))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupIntentPaymentMethodOptionsCardNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SetupIntentPaymentMethodOptionsCardNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<SetupIntentPaymentMethodOptionsCardNetwork> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(SetupIntentPaymentMethodOptionsCardNetwork::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SetupIntentPaymentMethodOptionsCardNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// If not provided, this value defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(&self) -> &str {
        use SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
            Challenge => "challenge",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            "challenge" => Ok(Challenge),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SetupIntentPaymentMethodOptionsCardRequestThreeDSecure"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SetupIntentPaymentMethodOptionsCardRequestThreeDSecure))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SetupIntentPaymentMethodOptionsCardRequestThreeDSecure>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
