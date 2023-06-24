#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FinancialAccount {
    /// The FinancialAccount ID.
    pub id: String,
    /// The rails the ReceivedCredit was sent over.
    ///
    /// A FinancialAccount can only send funds over `stripe`.
    pub network: FinancialAccountNetwork,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialAccount {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The rails the ReceivedCredit was sent over.
///
/// A FinancialAccount can only send funds over `stripe`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FinancialAccountNetwork {
    Stripe,
}

impl FinancialAccountNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for FinancialAccountNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stripe" => Ok(Self::Stripe),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for FinancialAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for FinancialAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for FinancialAccountNetwork"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialAccountNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<FinancialAccountNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialAccountNetwork::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for FinancialAccount {
    type Id = String;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
