/// You can use Tax `Settings` to manage configurations used by Stripe Tax calculations.
///
/// Related guide: [Using the Settings API](https://stripe.com/docs/tax/settings-api)
///
/// For more details see <<https://stripe.com/docs/api/tax/settings/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxSettings {
    pub defaults: stripe_misc::TaxProductResourceTaxSettingsDefaults,
    /// The place where your business is located.
    pub head_office: Option<stripe_misc::TaxProductResourceTaxSettingsHeadOffice>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The `active` status indicates you have all required settings to calculate tax.
    /// A status can transition out of `active` when new required settings are introduced.
    pub status: TaxSettingsStatus,
    pub status_details: stripe_misc::TaxProductResourceTaxSettingsStatusDetails,
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
                builder: TaxSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxSettingsBuilder {
        type Out = TaxSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "defaults" => Deserialize::begin(&mut self.defaults),
                "head_office" => Deserialize::begin(&mut self.head_office),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "status" => Deserialize::begin(&mut self.status),
                "status_details" => Deserialize::begin(&mut self.status_details),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                defaults: Deserialize::default(),
                head_office: Deserialize::default(),
                livemode: Deserialize::default(),
                status: Deserialize::default(),
                status_details: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(defaults),
                Some(head_office),
                Some(livemode),
                Some(status),
                Some(status_details),
            ) = (
                self.defaults.take(),
                self.head_office.take(),
                self.livemode,
                self.status,
                self.status_details.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { defaults, head_office, livemode, status, status_details })
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

    impl ObjectDeser for TaxSettings {
        type Builder = TaxSettingsBuilder;
    }

    impl FromValueOpt for TaxSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "defaults" => b.defaults = FromValueOpt::from_value(v),
                    "head_office" => b.head_office = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "status_details" => b.status_details = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
/// The `active` status indicates you have all required settings to calculate tax.
/// A status can transition out of `active` when new required settings are introduced.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxSettingsStatus {
    Active,
    Pending,
}
impl TaxSettingsStatus {
    pub fn as_str(self) -> &'static str {
        use TaxSettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for TaxSettingsStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxSettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for TaxSettingsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxSettingsStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxSettingsStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxSettingsStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxSettingsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxSettingsStatus"))
    }
}
