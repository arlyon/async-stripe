#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationFuelData {
    /// [Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying the primary fuel product purchased.
    pub industry_product_code: Option<String>,
    /// The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places.
    pub quantity_decimal: Option<String>,
    /// The type of fuel that was purchased.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<IssuingAuthorizationFuelDataType>,
    /// The units for `quantity_decimal`.
    pub unit: Option<IssuingAuthorizationFuelDataUnit>,
    /// The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    pub unit_cost_decimal: Option<String>,
}
#[doc(hidden)]
pub struct IssuingAuthorizationFuelDataBuilder {
    industry_product_code: Option<Option<String>>,
    quantity_decimal: Option<Option<String>>,
    type_: Option<Option<IssuingAuthorizationFuelDataType>>,
    unit: Option<Option<IssuingAuthorizationFuelDataUnit>>,
    unit_cost_decimal: Option<Option<String>>,
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

    impl Deserialize for IssuingAuthorizationFuelData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationFuelData>,
        builder: IssuingAuthorizationFuelDataBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationFuelData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationFuelDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationFuelDataBuilder {
        type Out = IssuingAuthorizationFuelData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "industry_product_code" => Deserialize::begin(&mut self.industry_product_code),
                "quantity_decimal" => Deserialize::begin(&mut self.quantity_decimal),
                "type" => Deserialize::begin(&mut self.type_),
                "unit" => Deserialize::begin(&mut self.unit),
                "unit_cost_decimal" => Deserialize::begin(&mut self.unit_cost_decimal),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                industry_product_code: Deserialize::default(),
                quantity_decimal: Deserialize::default(),
                type_: Deserialize::default(),
                unit: Deserialize::default(),
                unit_cost_decimal: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(industry_product_code),
                Some(quantity_decimal),
                Some(type_),
                Some(unit),
                Some(unit_cost_decimal),
            ) = (
                self.industry_product_code.take(),
                self.quantity_decimal.take(),
                self.type_,
                self.unit,
                self.unit_cost_decimal.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                industry_product_code,
                quantity_decimal,
                type_,
                unit,
                unit_cost_decimal,
            })
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

    impl ObjectDeser for IssuingAuthorizationFuelData {
        type Builder = IssuingAuthorizationFuelDataBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationFuelData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationFuelDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "industry_product_code" => {
                        b.industry_product_code = FromValueOpt::from_value(v)
                    }
                    "quantity_decimal" => b.quantity_decimal = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "unit" => b.unit = FromValueOpt::from_value(v),
                    "unit_cost_decimal" => b.unit_cost_decimal = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of fuel that was purchased.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationFuelDataType {
    Diesel,
    Other,
    UnleadedPlus,
    UnleadedRegular,
    UnleadedSuper,
}
impl IssuingAuthorizationFuelDataType {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationFuelDataType::*;
        match self {
            Diesel => "diesel",
            Other => "other",
            UnleadedPlus => "unleaded_plus",
            UnleadedRegular => "unleaded_regular",
            UnleadedSuper => "unleaded_super",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationFuelDataType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationFuelDataType::*;
        match s {
            "diesel" => Ok(Diesel),
            "other" => Ok(Other),
            "unleaded_plus" => Ok(UnleadedPlus),
            "unleaded_regular" => Ok(UnleadedRegular),
            "unleaded_super" => Ok(UnleadedSuper),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationFuelDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationFuelDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingAuthorizationFuelDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingAuthorizationFuelDataType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationFuelDataType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingAuthorizationFuelDataType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingAuthorizationFuelDataType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationFuelDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingAuthorizationFuelDataType")
        })
    }
}
/// The units for `quantity_decimal`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationFuelDataUnit {
    ChargingMinute,
    ImperialGallon,
    Kilogram,
    KilowattHour,
    Liter,
    Other,
    Pound,
    UsGallon,
}
impl IssuingAuthorizationFuelDataUnit {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationFuelDataUnit::*;
        match self {
            ChargingMinute => "charging_minute",
            ImperialGallon => "imperial_gallon",
            Kilogram => "kilogram",
            KilowattHour => "kilowatt_hour",
            Liter => "liter",
            Other => "other",
            Pound => "pound",
            UsGallon => "us_gallon",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationFuelDataUnit {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationFuelDataUnit::*;
        match s {
            "charging_minute" => Ok(ChargingMinute),
            "imperial_gallon" => Ok(ImperialGallon),
            "kilogram" => Ok(Kilogram),
            "kilowatt_hour" => Ok(KilowattHour),
            "liter" => Ok(Liter),
            "other" => Ok(Other),
            "pound" => Ok(Pound),
            "us_gallon" => Ok(UsGallon),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationFuelDataUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationFuelDataUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingAuthorizationFuelDataUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingAuthorizationFuelDataUnit {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationFuelDataUnit> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingAuthorizationFuelDataUnit::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingAuthorizationFuelDataUnit);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationFuelDataUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingAuthorizationFuelDataUnit")
        })
    }
}
