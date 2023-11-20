#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalFlowsRetention {
    /// Configuration when `retention.type=coupon_offer`.
    pub coupon_offer: Option<stripe_billing::PortalFlowsCouponOffer>,
    /// Type of retention strategy that will be used.
    #[serde(rename = "type")]
    pub type_: PortalFlowsRetentionType,
}
/// Type of retention strategy that will be used.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalFlowsRetentionType {
    CouponOffer,
}

impl PortalFlowsRetentionType {
    pub fn as_str(self) -> &'static str {
        use PortalFlowsRetentionType::*;
        match self {
            CouponOffer => "coupon_offer",
        }
    }
}

impl std::str::FromStr for PortalFlowsRetentionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalFlowsRetentionType::*;
        match s {
            "coupon_offer" => Ok(CouponOffer),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PortalFlowsRetentionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalFlowsRetentionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalFlowsRetentionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PortalFlowsRetentionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PortalFlowsRetentionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PortalFlowsRetentionType"))
    }
}
