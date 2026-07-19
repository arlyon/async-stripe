#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureDetails {
    /// For authenticated transactions: how the customer was authenticated by
    /// the issuing bank.
    pub authentication_flow: Option<ThreeDSecureDetailsAuthenticationFlow>,
    /// The Electronic Commerce Indicator (ECI). A protocol-level field
    /// indicating what degree of authentication was performed.
    pub electronic_commerce_indicator: Option<ThreeDSecureDetailsElectronicCommerceIndicator>,
    /// Indicates the outcome of 3D Secure authentication.
    pub result: Option<ThreeDSecureDetailsResult>,
    /// Additional information about why 3D Secure succeeded or failed based
    /// on the `result`.
    pub result_reason: Option<ThreeDSecureDetailsResultReason>,
    /// The 3D Secure 1 XID or 3D Secure 2 Directory Server Transaction ID
    /// (dsTransId) for this payment.
    pub transaction_id: Option<String>,
    /// The version of 3D Secure that was used.
    pub version: Option<ThreeDSecureDetailsVersion>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThreeDSecureDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThreeDSecureDetailsBuilder {
    authentication_flow: Option<Option<ThreeDSecureDetailsAuthenticationFlow>>,
    electronic_commerce_indicator: Option<Option<ThreeDSecureDetailsElectronicCommerceIndicator>>,
    result: Option<Option<ThreeDSecureDetailsResult>>,
    result_reason: Option<Option<ThreeDSecureDetailsResultReason>>,
    transaction_id: Option<Option<String>>,
    version: Option<Option<ThreeDSecureDetailsVersion>>,
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

    impl Deserialize for ThreeDSecureDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureDetails>,
        builder: ThreeDSecureDetailsBuilder,
    }

    impl Visitor for Place<ThreeDSecureDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureDetailsBuilder {
                    authentication_flow: Deserialize::default(),
                    electronic_commerce_indicator: Deserialize::default(),
                    result: Deserialize::default(),
                    result_reason: Deserialize::default(),
                    transaction_id: Deserialize::default(),
                    version: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "authentication_flow" => Deserialize::begin(&mut self.builder.authentication_flow),
                "electronic_commerce_indicator" => {
                    Deserialize::begin(&mut self.builder.electronic_commerce_indicator)
                }
                "result" => Deserialize::begin(&mut self.builder.result),
                "result_reason" => Deserialize::begin(&mut self.builder.result_reason),
                "transaction_id" => Deserialize::begin(&mut self.builder.transaction_id),
                "version" => Deserialize::begin(&mut self.builder.version),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(authentication_flow),
                Some(electronic_commerce_indicator),
                Some(result),
                Some(result_reason),
                Some(transaction_id),
                Some(version),
            ) = (
                self.builder.authentication_flow.take(),
                self.builder.electronic_commerce_indicator.take(),
                self.builder.result.take(),
                self.builder.result_reason.take(),
                self.builder.transaction_id.take(),
                self.builder.version.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(ThreeDSecureDetails {
                authentication_flow,
                electronic_commerce_indicator,
                result,
                result_reason,
                transaction_id,
                version,
            });
            Ok(())
        }
    }
};
/// For authenticated transactions: how the customer was authenticated by
/// the issuing bank.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureDetailsAuthenticationFlow {
    Challenge,
    Frictionless,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureDetailsAuthenticationFlow {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureDetailsAuthenticationFlow::*;
        match self {
            Challenge => "challenge",
            Frictionless => "frictionless",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsAuthenticationFlow {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsAuthenticationFlow::*;
        match s {
            "challenge" => Ok(Challenge),
            "frictionless" => Ok(Frictionless),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureDetailsAuthenticationFlow"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureDetailsAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureDetailsAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureDetailsAuthenticationFlow)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureDetailsAuthenticationFlow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ThreeDSecureDetailsAuthenticationFlow {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsAuthenticationFlow> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsAuthenticationFlow::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsAuthenticationFlow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The Electronic Commerce Indicator (ECI). A protocol-level field
/// indicating what degree of authentication was performed.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureDetailsElectronicCommerceIndicator {
    V01,
    V02,
    V05,
    V06,
    V07,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureDetailsElectronicCommerceIndicator {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureDetailsElectronicCommerceIndicator::*;
        match self {
            V01 => "01",
            V02 => "02",
            V05 => "05",
            V06 => "06",
            V07 => "07",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsElectronicCommerceIndicator {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsElectronicCommerceIndicator::*;
        match s {
            "01" => Ok(V01),
            "02" => Ok(V02),
            "05" => Ok(V05),
            "06" => Ok(V06),
            "07" => Ok(V07),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureDetailsElectronicCommerceIndicator"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureDetailsElectronicCommerceIndicator))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsElectronicCommerceIndicator> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(ThreeDSecureDetailsElectronicCommerceIndicator::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Indicates the outcome of 3D Secure authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureDetailsResult {
    AttemptAcknowledged,
    Authenticated,
    Exempted,
    Failed,
    NotSupported,
    ProcessingError,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureDetailsResult {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureDetailsResult::*;
        match self {
            AttemptAcknowledged => "attempt_acknowledged",
            Authenticated => "authenticated",
            Exempted => "exempted",
            Failed => "failed",
            NotSupported => "not_supported",
            ProcessingError => "processing_error",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsResult {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsResult::*;
        match s {
            "attempt_acknowledged" => Ok(AttemptAcknowledged),
            "authenticated" => Ok(Authenticated),
            "exempted" => Ok(Exempted),
            "failed" => Ok(Failed),
            "not_supported" => Ok(NotSupported),
            "processing_error" => Ok(ProcessingError),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ThreeDSecureDetailsResult");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureDetailsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureDetailsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureDetailsResult)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureDetailsResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ThreeDSecureDetailsResult {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsResult> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsResult::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Additional information about why 3D Secure succeeded or failed based
/// on the `result`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureDetailsResultReason {
    Abandoned,
    Bypassed,
    Canceled,
    CardNotEnrolled,
    NetworkNotSupported,
    ProtocolError,
    Rejected,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureDetailsResultReason {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureDetailsResultReason::*;
        match self {
            Abandoned => "abandoned",
            Bypassed => "bypassed",
            Canceled => "canceled",
            CardNotEnrolled => "card_not_enrolled",
            NetworkNotSupported => "network_not_supported",
            ProtocolError => "protocol_error",
            Rejected => "rejected",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsResultReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsResultReason::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "bypassed" => Ok(Bypassed),
            "canceled" => Ok(Canceled),
            "card_not_enrolled" => Ok(CardNotEnrolled),
            "network_not_supported" => Ok(NetworkNotSupported),
            "protocol_error" => Ok(ProtocolError),
            "rejected" => Ok(Rejected),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureDetailsResultReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureDetailsResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureDetailsResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureDetailsResultReason)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureDetailsResultReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ThreeDSecureDetailsResultReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsResultReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsResultReason::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsResultReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The version of 3D Secure that was used.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureDetailsVersion {
    V1_0_2,
    V2_1_0,
    V2_2_0,
    V2_3_0,
    V2_3_1,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureDetailsVersion {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureDetailsVersion::*;
        match self {
            V1_0_2 => "1.0.2",
            V2_1_0 => "2.1.0",
            V2_2_0 => "2.2.0",
            V2_3_0 => "2.3.0",
            V2_3_1 => "2.3.1",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsVersion {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsVersion::*;
        match s {
            "1.0.2" => Ok(V1_0_2),
            "2.1.0" => Ok(V2_1_0),
            "2.2.0" => Ok(V2_2_0),
            "2.3.0" => Ok(V2_3_0),
            "2.3.1" => Ok(V2_3_1),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ThreeDSecureDetailsVersion");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureDetailsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureDetailsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureDetailsVersion)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureDetailsVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ThreeDSecureDetailsVersion {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsVersion> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsVersion::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
