#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaypalSellerProtection {
    /// An array of conditions that are covered for the transaction, if applicable.
    pub dispute_categories: Option<Vec<PaypalSellerProtectionDisputeCategories>>,
    /// Indicates whether the transaction is eligible for PayPal's seller protection.
    pub status: PaypalSellerProtectionStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaypalSellerProtection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaypalSellerProtection").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaypalSellerProtectionBuilder {
    dispute_categories: Option<Option<Vec<PaypalSellerProtectionDisputeCategories>>>,
    status: Option<PaypalSellerProtectionStatus>,
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

    impl Deserialize for PaypalSellerProtection {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaypalSellerProtection>,
        builder: PaypalSellerProtectionBuilder,
    }

    impl Visitor for Place<PaypalSellerProtection> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaypalSellerProtectionBuilder {
                    dispute_categories: Deserialize::default(),
                    status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dispute_categories" => Deserialize::begin(&mut self.builder.dispute_categories),
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(dispute_categories), Some(status)) =
                (self.builder.dispute_categories.take(), self.builder.status.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaypalSellerProtection { dispute_categories, status });
            Ok(())
        }
    }
};
/// An array of conditions that are covered for the transaction, if applicable.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaypalSellerProtectionDisputeCategories {
    Fraudulent,
    ProductNotReceived,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaypalSellerProtectionDisputeCategories {
    pub fn as_str(&self) -> &str {
        use PaypalSellerProtectionDisputeCategories::*;
        match self {
            Fraudulent => "fraudulent",
            ProductNotReceived => "product_not_received",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaypalSellerProtectionDisputeCategories {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaypalSellerProtectionDisputeCategories::*;
        match s {
            "fraudulent" => Ok(Fraudulent),
            "product_not_received" => Ok(ProductNotReceived),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaypalSellerProtectionDisputeCategories"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaypalSellerProtectionDisputeCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaypalSellerProtectionDisputeCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaypalSellerProtectionDisputeCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaypalSellerProtectionDisputeCategories)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaypalSellerProtectionDisputeCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaypalSellerProtectionDisputeCategories {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaypalSellerProtectionDisputeCategories> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaypalSellerProtectionDisputeCategories::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaypalSellerProtectionDisputeCategories {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Indicates whether the transaction is eligible for PayPal's seller protection.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaypalSellerProtectionStatus {
    Eligible,
    NotEligible,
    PartiallyEligible,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaypalSellerProtectionStatus {
    pub fn as_str(&self) -> &str {
        use PaypalSellerProtectionStatus::*;
        match self {
            Eligible => "eligible",
            NotEligible => "not_eligible",
            PartiallyEligible => "partially_eligible",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaypalSellerProtectionStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaypalSellerProtectionStatus::*;
        match s {
            "eligible" => Ok(Eligible),
            "not_eligible" => Ok(NotEligible),
            "partially_eligible" => Ok(PartiallyEligible),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaypalSellerProtectionStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaypalSellerProtectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaypalSellerProtectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaypalSellerProtectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaypalSellerProtectionStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaypalSellerProtectionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaypalSellerProtectionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaypalSellerProtectionStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaypalSellerProtectionStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaypalSellerProtectionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
