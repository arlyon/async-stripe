/// Toggle settings for enabling/disabling an outbound ACH specific feature
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceOutboundAchToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
    /// Whether the Feature is operational.
    pub status: stripe_shared::TreasuryFinancialAccountsResourceToggleSettingsStatus,
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
    status: Option<stripe_shared::TreasuryFinancialAccountsResourceToggleSettingsStatus>,
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
                builder:
                    TreasuryFinancialAccountsResourceOutboundAchToggleSettingsBuilder::deser_default(
                    ),
            }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceOutboundAchToggleSettingsBuilder {
        type Out = TreasuryFinancialAccountsResourceOutboundAchToggleSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "requested" => Deserialize::begin(&mut self.requested),
                "status" => Deserialize::begin(&mut self.status),
                "status_details" => Deserialize::begin(&mut self.status_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { requested: None, status: None, status_details: None }
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
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TreasuryFinancialAccountsResourceOutboundAchToggleSettings {
        type Builder = TreasuryFinancialAccountsResourceOutboundAchToggleSettingsBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceOutboundAchToggleSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TreasuryFinancialAccountsResourceOutboundAchToggleSettingsBuilder::deser_default();
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
pub type TreasuryFinancialAccountsResourceOutboundAchToggleSettingsStatus =
    stripe_shared::TreasuryFinancialAccountsResourceToggleSettingsStatus;
