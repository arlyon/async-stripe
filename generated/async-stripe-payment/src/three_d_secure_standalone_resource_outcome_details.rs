/// Contains details on the result for a standalone 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureStandaloneResourceOutcomeDetails {
    /// Universally unique transaction identifier assigned by the issuer to identify the transaction.
    pub acs_transaction_id: Option<String>,
    /// The Authentication Response Message (ARes) is the issuer's response to the AReq message.
    pub ares: Option<String>,
    /// TransStatus field on the ARes
    pub ares_trans_status: Option<ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus>,
    /// A 28-character Base64 string proving that 3DS was completed.
    /// Store this value securely, and don’t reuse it for multiple authorizations.
    pub cryptogram: Option<String>,
    /// The 3DS2 Directory Server Transaction ID.
    pub ds_transaction_id: Option<String>,
    /// Electronic Commerce Indicator provided by the issuer to indicate the result of this 3DS Authentication.
    pub eci: Option<String>,
    pub network_details: Option<stripe_payment::ThreeDSecureStandaloneResourceNetworkDetails>,
    /// The 3DS protocol version used for this 3DS Authentication.
    pub protocol_version: ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion,
    /// The indicator provided to the issuer by Stripe in the AReq that indicates whether a challenge is requested for this Authentication.
    /// This indicator should match the flow_preference you specified but may be overridden (for compliance reasons for example).
    pub requestor_challenge_indicator:
        Option<ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator>,
    /// The Results Request Message (RReq) communicates the results of the authentication or verification.
    pub rreq: Option<String>,
    /// TransStatus field on the RReq
    pub rreq_trans_status: Option<ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus>,
    /// Universally unique transaction identifier assigned by Stripe to identify the transaction.
    pub three_ds_server_transaction_id: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceOutcomeDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThreeDSecureStandaloneResourceOutcomeDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThreeDSecureStandaloneResourceOutcomeDetailsBuilder {
    acs_transaction_id: Option<Option<String>>,
    ares: Option<Option<String>>,
    ares_trans_status: Option<Option<ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus>>,
    cryptogram: Option<Option<String>>,
    ds_transaction_id: Option<Option<String>>,
    eci: Option<Option<String>>,
    network_details: Option<Option<stripe_payment::ThreeDSecureStandaloneResourceNetworkDetails>>,
    protocol_version: Option<ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion>,
    requestor_challenge_indicator:
        Option<Option<ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator>>,
    rreq: Option<Option<String>>,
    rreq_trans_status: Option<Option<ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus>>,
    three_ds_server_transaction_id: Option<String>,
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

    impl Deserialize for ThreeDSecureStandaloneResourceOutcomeDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureStandaloneResourceOutcomeDetails>,
        builder: ThreeDSecureStandaloneResourceOutcomeDetailsBuilder,
    }

    impl Visitor for Place<ThreeDSecureStandaloneResourceOutcomeDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureStandaloneResourceOutcomeDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureStandaloneResourceOutcomeDetailsBuilder {
        type Out = ThreeDSecureStandaloneResourceOutcomeDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acs_transaction_id" => Deserialize::begin(&mut self.acs_transaction_id),
                "ares" => Deserialize::begin(&mut self.ares),
                "ares_trans_status" => Deserialize::begin(&mut self.ares_trans_status),
                "cryptogram" => Deserialize::begin(&mut self.cryptogram),
                "ds_transaction_id" => Deserialize::begin(&mut self.ds_transaction_id),
                "eci" => Deserialize::begin(&mut self.eci),
                "network_details" => Deserialize::begin(&mut self.network_details),
                "protocol_version" => Deserialize::begin(&mut self.protocol_version),
                "requestor_challenge_indicator" => {
                    Deserialize::begin(&mut self.requestor_challenge_indicator)
                }
                "rreq" => Deserialize::begin(&mut self.rreq),
                "rreq_trans_status" => Deserialize::begin(&mut self.rreq_trans_status),
                "three_ds_server_transaction_id" => {
                    Deserialize::begin(&mut self.three_ds_server_transaction_id)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acs_transaction_id: Some(None),
                ares: Some(None),
                ares_trans_status: Some(None),
                cryptogram: Some(None),
                ds_transaction_id: Some(None),
                eci: Some(None),
                network_details: Some(None),
                protocol_version: None,
                requestor_challenge_indicator: Some(None),
                rreq: Some(None),
                rreq_trans_status: Some(None),
                three_ds_server_transaction_id: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(acs_transaction_id),
                Some(ares),
                Some(ares_trans_status),
                Some(cryptogram),
                Some(ds_transaction_id),
                Some(eci),
                Some(network_details),
                Some(protocol_version),
                Some(requestor_challenge_indicator),
                Some(rreq),
                Some(rreq_trans_status),
                Some(three_ds_server_transaction_id),
            ) = (
                self.acs_transaction_id.take(),
                self.ares.take(),
                self.ares_trans_status.take(),
                self.cryptogram.take(),
                self.ds_transaction_id.take(),
                self.eci.take(),
                self.network_details.take(),
                self.protocol_version.take(),
                self.requestor_challenge_indicator.take(),
                self.rreq.take(),
                self.rreq_trans_status.take(),
                self.three_ds_server_transaction_id.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                acs_transaction_id,
                ares,
                ares_trans_status,
                cryptogram,
                ds_transaction_id,
                eci,
                network_details,
                protocol_version,
                requestor_challenge_indicator,
                rreq,
                rreq_trans_status,
                three_ds_server_transaction_id,
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

    impl ObjectDeser for ThreeDSecureStandaloneResourceOutcomeDetails {
        type Builder = ThreeDSecureStandaloneResourceOutcomeDetailsBuilder;
    }

    impl FromValueOpt for ThreeDSecureStandaloneResourceOutcomeDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureStandaloneResourceOutcomeDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acs_transaction_id" => b.acs_transaction_id = FromValueOpt::from_value(v),
                    "ares" => b.ares = FromValueOpt::from_value(v),
                    "ares_trans_status" => b.ares_trans_status = FromValueOpt::from_value(v),
                    "cryptogram" => b.cryptogram = FromValueOpt::from_value(v),
                    "ds_transaction_id" => b.ds_transaction_id = FromValueOpt::from_value(v),
                    "eci" => b.eci = FromValueOpt::from_value(v),
                    "network_details" => b.network_details = FromValueOpt::from_value(v),
                    "protocol_version" => b.protocol_version = FromValueOpt::from_value(v),
                    "requestor_challenge_indicator" => {
                        b.requestor_challenge_indicator = FromValueOpt::from_value(v)
                    }
                    "rreq" => b.rreq = FromValueOpt::from_value(v),
                    "rreq_trans_status" => b.rreq_trans_status = FromValueOpt::from_value(v),
                    "three_ds_server_transaction_id" => {
                        b.three_ds_server_transaction_id = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// TransStatus field on the ARes
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus {
    A,
    C,
    D,
    I,
    N,
    R,
    S,
    U,
    Y,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::*;
        match self {
            A => "A",
            C => "C",
            D => "D",
            I => "I",
            N => "N",
            R => "R",
            S => "S",
            U => "U",
            Y => "Y",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::*;
        match s {
            "A" => Ok(A),
            "C" => Ok(C),
            "D" => Ok(D),
            "I" => Ok(I),
            "N" => Ok(N),
            "R" => Ok(R),
            "S" => Ok(S),
            "U" => Ok(U),
            "Y" => Ok(Y),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The 3DS protocol version used for this 3DS Authentication.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    V2_1_0,
    V2_2_0,
    V2_3_1,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::*;
        match self {
            V2_1_0 => "2.1.0",
            V2_2_0 => "2.2.0",
            V2_3_1 => "2.3.1",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::*;
        match s {
            "2.1.0" => Ok(V2_1_0),
            "2.2.0" => Ok(V2_2_0),
            "2.3.1" => Ok(V2_3_1),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The indicator provided to the issuer by Stripe in the AReq that indicates whether a challenge is requested for this Authentication.
/// This indicator should match the flow_preference you specified but may be overridden (for compliance reasons for example).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator {
    V01,
    V02,
    V03,
    V04,
    V05,
    V06,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::*;
        match self {
            V01 => "01",
            V02 => "02",
            V03 => "03",
            V04 => "04",
            V05 => "05",
            V06 => "06",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::*;
        match s {
            "01" => Ok(V01),
            "02" => Ok(V02),
            "03" => Ok(V03),
            "04" => Ok(V04),
            "05" => Ok(V05),
            "06" => Ok(V06),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// TransStatus field on the RReq
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus {
    A,
    C,
    D,
    I,
    N,
    R,
    S,
    U,
    Y,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus {
    pub fn as_str(&self) -> &str {
        use ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus::*;
        match self {
            A => "A",
            C => "C",
            D => "D",
            I => "I",
            N => "N",
            R => "R",
            S => "S",
            U => "U",
            Y => "Y",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus::*;
        match s {
            "A" => Ok(A),
            "C" => Ok(C),
            "D" => Ok(D),
            "I" => Ok(I),
            "N" => Ok(N),
            "R" => Ok(R),
            "S" => Ok(S),
            "U" => Ok(U),
            "Y" => Ok(Y),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThreeDSecureStandaloneResourceOutcomeDetailsRreqTransStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
