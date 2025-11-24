#[derive(Clone, Debug)]
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
                builder: CustomerTaxBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CustomerTaxBuilder {
        type Out = CustomerTax;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "automatic_tax" => Deserialize::begin(&mut self.automatic_tax),
                "ip_address" => Deserialize::begin(&mut self.ip_address),
                "location" => Deserialize::begin(&mut self.location),
                "provider" => Deserialize::begin(&mut self.provider),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                automatic_tax: Deserialize::default(),
                ip_address: Deserialize::default(),
                location: Deserialize::default(),
                provider: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(automatic_tax), Some(ip_address), Some(location), Some(provider)) = (
                self.automatic_tax.take(),
                self.ip_address.take(),
                self.location.take(),
                self.provider.take(),
            ) else {
                return None;
            };
            Some(Self::Out { automatic_tax, ip_address, location, provider })
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

    impl ObjectDeser for CustomerTax {
        type Builder = CustomerTaxBuilder;
    }

    impl FromValueOpt for CustomerTax {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerTaxBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "automatic_tax" => b.automatic_tax = FromValueOpt::from_value(v),
                    "ip_address" => b.ip_address = FromValueOpt::from_value(v),
                    "location" => b.location = FromValueOpt::from_value(v),
                    "provider" => b.provider = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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

impl std::fmt::Debug for CustomerTaxAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for CustomerTaxAutomaticTax {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerTaxAutomaticTax> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerTaxAutomaticTax::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CustomerTaxAutomaticTax);
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

impl std::fmt::Debug for CustomerTaxProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for CustomerTaxProvider {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerTaxProvider> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerTaxProvider::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CustomerTaxProvider);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerTaxProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
