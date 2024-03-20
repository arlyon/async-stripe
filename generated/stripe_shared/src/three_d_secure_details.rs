#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsAuthenticationFlow::*;
        match s {
            "challenge" => Ok(Challenge),
            "frictionless" => Ok(Frictionless),
            _ => Err(()),
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
impl serde::Serialize for ThreeDSecureDetailsAuthenticationFlow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsElectronicCommerceIndicator::*;
        match s {
            "01" => Ok(V01),
            "02" => Ok(V02),
            "05" => Ok(V05),
            "06" => Ok(V06),
            "07" => Ok(V07),
            _ => Err(()),
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
impl serde::Serialize for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsResult::*;
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
impl serde::Serialize for ThreeDSecureDetailsResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
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
    type Err = ();
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
            _ => Err(()),
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
impl serde::Serialize for ThreeDSecureDetailsResultReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsVersion::*;
        match s {
            "1.0.2" => Ok(V1_0_2),
            "2.1.0" => Ok(V2_1_0),
            "2.2.0" => Ok(V2_2_0),
            _ => Err(()),
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
impl serde::Serialize for ThreeDSecureDetailsVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ThreeDSecureDetailsVersion"))
    }
}
