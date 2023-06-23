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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl stripe_types::Object for DeletedApplePayDomain {
    type Id = stripe_misc::apple_pay_domain::ApplePayDomainId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
