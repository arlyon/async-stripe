#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceCustomerDetails {
    /// The customer's postal address (for example, home or business location).
    pub address: Option<stripe_misc::TaxProductResourcePostalAddress>,
    /// The type of customer address provided.
    pub address_source: Option<stripe_shared::AddressSource>,
    /// The customer's IP address (IPv4 or IPv6).
    pub ip_address: Option<String>,
    /// The customer's tax IDs (for example, EU VAT numbers).
    pub tax_ids: Vec<stripe_misc::TaxProductResourceCustomerDetailsResourceTaxId>,
    /// The taxability override used for taxation.
    pub taxability_override: stripe_shared::TaxabilityOverride,
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
    address_source: Option<Option<stripe_shared::AddressSource>>,
    ip_address: Option<Option<String>>,
    tax_ids: Option<Vec<stripe_misc::TaxProductResourceCustomerDetailsResourceTaxId>>,
    taxability_override: Option<stripe_shared::TaxabilityOverride>,
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
                address: Some(None),
                address_source: Some(None),
                ip_address: Some(None),
                tax_ids: None,
                taxability_override: None,
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
                self.address_source.take(),
                self.ip_address.take(),
                self.tax_ids.take(),
                self.taxability_override.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { address, address_source, ip_address, tax_ids, taxability_override })
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
pub type TaxProductResourceCustomerDetailsAddressSource = stripe_shared::AddressSource;
/// The taxability override used for taxation.
pub type TaxProductResourceCustomerDetailsTaxabilityOverride = stripe_shared::TaxabilityOverride;
