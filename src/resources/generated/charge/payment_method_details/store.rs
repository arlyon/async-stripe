#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Store {
    /// The name of the convenience store chain where the payment was completed.
    pub chain: Option<StoreChain>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Store {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The name of the convenience store chain where the payment was completed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum StoreChain {
    Familymart,
    Lawson,
    Ministop,
    Seicomart,
}

impl StoreChain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Familymart => "familymart",
            Self::Lawson => "lawson",
            Self::Ministop => "ministop",
            Self::Seicomart => "seicomart",
        }
    }
}

impl AsRef<str> for StoreChain {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for StoreChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
