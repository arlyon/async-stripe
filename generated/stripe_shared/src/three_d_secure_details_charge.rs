#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ThreeDSecureDetailsCharge {
    /// For authenticated transactions: how the customer was authenticated by
    /// the issuing bank.
    pub authentication_flow: Option<ThreeDSecureDetailsChargeAuthenticationFlow>,
    /// Indicates the outcome of 3D Secure authentication.
    pub result: Option<ThreeDSecureDetailsChargeResult>,
    /// Additional information about why 3D Secure succeeded or failed based
    /// on the `result`.
    pub result_reason: Option<ThreeDSecureDetailsChargeResultReason>,
    /// The version of 3D Secure that was used.
    pub version: Option<ThreeDSecureDetailsChargeVersion>,
}
#[cfg(feature = "min-ser")]
pub struct ThreeDSecureDetailsChargeBuilder {
    authentication_flow: Option<Option<ThreeDSecureDetailsChargeAuthenticationFlow>>,
    result: Option<Option<ThreeDSecureDetailsChargeResult>>,
    result_reason: Option<Option<ThreeDSecureDetailsChargeResultReason>>,
    version: Option<Option<ThreeDSecureDetailsChargeVersion>>,
}

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ThreeDSecureDetailsCharge {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureDetailsCharge>,
        builder: ThreeDSecureDetailsChargeBuilder,
    }

    impl Visitor for Place<ThreeDSecureDetailsCharge> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: ThreeDSecureDetailsChargeBuilder::deser_default() }))
        }
    }

    impl MapBuilder for ThreeDSecureDetailsChargeBuilder {
        type Out = ThreeDSecureDetailsCharge;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "authentication_flow" => Deserialize::begin(&mut self.authentication_flow),
                "result" => Deserialize::begin(&mut self.result),
                "result_reason" => Deserialize::begin(&mut self.result_reason),
                "version" => Deserialize::begin(&mut self.version),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { authentication_flow: Deserialize::default(), result: Deserialize::default(), result_reason: Deserialize::default(), version: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let authentication_flow = self.authentication_flow.take()?;
            let result = self.result.take()?;
            let result_reason = self.result_reason.take()?;
            let version = self.version.take()?;

            Some(Self::Out { authentication_flow, result, result_reason, version })
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

    impl ObjectDeser for ThreeDSecureDetailsCharge {
        type Builder = ThreeDSecureDetailsChargeBuilder;
    }

    impl FromValueOpt for ThreeDSecureDetailsCharge {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureDetailsChargeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "authentication_flow" => b.authentication_flow = Some(FromValueOpt::from_value(v)?),
                    "result" => b.result = Some(FromValueOpt::from_value(v)?),
                    "result_reason" => b.result_reason = Some(FromValueOpt::from_value(v)?),
                    "version" => b.version = Some(FromValueOpt::from_value(v)?),

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
pub enum ThreeDSecureDetailsChargeAuthenticationFlow {
    Challenge,
    Frictionless,
}
impl ThreeDSecureDetailsChargeAuthenticationFlow {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsChargeAuthenticationFlow::*;
        match self {
            Challenge => "challenge",
            Frictionless => "frictionless",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsChargeAuthenticationFlow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsChargeAuthenticationFlow::*;
        match s {
            "challenge" => Ok(Challenge),
            "frictionless" => Ok(Frictionless),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsChargeAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsChargeAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ThreeDSecureDetailsChargeAuthenticationFlow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsChargeAuthenticationFlow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureDetailsChargeAuthenticationFlow"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecureDetailsChargeAuthenticationFlow {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsChargeAuthenticationFlow> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsChargeAuthenticationFlow::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(ThreeDSecureDetailsChargeAuthenticationFlow);
/// Indicates the outcome of 3D Secure authentication.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsChargeResult {
    AttemptAcknowledged,
    Authenticated,
    Exempted,
    Failed,
    NotSupported,
    ProcessingError,
}
impl ThreeDSecureDetailsChargeResult {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsChargeResult::*;
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

impl std::str::FromStr for ThreeDSecureDetailsChargeResult {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsChargeResult::*;
        match s {
            "attempt_acknowledged" => Ok(AttemptAcknowledged),
            "authenticated" => Ok(Authenticated),
            "exempted" => Ok(Exempted),
            "failed" => Ok(Failed),
            "not_supported" => Ok(NotSupported),
            "processing_error" => Ok(ProcessingError),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsChargeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsChargeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ThreeDSecureDetailsChargeResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsChargeResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureDetailsChargeResult"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecureDetailsChargeResult {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsChargeResult> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsChargeResult::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(ThreeDSecureDetailsChargeResult);
/// Additional information about why 3D Secure succeeded or failed based
/// on the `result`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsChargeResultReason {
    Abandoned,
    Bypassed,
    Canceled,
    CardNotEnrolled,
    NetworkNotSupported,
    ProtocolError,
    Rejected,
}
impl ThreeDSecureDetailsChargeResultReason {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsChargeResultReason::*;
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

impl std::str::FromStr for ThreeDSecureDetailsChargeResultReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsChargeResultReason::*;
        match s {
            "abandoned" => Ok(Abandoned),
            "bypassed" => Ok(Bypassed),
            "canceled" => Ok(Canceled),
            "card_not_enrolled" => Ok(CardNotEnrolled),
            "network_not_supported" => Ok(NetworkNotSupported),
            "protocol_error" => Ok(ProtocolError),
            "rejected" => Ok(Rejected),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsChargeResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsChargeResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ThreeDSecureDetailsChargeResultReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsChargeResultReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureDetailsChargeResultReason"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecureDetailsChargeResultReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsChargeResultReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsChargeResultReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(ThreeDSecureDetailsChargeResultReason);
/// The version of 3D Secure that was used.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsChargeVersion {
    V1_0_2,
    V2_1_0,
    V2_2_0,
}
impl ThreeDSecureDetailsChargeVersion {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsChargeVersion::*;
        match self {
            V1_0_2 => "1.0.2",
            V2_1_0 => "2.1.0",
            V2_2_0 => "2.2.0",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsChargeVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsChargeVersion::*;
        match s {
            "1.0.2" => Ok(V1_0_2),
            "2.1.0" => Ok(V2_1_0),
            "2.2.0" => Ok(V2_2_0),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsChargeVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsChargeVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ThreeDSecureDetailsChargeVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsChargeVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureDetailsChargeVersion"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ThreeDSecureDetailsChargeVersion {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ThreeDSecureDetailsChargeVersion> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ThreeDSecureDetailsChargeVersion::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(ThreeDSecureDetailsChargeVersion);
