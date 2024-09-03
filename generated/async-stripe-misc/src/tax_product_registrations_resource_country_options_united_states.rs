#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStates {
    pub local_amusement_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>,
    pub local_lease_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>,
    /// Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: String,
    /// Type of registration in the US.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TaxProductRegistrationsResourceCountryOptionsUnitedStatesType,
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder {
    local_amusement_tax: Option<
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>,
    >,
    local_lease_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>>,
    state: Option<String>,
    type_: Option<TaxProductRegistrationsResourceCountryOptionsUnitedStatesType>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsUnitedStates {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsUnitedStates>,
        builder: TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsUnitedStates> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder::deser_default(
                    ),
            }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsUnitedStates;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "local_amusement_tax" => Deserialize::begin(&mut self.local_amusement_tax),
                "local_lease_tax" => Deserialize::begin(&mut self.local_lease_tax),
                "state" => Deserialize::begin(&mut self.state),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                local_amusement_tax: Deserialize::default(),
                local_lease_tax: Deserialize::default(),
                state: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(local_amusement_tax), Some(local_lease_tax), Some(state), Some(type_)) = (
                self.local_amusement_tax.take(),
                self.local_lease_tax.take(),
                self.state.take(),
                self.type_,
            ) else {
                return None;
            };
            Some(Self::Out { local_amusement_tax, local_lease_tax, state, type_ })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsUnitedStates {
        type Builder = TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsUnitedStates {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "local_amusement_tax" => b.local_amusement_tax = FromValueOpt::from_value(v),
                    "local_lease_tax" => b.local_lease_tax = FromValueOpt::from_value(v),
                    "state" => b.state = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of registration in the US.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    LocalAmusementTax,
    LocalLeaseTax,
    StateCommunicationsTax,
    StateSalesTax,
}
impl TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::*;
        match self {
            LocalAmusementTax => "local_amusement_tax",
            LocalLeaseTax => "local_lease_tax",
            StateCommunicationsTax => "state_communications_tax",
            StateSalesTax => "state_sales_tax",
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::*;
        match s {
            "local_amusement_tax" => Ok(LocalAmusementTax),
            "local_lease_tax" => Ok(LocalLeaseTax),
            "state_communications_tax" => Ok(StateCommunicationsTax),
            "state_sales_tax" => Ok(StateSalesTax),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TaxProductRegistrationsResourceCountryOptionsUnitedStatesType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TaxProductRegistrationsResourceCountryOptionsUnitedStatesType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType",
            )
        })
    }
}
