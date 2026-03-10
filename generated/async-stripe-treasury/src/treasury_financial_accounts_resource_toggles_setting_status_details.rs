/// Additional details on the FinancialAccount Features information.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {
    /// Represents the reason why the status is `pending` or `restricted`.
    pub code: TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode,
    /// Represents what the user should do, if anything, to activate the Feature.
    pub resolution: Option<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution>,
    /// The `platform_restrictions` that are restricting this Feature.
    pub restriction:
        Option<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction>,
}
#[doc(hidden)]
pub struct TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsBuilder {
    code: Option<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode>,
    resolution:
        Option<Option<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution>>,
    restriction:
        Option<Option<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction>>,
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

    impl Deserialize for TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
        builder: TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsBuilder,
    }

    impl Visitor for Place<TreasuryFinancialAccountsResourceTogglesSettingStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsBuilder {
        type Out = TreasuryFinancialAccountsResourceTogglesSettingStatusDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.code),
                "resolution" => Deserialize::begin(&mut self.resolution),
                "restriction" => Deserialize::begin(&mut self.restriction),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                code: Deserialize::default(),
                resolution: Deserialize::default(),
                restriction: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(code), Some(resolution), Some(restriction)) =
                (self.code.take(), self.resolution.take(), self.restriction.take())
            else {
                return None;
            };
            Some(Self::Out { code, resolution, restriction })
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

    impl ObjectDeser for TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {
        type Builder = TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsBuilder;
    }

    impl FromValueOpt for TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsBuilder::deser_default(
                );
            for (k, v) in obj {
                match k.as_str() {
                    "code" => b.code = FromValueOpt::from_value(v),
                    "resolution" => b.resolution = FromValueOpt::from_value(v),
                    "restriction" => b.restriction = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Represents the reason why the status is `pending` or `restricted`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    Activating,
    CapabilityNotRequested,
    FinancialAccountClosed,
    RejectedOther,
    RejectedUnsupportedBusiness,
    RequirementsPastDue,
    RequirementsPendingVerification,
    RestrictedByPlatform,
    RestrictedOther,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    pub fn as_str(&self) -> &str {
        use TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::*;
        match self {
            Activating => "activating",
            CapabilityNotRequested => "capability_not_requested",
            FinancialAccountClosed => "financial_account_closed",
            RejectedOther => "rejected_other",
            RejectedUnsupportedBusiness => "rejected_unsupported_business",
            RequirementsPastDue => "requirements_past_due",
            RequirementsPendingVerification => "requirements_pending_verification",
            RestrictedByPlatform => "restricted_by_platform",
            RestrictedOther => "restricted_other",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::*;
        match s {
            "activating" => Ok(Activating),
            "capability_not_requested" => Ok(CapabilityNotRequested),
            "financial_account_closed" => Ok(FinancialAccountClosed),
            "rejected_other" => Ok(RejectedOther),
            "rejected_unsupported_business" => Ok(RejectedUnsupportedBusiness),
            "requirements_past_due" => Ok(RequirementsPastDue),
            "requirements_pending_verification" => Ok(RequirementsPendingVerification),
            "restricted_by_platform" => Ok(RestrictedByPlatform),
            "restricted_other" => Ok(RestrictedOther),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Represents what the user should do, if anything, to activate the Feature.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    ContactStripe,
    ProvideInformation,
    RemoveRestriction,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    pub fn as_str(&self) -> &str {
        use TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::*;
        match self {
            ContactStripe => "contact_stripe",
            ProvideInformation => "provide_information",
            RemoveRestriction => "remove_restriction",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::*;
        match s {
            "contact_stripe" => Ok(ContactStripe),
            "provide_information" => Ok(ProvideInformation),
            "remove_restriction" => Ok(RemoveRestriction),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The `platform_restrictions` that are restricting this Feature.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    InboundFlows,
    OutboundFlows,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    pub fn as_str(&self) -> &str {
        use TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::*;
        match self {
            InboundFlows => "inbound_flows",
            OutboundFlows => "outbound_flows",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::*;
        match s {
            "inbound_flows" => Ok(InboundFlows),
            "outbound_flows" => Ok(OutboundFlows),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
