#[derive(Clone, Debug)]
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
                builder: TaxProductResourceCustomerDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceCustomerDetailsBuilder {
        type Out = TaxProductResourceCustomerDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "address_source" => Deserialize::begin(&mut self.address_source),
                "ip_address" => Deserialize::begin(&mut self.ip_address),
                "tax_ids" => Deserialize::begin(&mut self.tax_ids),
                "taxability_override" => Deserialize::begin(&mut self.taxability_override),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                address_source: Deserialize::default(),
                ip_address: Deserialize::default(),
                tax_ids: Deserialize::default(),
                taxability_override: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(address),
                Some(address_source),
                Some(ip_address),
                Some(tax_ids),
                Some(taxability_override),
            ) = (
                self.address.take(),
                self.address_source,
                self.ip_address.take(),
                self.tax_ids.take(),
                self.taxability_override,
            )
            else {
                return None;
            };
            Some(Self::Out { address, address_source, ip_address, tax_ids, taxability_override })
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

    impl ObjectDeser for TaxProductResourceCustomerDetails {
        type Builder = TaxProductResourceCustomerDetailsBuilder;
    }

    impl FromValueOpt for TaxProductResourceCustomerDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceCustomerDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "address_source" => b.address_source = FromValueOpt::from_value(v),
                    "ip_address" => b.ip_address = FromValueOpt::from_value(v),
                    "tax_ids" => b.tax_ids = FromValueOpt::from_value(v),
                    "taxability_override" => b.taxability_override = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of customer address provided.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceCustomerDetailsAddressSource {
    Billing,
    Shipping,
}
impl TaxProductResourceCustomerDetailsAddressSource {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceCustomerDetailsAddressSource::*;
        match self {
            Billing => "billing",
            Shipping => "shipping",
        }
    }
}

impl std::str::FromStr for TaxProductResourceCustomerDetailsAddressSource {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceCustomerDetailsAddressSource::*;
        match s {
            "billing" => Ok(Billing),
            "shipping" => Ok(Shipping),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxProductResourceCustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceCustomerDetailsAddressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for TaxProductResourceCustomerDetailsAddressSource {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxProductResourceCustomerDetailsAddressSource> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductResourceCustomerDetailsAddressSource::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceCustomerDetailsAddressSource);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceCustomerDetailsAddressSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductResourceCustomerDetailsAddressSource",
            )
        })
    }
}
/// The taxability override used for taxation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceCustomerDetailsTaxabilityOverride {
    CustomerExempt,
    None,
    ReverseCharge,
}
impl TaxProductResourceCustomerDetailsTaxabilityOverride {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceCustomerDetailsTaxabilityOverride::*;
        match self {
            CustomerExempt => "customer_exempt",
            None => "none",
            ReverseCharge => "reverse_charge",
        }
    }
}

impl std::str::FromStr for TaxProductResourceCustomerDetailsTaxabilityOverride {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceCustomerDetailsTaxabilityOverride::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
            "none" => Ok(None),
            "reverse_charge" => Ok(ReverseCharge),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxProductResourceCustomerDetailsTaxabilityOverride> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductResourceCustomerDetailsTaxabilityOverride::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceCustomerDetailsTaxabilityOverride);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductResourceCustomerDetailsTaxabilityOverride",
            )
        })
    }
}
