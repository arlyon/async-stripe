#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedCustomer {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::customer::CustomerId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedCustomerObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedCustomer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedCustomerObject {
    Customer,
}

impl DeletedCustomerObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Customer => "customer",
        }
    }
}

impl std::str::FromStr for DeletedCustomerObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "customer" => Ok(Self::Customer),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedCustomerObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedCustomerObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedCustomerObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedCustomerObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedCustomerObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedCustomerObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<DeletedCustomerObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeletedCustomerObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for DeletedCustomer {
    type Id = stripe_types::customer::CustomerId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
