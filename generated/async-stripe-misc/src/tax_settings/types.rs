/// You can use Tax `Settings` to manage configurations used by Stripe Tax calculations.
///
/// Related guide: [Using the Settings API](https://docs.stripe.com/tax/settings-api)
///
/// For more details see <<https://stripe.com/docs/api/tax/settings/object>>.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxSettings {
    pub defaults: stripe_misc::TaxProductResourceTaxSettingsDefaults,
    /// The place where your business is located.
    pub head_office: Option<stripe_misc::TaxProductResourceTaxSettingsHeadOffice>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The status of the Tax `Settings`.
    pub status: TaxSettingsStatus,
    pub status_details: stripe_misc::TaxProductResourceTaxSettingsStatusDetails,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxSettingsBuilder {
    defaults: Option<stripe_misc::TaxProductResourceTaxSettingsDefaults>,
    head_office: Option<Option<stripe_misc::TaxProductResourceTaxSettingsHeadOffice>>,
    livemode: Option<bool>,
    status: Option<TaxSettingsStatus>,
    status_details: Option<stripe_misc::TaxProductResourceTaxSettingsStatusDetails>,
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

    impl Deserialize for TaxSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxSettings>,
        builder: TaxSettingsBuilder,
    }

    impl Visitor for Place<TaxSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxSettingsBuilder {
                    defaults: Deserialize::default(),
                    head_office: Deserialize::default(),
                    livemode: Deserialize::default(),
                    status: Deserialize::default(),
                    status_details: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "defaults" => Deserialize::begin(&mut self.builder.defaults),
                "head_office" => Deserialize::begin(&mut self.builder.head_office),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "status" => Deserialize::begin(&mut self.builder.status),
                "status_details" => Deserialize::begin(&mut self.builder.status_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(defaults),
                Some(head_office),
                Some(livemode),
                Some(status),
                Some(status_details),
            ) = (
                self.builder.defaults.take(),
                self.builder.head_office.take(),
                self.builder.livemode,
                self.builder.status.take(),
                self.builder.status_details.take(),
            )
            else {
                return Ok(());
            };
            *self.out =
                Some(TaxSettings { defaults, head_office, livemode, status, status_details });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxSettings {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxSettings", 6)?;
        s.serialize_field("defaults", &self.defaults)?;
        s.serialize_field("head_office", &self.head_office)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_details", &self.status_details)?;

        s.serialize_field("object", "tax.settings")?;
        s.end()
    }
}
/// The status of the Tax `Settings`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxSettingsStatus {
    Active,
    Pending,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxSettingsStatus {
    pub fn as_str(&self) -> &str {
        use TaxSettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxSettingsStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxSettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "TaxSettingsStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxSettingsStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TaxSettingsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TaxSettingsStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxSettingsStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxSettingsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
