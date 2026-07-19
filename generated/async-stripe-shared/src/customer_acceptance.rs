#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerAcceptance {
    /// The time that the customer accepts the mandate.
    pub accepted_at: Option<stripe_types::Timestamp>,
    pub offline: Option<stripe_shared::OfflineAcceptance>,
    pub online: Option<stripe_shared::OnlineAcceptance>,
    /// The mandate includes the type of customer acceptance information, such as: `online` or `offline`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: CustomerAcceptanceType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerAcceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerAcceptance").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerAcceptanceBuilder {
    accepted_at: Option<Option<stripe_types::Timestamp>>,
    offline: Option<Option<stripe_shared::OfflineAcceptance>>,
    online: Option<Option<stripe_shared::OnlineAcceptance>>,
    type_: Option<CustomerAcceptanceType>,
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

    impl Deserialize for CustomerAcceptance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerAcceptance>,
        builder: CustomerAcceptanceBuilder,
    }

    impl Visitor for Place<CustomerAcceptance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerAcceptanceBuilder {
                    accepted_at: Deserialize::default(),
                    offline: Deserialize::default(),
                    online: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "accepted_at" => Deserialize::begin(&mut self.builder.accepted_at),
                "offline" => Deserialize::begin(&mut self.builder.offline),
                "online" => Deserialize::begin(&mut self.builder.online),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(accepted_at), Some(offline), Some(online), Some(type_)) = (
                self.builder.accepted_at,
                self.builder.offline,
                self.builder.online.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(CustomerAcceptance { accepted_at, offline, online, type_ });
            Ok(())
        }
    }
};
/// The mandate includes the type of customer acceptance information, such as: `online` or `offline`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CustomerAcceptanceType {
    Offline,
    Online,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CustomerAcceptanceType {
    pub fn as_str(&self) -> &str {
        use CustomerAcceptanceType::*;
        match self {
            Offline => "offline",
            Online => "online",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CustomerAcceptanceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerAcceptanceType::*;
        match s {
            "offline" => Ok(Offline),
            "online" => Ok(Online),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CustomerAcceptanceType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CustomerAcceptanceType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerAcceptanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for CustomerAcceptanceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CustomerAcceptanceType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerAcceptanceType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerAcceptanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
