#[derive(Clone, Debug)]
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
                builder: ThreeDSecureDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureDetailsBuilder {
        type Out = ThreeDSecureDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "authentication_flow" => Deserialize::begin(&mut self.authentication_flow),
                "electronic_commerce_indicator" => {
                    Deserialize::begin(&mut self.electronic_commerce_indicator)
                }
                "result" => Deserialize::begin(&mut self.result),
                "result_reason" => Deserialize::begin(&mut self.result_reason),
                "transaction_id" => Deserialize::begin(&mut self.transaction_id),
                "version" => Deserialize::begin(&mut self.version),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                authentication_flow: Deserialize::default(),
                electronic_commerce_indicator: Deserialize::default(),
                result: Deserialize::default(),
                result_reason: Deserialize::default(),
                transaction_id: Deserialize::default(),
                version: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(authentication_flow),
                Some(electronic_commerce_indicator),
                Some(result),
                Some(result_reason),
                Some(transaction_id),
                Some(version),
            ) = (
                self.authentication_flow,
                self.electronic_commerce_indicator,
                self.result,
                self.result_reason,
                self.transaction_id.take(),
                self.version,
            )
            else {
                return None;
            };
            Some(Self::Out {
                authentication_flow,
                electronic_commerce_indicator,
                result,
                result_reason,
                transaction_id,
                version,
            })
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

    impl ObjectDeser for ThreeDSecureDetails {
        type Builder = ThreeDSecureDetailsBuilder;
    }

    impl FromValueOpt for ThreeDSecureDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "authentication_flow" => b.authentication_flow = FromValueOpt::from_value(v),
                    "electronic_commerce_indicator" => {
                        b.electronic_commerce_indicator = FromValueOpt::from_value(v)
                    }
                    "result" => b.result = FromValueOpt::from_value(v),
                    "result_reason" => b.result_reason = FromValueOpt::from_value(v),
                    "transaction_id" => b.transaction_id = FromValueOpt::from_value(v),
                    "version" => b.version = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// For authenticated transactions: how the customer was authenticated by
/// the issuing bank.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsAuthenticationFlow {
    Challenge,
    Frictionless,
}
impl ThreeDSecureDetailsAuthenticationFlow {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsAuthenticationFlow::*;
        match self {
            Challenge => "challenge",
            Frictionless => "frictionless",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsAuthenticationFlow {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsAuthenticationFlow::*;
        match s {
            "challenge" => Ok(Challenge),
            "frictionless" => Ok(Frictionless),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for ThreeDSecureDetailsAuthenticationFlow {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsAuthenticationFlow> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(ThreeDSecureDetailsAuthenticationFlow::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureDetailsAuthenticationFlow);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsAuthenticationFlow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ThreeDSecureDetailsAuthenticationFlow")
        })
    }
}
/// The Electronic Commerce Indicator (ECI). A protocol-level field
/// indicating what degree of authentication was performed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsElectronicCommerceIndicator {
    V01,
    V02,
    V05,
    V06,
    V07,
}
impl ThreeDSecureDetailsElectronicCommerceIndicator {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsElectronicCommerceIndicator::*;
        match self {
            V01 => "01",
            V02 => "02",
            V05 => "05",
            V06 => "06",
            V07 => "07",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsElectronicCommerceIndicator {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsElectronicCommerceIndicator::*;
        match s {
            "01" => Ok(V01),
            "02" => Ok(V02),
            "05" => Ok(V05),
            "06" => Ok(V06),
            "07" => Ok(V07),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsElectronicCommerceIndicator> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ThreeDSecureDetailsElectronicCommerceIndicator::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureDetailsElectronicCommerceIndicator);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ThreeDSecureDetailsElectronicCommerceIndicator",
            )
        })
    }
}
/// Indicates the outcome of 3D Secure authentication.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsResult {
    AttemptAcknowledged,
    Authenticated,
    Exempted,
    Failed,
    NotSupported,
    ProcessingError,
}
impl ThreeDSecureDetailsResult {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsResult::*;
        match self {
            AttemptAcknowledged => "attempt_acknowledged",
            Authenticated => "authenticated",
            Exempted => "exempted",
            Failed => "failed",
            NotSupported => "not_supported",
            ProcessingError => "processing_error",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsResult {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsResult::*;
        match s {
            "attempt_acknowledged" => Ok(AttemptAcknowledged),
            "authenticated" => Ok(Authenticated),
            "exempted" => Ok(Exempted),
            "failed" => Ok(Failed),
            "not_supported" => Ok(NotSupported),
            "processing_error" => Ok(ProcessingError),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for ThreeDSecureDetailsResult {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsResult> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsResult::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureDetailsResult);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureDetailsResult"))
    }
}
/// Additional information about why 3D Secure succeeded or failed based
/// on the `result`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsResultReason {
    Abandoned,
    Bypassed,
    Canceled,
    CardNotEnrolled,
    NetworkNotSupported,
    ProtocolError,
    Rejected,
}
impl ThreeDSecureDetailsResultReason {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsResultReason::*;
        match self {
            Abandoned => "abandoned",
            Bypassed => "bypassed",
            Canceled => "canceled",
            CardNotEnrolled => "card_not_enrolled",
            NetworkNotSupported => "network_not_supported",
            ProtocolError => "protocol_error",
            Rejected => "rejected",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsResultReason {
    type Err = stripe_types::StripeParseError;
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
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for ThreeDSecureDetailsResultReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsResultReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(ThreeDSecureDetailsResultReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureDetailsResultReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsResultReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ThreeDSecureDetailsResultReason")
        })
    }
}
/// The version of 3D Secure that was used.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsVersion {
    V1_0_2,
    V2_1_0,
    V2_2_0,
}
impl ThreeDSecureDetailsVersion {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsVersion::*;
        match self {
            V1_0_2 => "1.0.2",
            V2_1_0 => "2.1.0",
            V2_2_0 => "2.2.0",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsVersion {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsVersion::*;
        match s {
            "1.0.2" => Ok(V1_0_2),
            "2.1.0" => Ok(V2_1_0),
            "2.2.0" => Ok(V2_2_0),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for ThreeDSecureDetailsVersion {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsVersion> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsVersion::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThreeDSecureDetailsVersion);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureDetailsVersion"))
    }
}
