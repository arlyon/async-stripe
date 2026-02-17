#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaypalSellerProtection {
    /// An array of conditions that are covered for the transaction, if applicable.
    pub dispute_categories: Option<Vec<PaypalSellerProtectionDisputeCategories>>,
    /// Indicates whether the transaction is eligible for PayPal's seller protection.
    pub status: PaypalSellerProtectionStatus,
}
#[doc(hidden)]
pub struct PaypalSellerProtectionBuilder {
    dispute_categories: Option<Option<Vec<PaypalSellerProtectionDisputeCategories>>>,
    status: Option<PaypalSellerProtectionStatus>,
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
                builder: PaypalSellerProtectionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaypalSellerProtectionBuilder {
        type Out = PaypalSellerProtection;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dispute_categories" => Deserialize::begin(&mut self.dispute_categories),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { dispute_categories: Deserialize::default(), status: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(dispute_categories), Some(status)) =
                (self.dispute_categories.take(), self.status.take())
            else {
                return None;
            };
            Some(Self::Out { dispute_categories, status })
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

    impl ObjectDeser for PaypalSellerProtection {
        type Builder = PaypalSellerProtectionBuilder;
    }

    impl FromValueOpt for PaypalSellerProtection {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaypalSellerProtectionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "dispute_categories" => b.dispute_categories = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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

impl std::fmt::Debug for PaypalSellerProtectionDisputeCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PaypalSellerProtectionDisputeCategories {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaypalSellerProtectionDisputeCategories> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaypalSellerProtectionDisputeCategories::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaypalSellerProtectionDisputeCategories);
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

impl std::fmt::Debug for PaypalSellerProtectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PaypalSellerProtectionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaypalSellerProtectionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaypalSellerProtectionStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaypalSellerProtectionStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaypalSellerProtectionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
