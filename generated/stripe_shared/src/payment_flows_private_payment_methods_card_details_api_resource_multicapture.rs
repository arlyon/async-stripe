#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
    /// Indicates whether or not multiple captures are supported.
    pub status: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus,
}
/// Indicates whether or not multiple captures are supported.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus {
    Available,
    Unavailable,
}
impl PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus {
    pub fn as_str(self) -> &'static str {
        use PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus::*;
        match self {
            Available => "available",
            Unavailable => "unavailable",
        }
    }
}

impl std::str::FromStr
    for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus::*;
        match s {
            "available" => Ok(Available),
            "unavailable" => Ok(Unavailable),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display
    for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus"))
    }
}
