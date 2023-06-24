#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerUpdate {
    /// The types of customer updates that are supported.
    ///
    /// When empty, customers are not updateable.
    pub allowed_updates: Vec<CustomerUpdateAllowedUpdates>,
    /// Whether the feature is enabled.
    pub enabled: bool,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerUpdate {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The types of customer updates that are supported.
///
/// When empty, customers are not updateable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomerUpdateAllowedUpdates {
    Address,
    Email,
    Phone,
    Shipping,
    TaxId,
}

impl CustomerUpdateAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Address => "address",
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Shipping => "shipping",
            Self::TaxId => "tax_id",
        }
    }
}

impl std::str::FromStr for CustomerUpdateAllowedUpdates {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "address" => Ok(Self::Address),
            "email" => Ok(Self::Email),
            "phone" => Ok(Self::Phone),
            "shipping" => Ok(Self::Shipping),
            "tax_id" => Ok(Self::TaxId),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomerUpdateAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomerUpdateAllowedUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerUpdateAllowedUpdates {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CustomerUpdateAllowedUpdates"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerUpdateAllowedUpdates {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CustomerUpdateAllowedUpdates> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerUpdateAllowedUpdates::from_str(s)?);
        Ok(())
    }
}
