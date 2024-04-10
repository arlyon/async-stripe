#[derive(Clone, Debug)]
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

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
            Some(Self::Out {
                dispute_categories: self.dispute_categories.take()?,
                status: self.status?,
            })
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
                    "dispute_categories" => {
                        b.dispute_categories = Some(FromValueOpt::from_value(v)?)
                    }
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// An array of conditions that are covered for the transaction, if applicable.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaypalSellerProtectionDisputeCategories {
    Fraudulent,
    ProductNotReceived,
}
impl PaypalSellerProtectionDisputeCategories {
    pub fn as_str(self) -> &'static str {
        use PaypalSellerProtectionDisputeCategories::*;
        match self {
            Fraudulent => "fraudulent",
            ProductNotReceived => "product_not_received",
        }
    }
}

impl std::str::FromStr for PaypalSellerProtectionDisputeCategories {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaypalSellerProtectionDisputeCategories::*;
        match s {
            "fraudulent" => Ok(Fraudulent),
            "product_not_received" => Ok(ProductNotReceived),
            _ => Err(()),
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
        self.out = Some(
            PaypalSellerProtectionDisputeCategories::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaypalSellerProtectionDisputeCategories);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaypalSellerProtectionDisputeCategories {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaypalSellerProtectionDisputeCategories")
        })
    }
}
/// Indicates whether the transaction is eligible for PayPal's seller protection.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaypalSellerProtectionStatus {
    Eligible,
    NotEligible,
    PartiallyEligible,
}
impl PaypalSellerProtectionStatus {
    pub fn as_str(self) -> &'static str {
        use PaypalSellerProtectionStatus::*;
        match self {
            Eligible => "eligible",
            NotEligible => "not_eligible",
            PartiallyEligible => "partially_eligible",
        }
    }
}

impl std::str::FromStr for PaypalSellerProtectionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaypalSellerProtectionStatus::*;
        match s {
            "eligible" => Ok(Eligible),
            "not_eligible" => Ok(NotEligible),
            "partially_eligible" => Ok(PartiallyEligible),
            _ => Err(()),
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
        self.out = Some(PaypalSellerProtectionStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaypalSellerProtectionStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaypalSellerProtectionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaypalSellerProtectionStatus"))
    }
}
