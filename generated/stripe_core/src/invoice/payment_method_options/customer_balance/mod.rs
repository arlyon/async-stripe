#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerBalance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<
        stripe_core::invoice::payment_method_options::customer_balance::bank_transfer::BankTransfer,
    >,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    pub funding_type: Option<CustomerBalanceFundingType>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerBalance {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The funding method type to be used when there are not enough funds in the customer balance.
///
/// Permitted values include: `bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomerBalanceFundingType {
    BankTransfer,
}

impl CustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr for CustomerBalanceFundingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bank_transfer" => Ok(Self::BankTransfer),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomerBalanceFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerBalanceFundingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CustomerBalanceFundingType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerBalanceFundingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CustomerBalanceFundingType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerBalanceFundingType::from_str(s)?);
        Ok(())
    }
}
pub mod bank_transfer;
pub use bank_transfer::BankTransfer;
