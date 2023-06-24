#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Card {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<
        stripe_core::subscription::payment_method_options::card::mandate_options::MandateOptions,
    >,
    /// Selected network to process this Subscription on.
    ///
    /// Depends on the available networks of the card attached to the Subscription.
    /// Can be only set confirm-time.
    pub network: Option<CardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<CardRequestThreeDSecure>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Card {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Selected network to process this Subscription on.
///
/// Depends on the available networks of the card attached to the Subscription.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl CardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Amex => "amex",
            Self::CartesBancaires => "cartes_bancaires",
            Self::Diners => "diners",
            Self::Discover => "discover",
            Self::Interac => "interac",
            Self::Jcb => "jcb",
            Self::Mastercard => "mastercard",
            Self::Unionpay => "unionpay",
            Self::Unknown => "unknown",
            Self::Visa => "visa",
        }
    }
}

impl std::str::FromStr for CardNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amex" => Ok(Self::Amex),
            "cartes_bancaires" => Ok(Self::CartesBancaires),
            "diners" => Ok(Self::Diners),
            "discover" => Ok(Self::Discover),
            "interac" => Ok(Self::Interac),
            "jcb" => Ok(Self::Jcb),
            "mastercard" => Ok(Self::Mastercard),
            "unionpay" => Ok(Self::Unionpay),
            "unknown" => Ok(Self::Unknown),
            "visa" => Ok(Self::Visa),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CardNetwork"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CardNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CardNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CardNetwork::from_str(s)?);
        Ok(())
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardRequestThreeDSecure {
    Any,
    Automatic,
}

impl CardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Automatic => "automatic",
        }
    }
}

impl std::str::FromStr for CardRequestThreeDSecure {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "any" => Ok(Self::Any),
            "automatic" => Ok(Self::Automatic),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CardRequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CardRequestThreeDSecure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CardRequestThreeDSecure"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CardRequestThreeDSecure {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CardRequestThreeDSecure> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CardRequestThreeDSecure::from_str(s)?);
        Ok(())
    }
}
pub mod mandate_options;
pub use mandate_options::MandateOptions;
