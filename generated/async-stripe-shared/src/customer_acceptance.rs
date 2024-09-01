#[derive(Clone, Debug)]
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
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: CustomerAcceptanceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CustomerAcceptanceBuilder {
        type Out = CustomerAcceptance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "accepted_at" => Deserialize::begin(&mut self.accepted_at),
                "offline" => Deserialize::begin(&mut self.offline),
                "online" => Deserialize::begin(&mut self.online),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                accepted_at: Deserialize::default(),
                offline: Deserialize::default(),
                online: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(accepted_at), Some(offline), Some(online), Some(type_)) =
                (self.accepted_at, self.offline, self.online.take(), self.type_)
            else {
                return None;
            };
            Some(Self::Out { accepted_at, offline, online, type_ })
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

    impl ObjectDeser for CustomerAcceptance {
        type Builder = CustomerAcceptanceBuilder;
    }

    impl FromValueOpt for CustomerAcceptance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerAcceptanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "accepted_at" => b.accepted_at = FromValueOpt::from_value(v),
                    "offline" => b.offline = FromValueOpt::from_value(v),
                    "online" => b.online = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The mandate includes the type of customer acceptance information, such as: `online` or `offline`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerAcceptanceType {
    Offline,
    Online,
}
impl CustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        use CustomerAcceptanceType::*;
        match self {
            Offline => "offline",
            Online => "online",
        }
    }
}

impl std::str::FromStr for CustomerAcceptanceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerAcceptanceType::*;
        match s {
            "offline" => Ok(Offline),
            "online" => Ok(Online),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for CustomerAcceptanceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerAcceptanceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerAcceptanceType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CustomerAcceptanceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerAcceptanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CustomerAcceptanceType"))
    }
}
