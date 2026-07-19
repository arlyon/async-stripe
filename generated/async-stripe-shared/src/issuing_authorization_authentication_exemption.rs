#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationAuthenticationExemption {
    /// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
    pub claimed_by: IssuingAuthorizationAuthenticationExemptionClaimedBy,
    /// The specific exemption claimed for this authorization.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: IssuingAuthorizationAuthenticationExemptionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationAuthenticationExemption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationAuthenticationExemption").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingAuthorizationAuthenticationExemptionBuilder {
    claimed_by: Option<IssuingAuthorizationAuthenticationExemptionClaimedBy>,
    type_: Option<IssuingAuthorizationAuthenticationExemptionType>,
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
                builder: IssuingAuthorizationAuthenticationExemptionBuilder {
                    claimed_by: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "claimed_by" => Deserialize::begin(&mut self.builder.claimed_by),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(claimed_by), Some(type_)) =
                (self.builder.claimed_by.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(IssuingAuthorizationAuthenticationExemption { claimed_by, type_ });
            Ok(())
        }
    }
};
/// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingAuthorizationAuthenticationExemptionClaimedBy {
    Acquirer,
    Issuer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingAuthorizationAuthenticationExemptionClaimedBy {
    pub fn as_str(&self) -> &str {
        use IssuingAuthorizationAuthenticationExemptionClaimedBy::*;
        match self {
            Acquirer => "acquirer",
            Issuer => "issuer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationAuthenticationExemptionClaimedBy::*;
        match s {
            "acquirer" => Ok(Acquirer),
            "issuer" => Ok(Issuer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingAuthorizationAuthenticationExemptionClaimedBy"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingAuthorizationAuthenticationExemptionClaimedBy))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingAuthorizationAuthenticationExemptionClaimedBy> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingAuthorizationAuthenticationExemptionClaimedBy::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationAuthenticationExemptionClaimedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The specific exemption claimed for this authorization.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingAuthorizationAuthenticationExemptionType {
    LowValueTransaction,
    TransactionRiskAnalysis,
    Unknown,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    /// This variant is prefixed with an underscore to avoid conflicts with Stripe's 'Unknown' variant.
    _Unknown(String),
}
impl IssuingAuthorizationAuthenticationExemptionType {
    pub fn as_str(&self) -> &str {
        use IssuingAuthorizationAuthenticationExemptionType::*;
        match self {
            LowValueTransaction => "low_value_transaction",
            TransactionRiskAnalysis => "transaction_risk_analysis",
            Unknown => "unknown",
            _Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationAuthenticationExemptionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationAuthenticationExemptionType::*;
        match s {
            "low_value_transaction" => Ok(LowValueTransaction),
            "transaction_risk_analysis" => Ok(TransactionRiskAnalysis),
            "unknown" => Ok(Unknown),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingAuthorizationAuthenticationExemptionType"
                );
                Ok(_Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationAuthenticationExemptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingAuthorizationAuthenticationExemptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationAuthenticationExemptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingAuthorizationAuthenticationExemptionType))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for IssuingAuthorizationAuthenticationExemptionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingAuthorizationAuthenticationExemptionType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingAuthorizationAuthenticationExemptionType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationAuthenticationExemptionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
