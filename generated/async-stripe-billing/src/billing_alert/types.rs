/// A billing alert is a resource that notifies you when a certain usage threshold on a meter is crossed.
/// For example, you might create a billing alert to notify you when a certain user made 100 API requests.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingAlert {
    /// Defines the type of the alert.
    pub alert_type: stripe_billing::BillingAlertAlertType,
    /// Unique identifier for the object.
    pub id: stripe_billing::BillingAlertId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Status of the alert. This can be active, inactive or archived.
    pub status: Option<BillingAlertStatus>,
    /// Title of the alert.
    pub title: String,
    /// Encapsulates configuration of the alert to monitor usage on a specific [Billing Meter](https://stripe.com/docs/api/billing/meter).
    pub usage_threshold: Option<stripe_billing::ThresholdsResourceUsageThresholdConfig>,
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
                builder: BillingAlertBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingAlertBuilder {
        type Out = BillingAlert;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alert_type" => Deserialize::begin(&mut self.alert_type),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "status" => Deserialize::begin(&mut self.status),
                "title" => Deserialize::begin(&mut self.title),
                "usage_threshold" => Deserialize::begin(&mut self.usage_threshold),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                alert_type: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                status: Deserialize::default(),
                title: Deserialize::default(),
                usage_threshold: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(alert_type),
                Some(id),
                Some(livemode),
                Some(status),
                Some(title),
                Some(usage_threshold),
            ) = (
                self.alert_type,
                self.id.take(),
                self.livemode,
                self.status,
                self.title.take(),
                self.usage_threshold.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { alert_type, id, livemode, status, title, usage_threshold })
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

    impl ObjectDeser for BillingAlert {
        type Builder = BillingAlertBuilder;
    }

    impl FromValueOpt for BillingAlert {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingAlertBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "alert_type" => b.alert_type = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "title" => b.title = FromValueOpt::from_value(v),
                    "usage_threshold" => b.usage_threshold = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingAlertStatus {
    Active,
    Archived,
    Inactive,
}
impl BillingAlertStatus {
    pub fn as_str(self) -> &'static str {
        use BillingAlertStatus::*;
        match self {
            Active => "active",
            Archived => "archived",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for BillingAlertStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingAlertStatus::*;
        match s {
            "active" => Ok(Active),
            "archived" => Ok(Archived),
            "inactive" => Ok(Inactive),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingAlertStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingAlertStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for BillingAlertStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingAlertStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingAlertStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingAlertStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingAlertStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BillingAlertStatus"))
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingAlertAlertType {
    UsageThreshold,
}
impl BillingAlertAlertType {
    pub fn as_str(self) -> &'static str {
        use BillingAlertAlertType::*;
        match self {
            UsageThreshold => "usage_threshold",
        }
    }
}

impl std::str::FromStr for BillingAlertAlertType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingAlertAlertType::*;
        match s {
            "usage_threshold" => Ok(UsageThreshold),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingAlertAlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingAlertAlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for BillingAlertAlertType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingAlertAlertType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingAlertAlertType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingAlertAlertType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingAlertAlertType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BillingAlertAlertType"))
    }
}
