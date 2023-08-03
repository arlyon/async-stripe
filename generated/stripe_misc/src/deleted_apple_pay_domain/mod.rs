#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedApplePayDomain {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_misc::apple_pay_domain::ApplePayDomainId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedApplePayDomainObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DeletedApplePayDomainObject {
    ApplePayDomain,
}

impl DeletedApplePayDomainObject {
    pub fn as_str(self) -> &'static str {
        use DeletedApplePayDomainObject::*;
        match self {
            ApplePayDomain => "apple_pay_domain",
        }
    }
}

impl std::str::FromStr for DeletedApplePayDomainObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeletedApplePayDomainObject::*;
        match s {
            "apple_pay_domain" => Ok(ApplePayDomain),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedApplePayDomainObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedApplePayDomainObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DeletedApplePayDomainObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for DeletedApplePayDomainObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedApplePayDomainObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for DeletedApplePayDomainObject"))
    }
}
impl stripe_types::Object for DeletedApplePayDomain {
    type Id = stripe_misc::apple_pay_domain::ApplePayDomainId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
