#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    pub jurisdiction: Option<String>,
    /// The type of the election for the state sales tax registration.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType,
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionBuilder {
    jurisdiction: Option<Option<String>>,
    type_: Option<TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection>,
        builder: TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "jurisdiction" => Deserialize::begin(&mut self.jurisdiction),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { jurisdiction: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(jurisdiction), Some(type_)) = (self.jurisdiction.take(), self.type_) else {
                return None;
            };
            Some(Self::Out { jurisdiction, type_ })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection {
        type Builder = TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "jurisdiction" => b.jurisdiction = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the election for the state sales tax registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    LocalUseTax,
    SimplifiedSellersUseTax,
    SingleLocalUseTax,
}
impl TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::*;
        match self {
            LocalUseTax => "local_use_tax",
            SimplifiedSellersUseTax => "simplified_sellers_use_tax",
            SingleLocalUseTax => "single_local_use_tax",
        }
    }
}

impl std::str::FromStr
    for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::*;
        match s {
            "local_use_tax" => Ok(LocalUseTax),
            "simplified_sellers_use_tax" => Ok(SimplifiedSellersUseTax),
            "single_local_use_tax" => Ok(SingleLocalUseTax),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType"))
    }
}
