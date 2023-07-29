#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentData {
    /// Indicates when the funds will be captured from the customer's account.
    pub capture_method: Option<PaymentIntentDataCaptureMethod>,
    /// Indicates that you intend to make future payments with the payment method collected during checkout.
    pub setup_future_usage: Option<PaymentIntentDataSetupFutureUsage>,
}
/// Indicates when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentIntentDataCaptureMethod {
    Automatic,
    Manual,
}

impl PaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr for PaymentIntentDataCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentDataCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PaymentIntentDataCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentDataCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentIntentDataCaptureMethod")
        })
    }
}
/// Indicates that you intend to make future payments with the payment method collected during checkout.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}

impl PaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for PaymentIntentDataSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentDataSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PaymentIntentDataSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentDataSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentIntentDataSetupFutureUsage")
        })
    }
}
