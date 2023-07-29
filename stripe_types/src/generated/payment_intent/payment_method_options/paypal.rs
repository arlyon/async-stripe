#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Paypal {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<PaypalCaptureMethod>,
    /// Preferred locale of the PayPal checkout page that the customer is redirected to.
    pub preferred_locale: Option<String>,
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaypalCaptureMethod {
    Manual,
}

impl PaypalCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr for PaypalCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "manual" => Ok(Self::Manual),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaypalCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaypalCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PaypalCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaypalCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaypalCaptureMethod"))
    }
}
