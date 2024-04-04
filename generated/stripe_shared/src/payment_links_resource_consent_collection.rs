#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceConsentCollection {
    /// Settings related to the payment method reuse text shown in the Checkout UI.
    pub payment_method_reuse_agreement:
        Option<stripe_shared::PaymentLinksResourcePaymentMethodReuseAgreement>,
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    pub promotions: Option<PaymentLinksResourceConsentCollectionPromotions>,
    /// If set to `required`, it requires cutomers to accept the terms of service before being able to pay.
    /// If set to `none`, customers won't be shown a checkbox to accept the terms of service.
    pub terms_of_service: Option<PaymentLinksResourceConsentCollectionTermsOfService>,
}
/// If set to `auto`, enables the collection of customer consent for promotional communications.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourceConsentCollectionPromotions {
    Auto,
    None,
}
impl PaymentLinksResourceConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourceConsentCollectionPromotions::*;
        match self {
            Auto => "auto",
            None => "none",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceConsentCollectionPromotions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceConsentCollectionPromotions::*;
        match s {
            "auto" => Ok(Auto),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for PaymentLinksResourceConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourceConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourceConsentCollectionPromotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceConsentCollectionPromotions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentLinksResourceConsentCollectionPromotions",
            )
        })
    }
}
/// If set to `required`, it requires cutomers to accept the terms of service before being able to pay.
/// If set to `none`, customers won't be shown a checkbox to accept the terms of service.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourceConsentCollectionTermsOfService {
    None,
    Required,
}
impl PaymentLinksResourceConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourceConsentCollectionTermsOfService::*;
        match self {
            None => "none",
            Required => "required",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceConsentCollectionTermsOfService {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceConsentCollectionTermsOfService::*;
        match s {
            "none" => Ok(None),
            "required" => Ok(Required),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for PaymentLinksResourceConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourceConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourceConsentCollectionTermsOfService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceConsentCollectionTermsOfService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentLinksResourceConsentCollectionTermsOfService",
            )
        })
    }
}
