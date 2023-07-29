#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApplePayDomain {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub domain_name: String,
    /// Unique identifier for the object.
    pub id: stripe_misc::apple_pay_domain::ApplePayDomainId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ApplePayDomainObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ApplePayDomainObject {
    ApplePayDomain,
}

impl ApplePayDomainObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ApplePayDomain => "apple_pay_domain",
        }
    }
}

impl std::str::FromStr for ApplePayDomainObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "apple_pay_domain" => Ok(Self::ApplePayDomain),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ApplePayDomainObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ApplePayDomainObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ApplePayDomainObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ApplePayDomainObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ApplePayDomainObject"))
    }
}
impl stripe_types::Object for ApplePayDomain {
    type Id = stripe_misc::apple_pay_domain::ApplePayDomainId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ApplePayDomainId);
pub mod deleted;
pub use deleted::DeletedApplePayDomain;
