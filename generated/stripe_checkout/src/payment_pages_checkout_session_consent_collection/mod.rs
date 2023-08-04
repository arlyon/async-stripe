#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentPagesCheckoutSessionConsentCollection {
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    ///
    /// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
    /// Only available to US merchants.
    pub promotions: Option<PaymentPagesCheckoutSessionConsentCollectionPromotions>,
    /// If set to `required`, it requires customers to accept the terms of service before being able to pay.
    pub terms_of_service: Option<PaymentPagesCheckoutSessionConsentCollectionTermsOfService>,
}
/// If set to `auto`, enables the collection of customer consent for promotional communications.
///
/// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
/// Only available to US merchants.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionConsentCollectionPromotions {
    Auto,
    None,
}

impl PaymentPagesCheckoutSessionConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionConsentCollectionPromotions::*;
        match self {
            Auto => "auto",
            None => "none",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionConsentCollectionPromotions::*;
        match s {
            "auto" => Ok(Auto),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionConsentCollectionPromotions",
            )
        })
    }
}
/// If set to `required`, it requires customers to accept the terms of service before being able to pay.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    None,
    Required,
}

impl PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionConsentCollectionTermsOfService::*;
        match self {
            None => "none",
            Required => "required",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionConsentCollectionTermsOfService::*;
        match s {
            "none" => Ok(None),
            "required" => Ok(Required),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionConsentCollectionTermsOfService",
            )
        })
    }
}
