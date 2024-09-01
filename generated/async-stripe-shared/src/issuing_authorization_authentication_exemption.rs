#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationAuthenticationExemption {
    /// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
    pub claimed_by: IssuingAuthorizationAuthenticationExemptionClaimedBy,
    /// The specific exemption claimed for this authorization.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: IssuingAuthorizationAuthenticationExemptionType,
}
#[doc(hidden)]
pub struct IssuingAuthorizationAuthenticationExemptionBuilder {
    claimed_by: Option<IssuingAuthorizationAuthenticationExemptionClaimedBy>,
    type_: Option<IssuingAuthorizationAuthenticationExemptionType>,
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

    impl Deserialize for IssuingAuthorizationAuthenticationExemption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationAuthenticationExemption>,
        builder: IssuingAuthorizationAuthenticationExemptionBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationAuthenticationExemption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationAuthenticationExemptionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationAuthenticationExemptionBuilder {
        type Out = IssuingAuthorizationAuthenticationExemption;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "claimed_by" => Deserialize::begin(&mut self.claimed_by),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { claimed_by: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(claimed_by), Some(type_)) = (self.claimed_by, self.type_) else {
                return None;
            };
            Some(Self::Out { claimed_by, type_ })
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

    impl ObjectDeser for IssuingAuthorizationAuthenticationExemption {
        type Builder = IssuingAuthorizationAuthenticationExemptionBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationAuthenticationExemption {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationAuthenticationExemptionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "claimed_by" => b.claimed_by = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationAuthenticationExemptionClaimedBy {
    Acquirer,
    Issuer,
}
impl IssuingAuthorizationAuthenticationExemptionClaimedBy {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationAuthenticationExemptionClaimedBy::*;
        match self {
            Acquirer => "acquirer",
            Issuer => "issuer",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationAuthenticationExemptionClaimedBy::*;
        match s {
            "acquirer" => Ok(Acquirer),
            "issuer" => Ok(Issuer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationAuthenticationExemptionClaimedBy> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingAuthorizationAuthenticationExemptionClaimedBy::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingAuthorizationAuthenticationExemptionClaimedBy);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingAuthorizationAuthenticationExemptionClaimedBy",
            )
        })
    }
}
/// The specific exemption claimed for this authorization.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingAuthorizationAuthenticationExemptionType {
    LowValueTransaction,
    TransactionRiskAnalysis,
    Unknown,
}
impl IssuingAuthorizationAuthenticationExemptionType {
    pub fn as_str(self) -> &'static str {
        use IssuingAuthorizationAuthenticationExemptionType::*;
        match self {
            LowValueTransaction => "low_value_transaction",
            TransactionRiskAnalysis => "transaction_risk_analysis",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationAuthenticationExemptionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationAuthenticationExemptionType::*;
        match s {
            "low_value_transaction" => Ok(LowValueTransaction),
            "transaction_risk_analysis" => Ok(TransactionRiskAnalysis),
            "unknown" => Ok(Unknown),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationAuthenticationExemptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingAuthorizationAuthenticationExemptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingAuthorizationAuthenticationExemptionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingAuthorizationAuthenticationExemptionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationAuthenticationExemptionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingAuthorizationAuthenticationExemptionType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingAuthorizationAuthenticationExemptionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationAuthenticationExemptionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingAuthorizationAuthenticationExemptionType",
            )
        })
    }
}
