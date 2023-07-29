#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ConsentCollection {
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    ///
    /// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
    /// Only available to US merchants.
    pub promotions: Option<ConsentCollectionPromotions>,
    /// If set to `required`, it requires customers to accept the terms of service before being able to pay.
    pub terms_of_service: Option<ConsentCollectionTermsOfService>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ConsentCollection {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// If set to `auto`, enables the collection of customer consent for promotional communications.
///
/// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
/// Only available to US merchants.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ConsentCollectionPromotions {
    Auto,
    None,
}

impl ConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for ConsentCollectionPromotions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "none" => Ok(Self::None),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ConsentCollectionPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ConsentCollectionPromotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConsentCollectionPromotions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ConsentCollectionPromotions"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ConsentCollectionPromotions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ConsentCollectionPromotions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ConsentCollectionPromotions::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// If set to `required`, it requires customers to accept the terms of service before being able to pay.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ConsentCollectionTermsOfService {
    None,
    Required,
}

impl ConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Required => "required",
        }
    }
}

impl std::str::FromStr for ConsentCollectionTermsOfService {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "required" => Ok(Self::Required),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ConsentCollectionTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ConsentCollectionTermsOfService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConsentCollectionTermsOfService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ConsentCollectionTermsOfService")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ConsentCollectionTermsOfService {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<ConsentCollectionTermsOfService> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(ConsentCollectionTermsOfService::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
