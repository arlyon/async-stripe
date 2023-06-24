#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedApplePayDomain {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: stripe_misc::apple_pay_domain::ApplePayDomainId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedApplePayDomainObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedApplePayDomain {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedApplePayDomainObject {
    ApplePayDomain,
}

impl DeletedApplePayDomainObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ApplePayDomain => "apple_pay_domain",
        }
    }
}

impl std::str::FromStr for DeletedApplePayDomainObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "apple_pay_domain" => Ok(Self::ApplePayDomain),

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
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedApplePayDomainObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedApplePayDomainObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<DeletedApplePayDomainObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeletedApplePayDomainObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for DeletedApplePayDomain {
    type Id = stripe_misc::apple_pay_domain::ApplePayDomainId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
