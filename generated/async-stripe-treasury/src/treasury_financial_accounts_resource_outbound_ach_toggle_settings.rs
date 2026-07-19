/// Toggle settings for enabling/disabling an outbound ACH specific feature
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceOutboundAchToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
    /// Whether the Feature is operational.
    pub status: TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus,
    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details:
        Vec<stripe_treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountsResourceOutboundAchToggleSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryFinancialAccountsResourceOutboundAchToggleSettings")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceOutboundAchToggleSettingsBuilder {
    requested: Option<bool>,
    status: Option<TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus>,
    status_details:
        Option<Vec<stripe_treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>>,
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

    impl Deserialize for TreasuryFinancialAccountsResourceOutboundAchToggleSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceOutboundAchToggleSettings>,
        builder: TreasuryFinancialAccountsResourceOutboundAchToggleSettingsBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceOutboundAchToggleSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryFinancialAccountsResourceOutboundAchToggleSettingsBuilder {
                    requested: Deserialize::default(),
                    status: Deserialize::default(),
                    status_details: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "requested" => Deserialize::begin(&mut self.builder.requested),
                "status" => Deserialize::begin(&mut self.builder.status),
                "status_details" => Deserialize::begin(&mut self.builder.status_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(requested), Some(status), Some(status_details)) = (
                self.builder.requested,
                self.builder.status.take(),
                self.builder.status_details.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(TreasuryFinancialAccountsResourceOutboundAchToggleSettings {
                requested,
                status,
                status_details,
            });
            Ok(())
        }
    }
};
/// Whether the Feature is operational.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
    pub fn as_str(&self) -> &str {
        use TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus::*;
        match self {
            Active => "active",
            Pending => "pending",
            Restricted => "restricted",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus::*;
        match s {
            "active" => Ok(Active),
            "pending" => Ok(Pending),
            "restricted" => Ok(Restricted),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
