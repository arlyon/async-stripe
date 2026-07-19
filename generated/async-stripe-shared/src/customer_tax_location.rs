#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerTaxLocation {
    /// The identified tax country of the customer.
    pub country: String,
    /// The data source used to infer the customer's location.
    pub source: CustomerTaxLocationSource,
    /// The identified tax state, county, province, or region of the customer.
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerTaxLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerTaxLocation").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerTaxLocationBuilder {
    country: Option<String>,
    source: Option<CustomerTaxLocationSource>,
    state: Option<Option<String>>,
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

    impl Deserialize for CustomerTaxLocation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerTaxLocation>,
        builder: CustomerTaxLocationBuilder,
    }

    impl Visitor for Place<CustomerTaxLocation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerTaxLocationBuilder {
                    country: Deserialize::default(),
                    source: Deserialize::default(),
                    state: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "country" => Deserialize::begin(&mut self.builder.country),
                "source" => Deserialize::begin(&mut self.builder.source),
                "state" => Deserialize::begin(&mut self.builder.state),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(country), Some(source), Some(state)) = (
                self.builder.country.take(),
                self.builder.source.take(),
                self.builder.state.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(CustomerTaxLocation { country, source, state });
            Ok(())
        }
    }
};
/// The data source used to infer the customer's location.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CustomerTaxLocationSource {
    BillingAddress,
    IpAddress,
    PaymentMethod,
    ShippingDestination,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CustomerTaxLocationSource {
    pub fn as_str(&self) -> &str {
        use CustomerTaxLocationSource::*;
        match self {
            BillingAddress => "billing_address",
            IpAddress => "ip_address",
            PaymentMethod => "payment_method",
            ShippingDestination => "shipping_destination",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CustomerTaxLocationSource {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerTaxLocationSource::*;
        match s {
            "billing_address" => Ok(BillingAddress),
            "ip_address" => Ok(IpAddress),
            "payment_method" => Ok(PaymentMethod),
            "shipping_destination" => Ok(ShippingDestination),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CustomerTaxLocationSource");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CustomerTaxLocationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CustomerTaxLocationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerTaxLocationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CustomerTaxLocationSource)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerTaxLocationSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for CustomerTaxLocationSource {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CustomerTaxLocationSource> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerTaxLocationSource::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerTaxLocationSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
