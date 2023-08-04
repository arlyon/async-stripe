#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsCardInstallmentsPlan {
    /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    pub count: Option<u64>,
    /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
    /// One of `month`.
    pub interval: Option<PaymentMethodDetailsCardInstallmentsPlanInterval>,
    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: PaymentMethodDetailsCardInstallmentsPlanType,
}
/// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
/// One of `month`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsCardInstallmentsPlanInterval {
    Month,
}

impl PaymentMethodDetailsCardInstallmentsPlanInterval {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsCardInstallmentsPlanInterval::*;
        match self {
            Month => "month",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCardInstallmentsPlanInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCardInstallmentsPlanInterval::*;
        match s {
            "month" => Ok(Month),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCardInstallmentsPlanInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodDetailsCardInstallmentsPlanInterval",
            )
        })
    }
}
/// Type of installment plan, one of `fixed_count`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsCardInstallmentsPlanType {
    FixedCount,
}

impl PaymentMethodDetailsCardInstallmentsPlanType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsCardInstallmentsPlanType::*;
        match self {
            FixedCount => "fixed_count",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCardInstallmentsPlanType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCardInstallmentsPlanType::*;
        match s {
            "fixed_count" => Ok(FixedCount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardInstallmentsPlanType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardInstallmentsPlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsCardInstallmentsPlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodDetailsCardInstallmentsPlanType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCardInstallmentsPlanType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodDetailsCardInstallmentsPlanType",
            )
        })
    }
}
