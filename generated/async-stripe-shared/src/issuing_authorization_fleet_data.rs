#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationFleetData {
    /// Answers to prompts presented to the cardholder at the point of sale.
    /// Prompted fields vary depending on the configuration of your physical fleet cards.
    /// Typical points of sale support only numeric entry.
    pub cardholder_prompt_data:
        Option<stripe_shared::IssuingAuthorizationFleetCardholderPromptData>,
    /// The type of purchase.
    pub purchase_type: Option<IssuingAuthorizationFleetDataPurchaseType>,
    /// More information about the total amount.
    /// Typically this information is received from the merchant after the authorization has been approved and the fuel dispensed.
    /// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
    pub reported_breakdown: Option<stripe_shared::IssuingAuthorizationFleetReportedBreakdown>,
    /// The type of fuel service.
    pub service_type: Option<IssuingAuthorizationFleetDataServiceType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFleetData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationFleetData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationFleetDataBuilder {
    cardholder_prompt_data: Option<stripe_shared::IssuingAuthorizationFleetCardholderPromptData>,
    purchase_type: Option<IssuingAuthorizationFleetDataPurchaseType>,
    reported_breakdown: Option<stripe_shared::IssuingAuthorizationFleetReportedBreakdown>,
    service_type: Option<IssuingAuthorizationFleetDataServiceType>,
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

    impl Deserialize for IssuingAuthorizationFleetData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationFleetData>,
        builder: IssuingAuthorizationFleetDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationFleetData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationFleetDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationFleetDataBuilder {
        type Out = IssuingAuthorizationFleetData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cardholder_prompt_data" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.cardholder_prompt_data,
                ),
                "purchase_type" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.purchase_type)
                }
                "reported_breakdown" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.reported_breakdown)
                }
                "service_type" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.service_type)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                cardholder_prompt_data: None,
                purchase_type: None,
                reported_breakdown: None,
                service_type: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (cardholder_prompt_data, purchase_type, reported_breakdown, service_type) = (
                self.cardholder_prompt_data.take(),
                self.purchase_type.take(),
                self.reported_breakdown.take(),
                self.service_type.take(),
            ) else {
                return None;
            };
            Some(Self::Out {
                cardholder_prompt_data,
                purchase_type,
                reported_breakdown,
                service_type,
            })
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

    impl ObjectDeser for IssuingAuthorizationFleetData {
        type Builder = IssuingAuthorizationFleetDataBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationFleetData {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// The type of purchase.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingAuthorizationFleetDataPurchaseType {
    FuelAndNonFuelPurchase,
    FuelPurchase,
    NonFuelPurchase,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingAuthorizationFleetDataPurchaseType {
    pub fn as_str(&self) -> &str {
        use IssuingAuthorizationFleetDataPurchaseType::*;
        match self {
            FuelAndNonFuelPurchase => "fuel_and_non_fuel_purchase",
            FuelPurchase => "fuel_purchase",
            NonFuelPurchase => "non_fuel_purchase",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationFleetDataPurchaseType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationFleetDataPurchaseType::*;
        match s {
            "fuel_and_non_fuel_purchase" => Ok(FuelAndNonFuelPurchase),
            "fuel_purchase" => Ok(FuelPurchase),
            "non_fuel_purchase" => Ok(NonFuelPurchase),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingAuthorizationFleetDataPurchaseType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationFleetDataPurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingAuthorizationFleetDataPurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFleetDataPurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingAuthorizationFleetDataPurchaseType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingAuthorizationFleetDataPurchaseType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingAuthorizationFleetDataPurchaseType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationFleetDataPurchaseType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingAuthorizationFleetDataPurchaseType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingAuthorizationFleetDataPurchaseType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationFleetDataPurchaseType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The type of fuel service.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingAuthorizationFleetDataServiceType {
    FullService,
    NonFuelTransaction,
    SelfService,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingAuthorizationFleetDataServiceType {
    pub fn as_str(&self) -> &str {
        use IssuingAuthorizationFleetDataServiceType::*;
        match self {
            FullService => "full_service",
            NonFuelTransaction => "non_fuel_transaction",
            SelfService => "self_service",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationFleetDataServiceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationFleetDataServiceType::*;
        match s {
            "full_service" => Ok(FullService),
            "non_fuel_transaction" => Ok(NonFuelTransaction),
            "self_service" => Ok(SelfService),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingAuthorizationFleetDataServiceType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationFleetDataServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingAuthorizationFleetDataServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFleetDataServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingAuthorizationFleetDataServiceType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingAuthorizationFleetDataServiceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingAuthorizationFleetDataServiceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationFleetDataServiceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingAuthorizationFleetDataServiceType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingAuthorizationFleetDataServiceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationFleetDataServiceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
