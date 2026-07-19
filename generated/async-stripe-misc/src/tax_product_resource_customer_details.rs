#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceCustomerDetails {
    /// The customer's postal address (for example, home or business location).
    pub address: Option<stripe_misc::TaxProductResourcePostalAddress>,
    /// The type of customer address provided.
    pub address_source: Option<TaxProductResourceCustomerDetailsAddressSource>,
    /// The customer's IP address (IPv4 or IPv6).
    pub ip_address: Option<String>,
    /// The customer's tax IDs (for example, EU VAT numbers).
    pub tax_ids: Vec<stripe_misc::TaxProductResourceCustomerDetailsResourceTaxId>,
    /// The taxability override used for taxation.
    pub taxability_override: TaxProductResourceCustomerDetailsTaxabilityOverride,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceCustomerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceCustomerDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceCustomerDetailsBuilder {
    address: Option<Option<stripe_misc::TaxProductResourcePostalAddress>>,
    address_source: Option<Option<TaxProductResourceCustomerDetailsAddressSource>>,
    ip_address: Option<Option<String>>,
    tax_ids: Option<Vec<stripe_misc::TaxProductResourceCustomerDetailsResourceTaxId>>,
    taxability_override: Option<TaxProductResourceCustomerDetailsTaxabilityOverride>,
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

    impl Deserialize for TaxProductResourceCustomerDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceCustomerDetails>,
        builder: TaxProductResourceCustomerDetailsBuilder,
    }

    impl Visitor for Place<TaxProductResourceCustomerDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceCustomerDetailsBuilder {
                    address: Deserialize::default(),
                    address_source: Deserialize::default(),
                    ip_address: Deserialize::default(),
                    tax_ids: Deserialize::default(),
                    taxability_override: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.builder.address),
                "address_source" => Deserialize::begin(&mut self.builder.address_source),
                "ip_address" => Deserialize::begin(&mut self.builder.ip_address),
                "tax_ids" => Deserialize::begin(&mut self.builder.tax_ids),
                "taxability_override" => Deserialize::begin(&mut self.builder.taxability_override),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(address),
                Some(address_source),
                Some(ip_address),
                Some(tax_ids),
                Some(taxability_override),
            ) = (
                self.builder.address.take(),
                self.builder.address_source.take(),
                self.builder.ip_address.take(),
                self.builder.tax_ids.take(),
                self.builder.taxability_override.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TaxProductResourceCustomerDetails {
                address,
                address_source,
                ip_address,
                tax_ids,
                taxability_override,
            });
            Ok(())
        }
    }
};
/// The type of customer address provided.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductResourceCustomerDetailsAddressSource {
    Billing,
    Shipping,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductResourceCustomerDetailsAddressSource {
    pub fn as_str(&self) -> &str {
        use TaxProductResourceCustomerDetailsAddressSource::*;
        match self {
            Billing => "billing",
            Shipping => "shipping",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductResourceCustomerDetailsAddressSource {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceCustomerDetailsAddressSource::*;
        match s {
            "billing" => Ok(Billing),
            "shipping" => Ok(Shipping),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductResourceCustomerDetailsAddressSource"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxProductResourceCustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxProductResourceCustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceCustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxProductResourceCustomerDetailsAddressSource))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceCustomerDetailsAddressSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TaxProductResourceCustomerDetailsAddressSource {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TaxProductResourceCustomerDetailsAddressSource> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(TaxProductResourceCustomerDetailsAddressSource::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceCustomerDetailsAddressSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The taxability override used for taxation.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductResourceCustomerDetailsTaxabilityOverride {
    CustomerExempt,
    None,
    ReverseCharge,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductResourceCustomerDetailsTaxabilityOverride {
    pub fn as_str(&self) -> &str {
        use TaxProductResourceCustomerDetailsTaxabilityOverride::*;
        match self {
            CustomerExempt => "customer_exempt",
            None => "none",
            ReverseCharge => "reverse_charge",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductResourceCustomerDetailsTaxabilityOverride {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceCustomerDetailsTaxabilityOverride::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
            "none" => Ok(None),
            "reverse_charge" => Ok(ReverseCharge),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductResourceCustomerDetailsTaxabilityOverride"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxProductResourceCustomerDetailsTaxabilityOverride))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TaxProductResourceCustomerDetailsTaxabilityOverride> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductResourceCustomerDetailsTaxabilityOverride::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
