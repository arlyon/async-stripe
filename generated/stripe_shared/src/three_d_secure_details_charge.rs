#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ThreeDSecureDetailsCharge {
    /// For authenticated transactions: how the customer was authenticated by
    /// the issuing bank.
    pub authentication_flow: Option<ThreeDSecureDetailsChargeAuthenticationFlow>,
    /// The Electronic Commerce Indicator (ECI). A protocol-level field
    /// indicating what degree of authentication was performed.
    pub electronic_commerce_indicator: Option<ThreeDSecureDetailsChargeElectronicCommerceIndicator>,
    /// The exemption requested via 3DS and accepted by the issuer at authentication time.
    pub exemption_indicator: Option<ThreeDSecureDetailsChargeExemptionIndicator>,
    /// Whether Stripe requested the value of `exemption_indicator` in the transaction. This will depend on
    /// the outcome of Stripe's internal risk assessment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_indicator_applied: Option<bool>,
    /// Indicates the outcome of 3D Secure authentication.
    pub result: Option<ThreeDSecureDetailsChargeResult>,
    /// Additional information about why 3D Secure succeeded or failed based
    /// on the `result`.
    pub result_reason: Option<ThreeDSecureDetailsChargeResultReason>,
    /// The 3D Secure 1 XID or 3D Secure 2 Directory Server Transaction ID
    /// (dsTransId) for this payment.
    pub transaction_id: Option<String>,
    /// The version of 3D Secure that was used.
    pub version: Option<ThreeDSecureDetailsChargeVersion>,
}
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ThreeDSecureDetailsChargeAuthenticationFlow",
            )
        })
    }
}
/// The Electronic Commerce Indicator (ECI). A protocol-level field
/// indicating what degree of authentication was performed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    V01,
    V02,
    V05,
    V06,
    V07,
}
impl ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsChargeElectronicCommerceIndicator::*;
        match self {
            V01 => "01",
            V02 => "02",
            V05 => "05",
            V06 => "06",
            V07 => "07",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsChargeElectronicCommerceIndicator::*;
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
impl std::fmt::Display for ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ThreeDSecureDetailsChargeElectronicCommerceIndicator",
            )
        })
    }
}
/// The exemption requested via 3DS and accepted by the issuer at authentication time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThreeDSecureDetailsChargeExemptionIndicator {
    LowRisk,
    None,
}
impl ThreeDSecureDetailsChargeExemptionIndicator {
    pub fn as_str(self) -> &'static str {
        use ThreeDSecureDetailsChargeExemptionIndicator::*;
        match self {
            LowRisk => "low_risk",
            None => "none",
        }
    }
}

impl std::str::FromStr for ThreeDSecureDetailsChargeExemptionIndicator {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThreeDSecureDetailsChargeExemptionIndicator::*;
        match s {
            "low_risk" => Ok(LowRisk),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ThreeDSecureDetailsChargeExemptionIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThreeDSecureDetailsChargeExemptionIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ThreeDSecureDetailsChargeExemptionIndicator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ThreeDSecureDetailsChargeExemptionIndicator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ThreeDSecureDetailsChargeExemptionIndicator",
            )
        })
    }
}
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ThreeDSecureDetailsChargeResult")
        })
    }
}
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ThreeDSecureDetailsChargeResultReason")
        })
    }
}
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ThreeDSecureDetailsChargeVersion")
        })
    }
}
