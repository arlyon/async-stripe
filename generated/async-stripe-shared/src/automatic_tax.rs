#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AutomaticTax {
    /// If Stripe disabled automatic tax, this enum describes why.
    pub disabled_reason: Option<AutomaticTaxDisabledReason>,
    /// Whether Stripe automatically computes tax on this invoice.
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    pub liability: Option<stripe_shared::ConnectAccountReference>,
    /// The tax provider powering automatic tax.
    pub provider: Option<String>,
    /// The status of the most recent automated tax calculation for this invoice.
    pub status: Option<AutomaticTaxStatus>,
}
#[doc(hidden)]
pub struct AutomaticTaxBuilder {
    disabled_reason: Option<Option<AutomaticTaxDisabledReason>>,
    enabled: Option<bool>,
    liability: Option<Option<stripe_shared::ConnectAccountReference>>,
    provider: Option<Option<String>>,
    status: Option<Option<AutomaticTaxStatus>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AutomaticTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AutomaticTax>,
        builder: AutomaticTaxBuilder,
    }

    impl Visitor for Place<AutomaticTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AutomaticTaxBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AutomaticTaxBuilder {
        type Out = AutomaticTax;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "disabled_reason" => Deserialize::begin(&mut self.disabled_reason),
                "enabled" => Deserialize::begin(&mut self.enabled),
                "liability" => Deserialize::begin(&mut self.liability),
                "provider" => Deserialize::begin(&mut self.provider),
                "status" => Deserialize::begin(&mut self.status),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                disabled_reason: Deserialize::default(),
                enabled: Deserialize::default(),
                liability: Deserialize::default(),
                provider: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(disabled_reason),
                Some(enabled),
                Some(liability),
                Some(provider),
                Some(status),
            ) = (
                self.disabled_reason,
                self.enabled,
                self.liability.take(),
                self.provider.take(),
                self.status,
            )
            else {
                return None;
            };
            Some(Self::Out { disabled_reason, enabled, liability, provider, status })
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

    impl ObjectDeser for AutomaticTax {
        type Builder = AutomaticTaxBuilder;
    }

    impl FromValueOpt for AutomaticTax {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AutomaticTaxBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "disabled_reason" => b.disabled_reason = FromValueOpt::from_value(v),
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "liability" => b.liability = FromValueOpt::from_value(v),
                    "provider" => b.provider = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// If Stripe disabled automatic tax, this enum describes why.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AutomaticTaxDisabledReason {
    FinalizationRequiresLocationInputs,
    FinalizationSystemError,
}
impl AutomaticTaxDisabledReason {
    pub fn as_str(self) -> &'static str {
        use AutomaticTaxDisabledReason::*;
        match self {
            FinalizationRequiresLocationInputs => "finalization_requires_location_inputs",
            FinalizationSystemError => "finalization_system_error",
        }
    }
}

impl std::str::FromStr for AutomaticTaxDisabledReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AutomaticTaxDisabledReason::*;
        match s {
            "finalization_requires_location_inputs" => Ok(FinalizationRequiresLocationInputs),
            "finalization_system_error" => Ok(FinalizationSystemError),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for AutomaticTaxDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AutomaticTaxDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AutomaticTaxDisabledReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AutomaticTaxDisabledReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AutomaticTaxDisabledReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AutomaticTaxDisabledReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AutomaticTaxDisabledReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AutomaticTaxDisabledReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AutomaticTaxDisabledReason"))
    }
}
/// The status of the most recent automated tax calculation for this invoice.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}
impl AutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        use AutomaticTaxStatus::*;
        match self {
            Complete => "complete",
            Failed => "failed",
            RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl std::str::FromStr for AutomaticTaxStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AutomaticTaxStatus::*;
        match s {
            "complete" => Ok(Complete),
            "failed" => Ok(Failed),
            "requires_location_inputs" => Ok(RequiresLocationInputs),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for AutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AutomaticTaxStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AutomaticTaxStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AutomaticTaxStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AutomaticTaxStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AutomaticTaxStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AutomaticTaxStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AutomaticTaxStatus"))
    }
}
