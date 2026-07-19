/// A billing alert is a resource that notifies you when a certain usage threshold on a meter is crossed.
/// For example, you might create a billing alert to notify you when a certain user made 100 API requests.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingAlert {
    /// Defines the type of the alert.
    pub alert_type: stripe_billing::BillingAlertAlertType,
    /// Unique identifier for the object.
    pub id: stripe_billing::BillingAlertId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Status of the alert. This can be active, inactive or archived.
    pub status: Option<BillingAlertStatus>,
    /// Title of the alert.
    pub title: String,
    /// Encapsulates configuration of the alert to monitor usage on a specific [Billing Meter](https://docs.stripe.com/api/billing/meter).
    pub usage_threshold: Option<stripe_billing::ThresholdsResourceUsageThresholdConfig>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingAlert {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingAlert").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingAlertBuilder {
    alert_type: Option<stripe_billing::BillingAlertAlertType>,
    id: Option<stripe_billing::BillingAlertId>,
    livemode: Option<bool>,
    status: Option<Option<BillingAlertStatus>>,
    title: Option<String>,
    usage_threshold: Option<Option<stripe_billing::ThresholdsResourceUsageThresholdConfig>>,
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

    impl Deserialize for BillingAlert {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingAlert>,
        builder: BillingAlertBuilder,
    }

    impl Visitor for Place<BillingAlert> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingAlertBuilder {
                    alert_type: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    status: Deserialize::default(),
                    title: Deserialize::default(),
                    usage_threshold: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alert_type" => Deserialize::begin(&mut self.builder.alert_type),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "status" => Deserialize::begin(&mut self.builder.status),
                "title" => Deserialize::begin(&mut self.builder.title),
                "usage_threshold" => Deserialize::begin(&mut self.builder.usage_threshold),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(alert_type),
                Some(id),
                Some(livemode),
                Some(status),
                Some(title),
                Some(usage_threshold),
            ) = (
                self.builder.alert_type.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.status.take(),
                self.builder.title.take(),
                self.builder.usage_threshold.take(),
            )
            else {
                return Ok(());
            };
            *self.out =
                Some(BillingAlert { alert_type, id, livemode, status, title, usage_threshold });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingAlert {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingAlert", 7)?;
        s.serialize_field("alert_type", &self.alert_type)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("usage_threshold", &self.usage_threshold)?;

        s.serialize_field("object", "billing.alert")?;
        s.end()
    }
}
/// Status of the alert. This can be active, inactive or archived.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingAlertStatus {
    Active,
    Archived,
    Inactive,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingAlertStatus {
    pub fn as_str(&self) -> &str {
        use BillingAlertStatus::*;
        match self {
            Active => "active",
            Archived => "archived",
            Inactive => "inactive",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingAlertStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingAlertStatus::*;
        match s {
            "active" => Ok(Active),
            "archived" => Ok(Archived),
            "inactive" => Ok(Inactive),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "BillingAlertStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingAlertStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingAlertStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingAlertStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingAlertStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingAlertStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for BillingAlertStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingAlertStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingAlertStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingAlertStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for BillingAlert {
    type Id = stripe_billing::BillingAlertId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(BillingAlertId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingAlertAlertType {
    UsageThreshold,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingAlertAlertType {
    pub fn as_str(&self) -> &str {
        use BillingAlertAlertType::*;
        match self {
            UsageThreshold => "usage_threshold",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingAlertAlertType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingAlertAlertType::*;
        match s {
            "usage_threshold" => Ok(UsageThreshold),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "BillingAlertAlertType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingAlertAlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingAlertAlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingAlertAlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingAlertAlertType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for BillingAlertAlertType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for BillingAlertAlertType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingAlertAlertType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingAlertAlertType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingAlertAlertType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
