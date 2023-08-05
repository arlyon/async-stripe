#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsKonbiniStore {
    /// The name of the convenience store chain where the payment was completed.
    pub chain: Option<PaymentMethodDetailsKonbiniStoreChain>,
}
/// The name of the convenience store chain where the payment was completed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsKonbiniStoreChain {
    Familymart,
    Lawson,
    Ministop,
    Seicomart,
}

impl PaymentMethodDetailsKonbiniStoreChain {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsKonbiniStoreChain::*;
        match self {
            Familymart => "familymart",
            Lawson => "lawson",
            Ministop => "ministop",
            Seicomart => "seicomart",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsKonbiniStoreChain {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsKonbiniStoreChain::*;
        match s {
            "familymart" => Ok(Familymart),
            "lawson" => Ok(Lawson),
            "ministop" => Ok(Ministop),
            "seicomart" => Ok(Seicomart),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsKonbiniStoreChain {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsKonbiniStoreChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsKonbiniStoreChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDetailsKonbiniStoreChain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsKonbiniStoreChain {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentMethodDetailsKonbiniStoreChain")
        })
    }
}
