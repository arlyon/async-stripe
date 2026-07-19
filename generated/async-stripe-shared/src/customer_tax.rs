#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerTax {
    /// Surfaces if automatic tax computation is possible given the current customer location information.
    pub automatic_tax: CustomerTaxAutomaticTax,
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    pub ip_address: Option<String>,
    /// The identified tax location of the customer.
    pub location: Option<stripe_shared::CustomerTaxLocation>,
    /// The tax calculation provider used for location resolution.
    /// Defaults to `stripe` when not using a [third-party provider](/tax/third-party-apps).
    pub provider: CustomerTaxProvider,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerTax").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerTaxBuilder {
    automatic_tax: Option<CustomerTaxAutomaticTax>,
    ip_address: Option<Option<String>>,
    location: Option<Option<stripe_shared::CustomerTaxLocation>>,
    provider: Option<CustomerTaxProvider>,
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

    impl Deserialize for CustomerTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerTax>,
        builder: CustomerTaxBuilder,
    }

    impl Visitor for Place<CustomerTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerTaxBuilder {
                    automatic_tax: Deserialize::default(),
                    ip_address: Deserialize::default(),
                    location: Deserialize::default(),
                    provider: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "automatic_tax" => Deserialize::begin(&mut self.builder.automatic_tax),
                "ip_address" => Deserialize::begin(&mut self.builder.ip_address),
                "location" => Deserialize::begin(&mut self.builder.location),
                "provider" => Deserialize::begin(&mut self.builder.provider),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(automatic_tax), Some(ip_address), Some(location), Some(provider)) = (
                self.builder.automatic_tax.take(),
                self.builder.ip_address.take(),
                self.builder.location.take(),
                self.builder.provider.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(CustomerTax { automatic_tax, ip_address, location, provider });
            Ok(())
        }
    }
};
/// Surfaces if automatic tax computation is possible given the current customer location information.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CustomerTaxAutomaticTax {
    Failed,
    NotCollecting,
    Supported,
    UnrecognizedLocation,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CustomerTaxAutomaticTax {
    pub fn as_str(&self) -> &str {
        use CustomerTaxAutomaticTax::*;
        match self {
            Failed => "failed",
            NotCollecting => "not_collecting",
            Supported => "supported",
            UnrecognizedLocation => "unrecognized_location",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CustomerTaxAutomaticTax {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerTaxAutomaticTax::*;
        match s {
            "failed" => Ok(Failed),
            "not_collecting" => Ok(NotCollecting),
            "supported" => Ok(Supported),
            "unrecognized_location" => Ok(UnrecognizedLocation),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CustomerTaxAutomaticTax");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CustomerTaxAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CustomerTaxAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerTaxAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CustomerTaxAutomaticTax)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerTaxAutomaticTax {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for CustomerTaxAutomaticTax {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CustomerTaxAutomaticTax> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerTaxAutomaticTax::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerTaxAutomaticTax {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The tax calculation provider used for location resolution.
/// Defaults to `stripe` when not using a [third-party provider](/tax/third-party-apps).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CustomerTaxProvider {
    Anrok,
    Avalara,
    Sphere,
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CustomerTaxProvider {
    pub fn as_str(&self) -> &str {
        use CustomerTaxProvider::*;
        match self {
            Anrok => "anrok",
            Avalara => "avalara",
            Sphere => "sphere",
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CustomerTaxProvider {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerTaxProvider::*;
        match s {
            "anrok" => Ok(Anrok),
            "avalara" => Ok(Avalara),
            "sphere" => Ok(Sphere),
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CustomerTaxProvider");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CustomerTaxProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CustomerTaxProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerTaxProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CustomerTaxProvider)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerTaxProvider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for CustomerTaxProvider {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CustomerTaxProvider> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerTaxProvider::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerTaxProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
