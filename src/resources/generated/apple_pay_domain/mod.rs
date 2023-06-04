#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ApplePayDomain {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    pub domain_name: String,
    /// Unique identifier for the object.
    pub id: crate::apple_pay_domain::ApplePayDomainId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ApplePayDomainObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ApplePayDomain {
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
impl crate::Object for ApplePayDomain {
    type Id = crate::apple_pay_domain::ApplePayDomainId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(ApplePayDomainId);
pub mod deleted;
pub mod requests;
pub use deleted::DeletedApplePayDomain;
