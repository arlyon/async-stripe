/// Toggle settings for enabling/disabling an inbound ACH specific feature
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceInboundAchToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
    /// Whether the Feature is operational.
    pub status: TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus,
    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details:
        Vec<stripe_treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceInboundAchToggleSettingsBuilder {
    requested: Option<bool>,
    status: Option<TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus>,
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

    impl Deserialize for TreasuryFinancialAccountsResourceInboundAchToggleSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceInboundAchToggleSettings>,
        builder: TreasuryFinancialAccountsResourceInboundAchToggleSettingsBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceInboundAchToggleSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TreasuryFinancialAccountsResourceInboundAchToggleSettingsBuilder::deser_default(
                    ),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceInboundAchToggleSettingsBuilder {
        type Out = TreasuryFinancialAccountsResourceInboundAchToggleSettings;
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
                (self.requested, self.status.take(), self.status_details.take())
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceInboundAchToggleSettings {
        type Builder = TreasuryFinancialAccountsResourceInboundAchToggleSettingsBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceInboundAchToggleSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TreasuryFinancialAccountsResourceInboundAchToggleSettingsBuilder::deser_default();
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus {
    pub fn as_str(&self) -> &str {
        use TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
            Restricted => "restricted",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            "restricted" => Ok(Restricted),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryFinancialAccountsResourceInboundAchToggleSettingsStatus
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
