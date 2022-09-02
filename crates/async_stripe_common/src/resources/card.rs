use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum CardBrand {
    #[serde(rename = "American Express")]
    AmericanExpress,
    #[serde(rename = "Diners Club")]
    DinersClub,
    #[serde(rename = "Discover")]
    Discover,
    #[serde(rename = "JCB")]
    JCB,
    #[serde(rename = "Visa")]
    Visa,
    #[serde(rename = "MasterCard")]
    MasterCard,
    #[serde(rename = "UnionPay")]
    UnionPay,

    /// An unknown card brand.
    ///
    /// May also be a variant not yet supported by the library.
    #[serde(other)]
    #[serde(rename = "Unknown")]
    Unknown,
}

impl std::default::Default for CardBrand {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum CardType {
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "prepaid")]
    Prepaid,

    /// An unknown card type.
    ///
    /// May also be a variant not yet supported by the library.
    #[serde(other)]
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::default::Default for CardType {
    fn default() -> Self {
        Self::Unknown
    }
}
