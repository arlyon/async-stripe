#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
    /// If Stripe disabled automatic tax, this enum describes why.
    pub disabled_reason:
        Option<SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason>,
    /// Whether Stripe automatically computes tax on invoices created during this phase.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    pub liability: Option<stripe_shared::ConnectAccountReference>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedulesResourceDefaultSettingsAutomaticTax")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder {
    disabled_reason:
        Option<Option<SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason>>,
    enabled: Option<bool>,
    liability: Option<Option<stripe_shared::ConnectAccountReference>>,
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

    impl Deserialize for SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourceDefaultSettingsAutomaticTax>,
        builder: SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourceDefaultSettingsAutomaticTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder {
                    disabled_reason: Deserialize::default(),
                    enabled: Deserialize::default(),
                    liability: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "disabled_reason" => Deserialize::begin(&mut self.builder.disabled_reason),
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "liability" => Deserialize::begin(&mut self.builder.liability),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(disabled_reason), Some(enabled), Some(liability)) = (
                self.builder.disabled_reason.take(),
                self.builder.enabled,
                self.builder.liability.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
                disabled_reason,
                enabled,
                liability,
            });
            Ok(())
        }
    }
};
/// If Stripe disabled automatic tax, this enum describes why.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason {
    RequiresLocationInputs,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason {
    pub fn as_str(&self) -> &str {
        use SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason::*;
        match self {
            RequiresLocationInputs => "requires_location_inputs",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason::*;
        match s {
            "requires_location_inputs" => Ok(RequiresLocationInputs),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxDisabledReason
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
