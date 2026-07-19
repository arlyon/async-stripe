#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsUsBankAccount {
    pub financial_connections: Option<stripe_shared::LinkedAccountOptionsCommon>,
    pub mandate_options: Option<stripe_shared::PaymentMethodOptionsUsBankAccountMandateOptions>,
    /// Bank account verification method. The default value is `automatic`.
    pub verification_method: Option<SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptionsUsBankAccount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsUsBankAccountBuilder {
    financial_connections: Option<Option<stripe_shared::LinkedAccountOptionsCommon>>,
    mandate_options: Option<Option<stripe_shared::PaymentMethodOptionsUsBankAccountMandateOptions>>,
    verification_method:
        Option<Option<SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>>,
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

    impl Deserialize for SetupIntentPaymentMethodOptionsUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsUsBankAccount>,
        builder: SetupIntentPaymentMethodOptionsUsBankAccountBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsUsBankAccountBuilder {
                    financial_connections: Deserialize::default(),
                    mandate_options: Deserialize::default(),
                    verification_method: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "financial_connections" => {
                    Deserialize::begin(&mut self.builder.financial_connections)
                }
                "mandate_options" => Deserialize::begin(&mut self.builder.mandate_options),
                "verification_method" => Deserialize::begin(&mut self.builder.verification_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(financial_connections), Some(mandate_options), Some(verification_method)) = (
                self.builder.financial_connections.take(),
                self.builder.mandate_options.take(),
                self.builder.verification_method.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SetupIntentPaymentMethodOptionsUsBankAccount {
                financial_connections,
                mandate_options,
                verification_method,
            });
            Ok(())
        }
    }
};
/// Bank account verification method. The default value is `automatic`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(&self) -> &str {
        use SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
