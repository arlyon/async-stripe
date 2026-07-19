#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    pub jurisdiction: Option<String>,
    /// The type of the election for the state sales tax registration.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionBuilder {
    jurisdiction: Option<Option<String>>,
    type_: Option<TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType>,
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
                builder:
                    TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionBuilder {
                        jurisdiction: Deserialize::default(),
                        type_: Deserialize::default(),
                    },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "jurisdiction" => Deserialize::begin(&mut self.builder.jurisdiction),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(jurisdiction), Some(type_)) =
                (self.builder.jurisdiction.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection {
                    jurisdiction,
                    type_,
                });
            Ok(())
        }
    }
};
/// The type of the election for the state sales tax registration.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    LocalUseTax,
    SimplifiedSellersUseTax,
    SingleLocalUseTax,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    pub fn as_str(&self) -> &str {
        use TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::*;
        match self {
            LocalUseTax => "local_use_tax",
            SimplifiedSellersUseTax => "simplified_sellers_use_tax",
            SingleLocalUseTax => "single_local_use_tax",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::*;
        match s {
            "local_use_tax" => Ok(LocalUseTax),
            "simplified_sellers_use_tax" => Ok(SimplifiedSellersUseTax),
            "single_local_use_tax" => Ok(SingleLocalUseTax),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType"
                );
                Ok(Unknown(v.to_owned()))
            }
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType
        ))
        .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize
    for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
