/// Toggle settings for enabling/disabling a feature
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
    /// Whether the Feature is operational.
    pub status: TreasuryFinancialAccountsResourceToggleSettingsStatus,
    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details:
        Vec<stripe_treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceToggleSettingsBuilder {
    requested: Option<bool>,
    status: Option<TreasuryFinancialAccountsResourceToggleSettingsStatus>,
    status_details:
        Option<Vec<stripe_treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>>,
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

    impl Deserialize for TreasuryFinancialAccountsResourceToggleSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceToggleSettings>,
        builder: TreasuryFinancialAccountsResourceToggleSettingsBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceToggleSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceToggleSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceToggleSettingsBuilder {
        type Out = TreasuryFinancialAccountsResourceToggleSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "requested" => Deserialize::begin(&mut self.requested),
                "status" => Deserialize::begin(&mut self.status),
                "status_details" => Deserialize::begin(&mut self.status_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                requested: Deserialize::default(),
                status: Deserialize::default(),
                status_details: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(requested), Some(status), Some(status_details)) =
                (self.requested, self.status, self.status_details.take())
            else {
                return None;
            };
            Some(Self::Out { requested, status, status_details })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceToggleSettings {
        type Builder = TreasuryFinancialAccountsResourceToggleSettingsBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceToggleSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryFinancialAccountsResourceToggleSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "requested" => b.requested = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "status_details" => b.status_details = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Whether the Feature is operational.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryFinancialAccountsResourceToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}
impl TreasuryFinancialAccountsResourceToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        use TreasuryFinancialAccountsResourceToggleSettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
            Restricted => "restricted",
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceToggleSettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            "restricted" => Ok(Restricted),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryFinancialAccountsResourceToggleSettingsStatus>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryFinancialAccountsResourceToggleSettingsStatus::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryFinancialAccountsResourceToggleSettingsStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TreasuryFinancialAccountsResourceToggleSettingsStatus",
            )
        })
    }
}
