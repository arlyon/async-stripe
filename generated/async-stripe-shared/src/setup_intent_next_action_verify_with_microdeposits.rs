#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentNextActionVerifyWithMicrodeposits {
    /// The timestamp when the microdeposits are expected to land.
    pub arrival_date: stripe_types::Timestamp,
    /// The URL for the hosted verification page, which allows customers to verify their bank account.
    pub hosted_verification_url: String,
    /// The type of the microdeposit sent to the customer.
    /// Used to distinguish between different verification methods.
    pub microdeposit_type: Option<SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentNextActionVerifyWithMicrodeposits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentNextActionVerifyWithMicrodeposits").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentNextActionVerifyWithMicrodepositsBuilder {
    arrival_date: Option<stripe_types::Timestamp>,
    hosted_verification_url: Option<String>,
    microdeposit_type: Option<Option<SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType>>,
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

    impl Deserialize for SetupIntentNextActionVerifyWithMicrodeposits {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentNextActionVerifyWithMicrodeposits>,
        builder: SetupIntentNextActionVerifyWithMicrodepositsBuilder,
    }

    impl Visitor for Place<SetupIntentNextActionVerifyWithMicrodeposits> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentNextActionVerifyWithMicrodepositsBuilder {
                    arrival_date: Deserialize::default(),
                    hosted_verification_url: Deserialize::default(),
                    microdeposit_type: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "arrival_date" => Deserialize::begin(&mut self.builder.arrival_date),
                "hosted_verification_url" => {
                    Deserialize::begin(&mut self.builder.hosted_verification_url)
                }
                "microdeposit_type" => Deserialize::begin(&mut self.builder.microdeposit_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(arrival_date), Some(hosted_verification_url), Some(microdeposit_type)) = (
                self.builder.arrival_date,
                self.builder.hosted_verification_url.take(),
                self.builder.microdeposit_type.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SetupIntentNextActionVerifyWithMicrodeposits {
                arrival_date,
                hosted_verification_url,
                microdeposit_type,
            });
            Ok(())
        }
    }
};
/// The type of the microdeposit sent to the customer.
/// Used to distinguish between different verification methods.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    Amounts,
    DescriptorCode,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    pub fn as_str(&self) -> &str {
        use SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::*;
        match self {
            Amounts => "amounts",
            DescriptorCode => "descriptor_code",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::*;
        match s {
            "amounts" => Ok(Amounts),
            "descriptor_code" => Ok(DescriptorCode),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
