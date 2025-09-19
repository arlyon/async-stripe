#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FundingInstructionsBankTransfer {
    /// The country of the bank account to fund
    pub country: String,
    /// A list of financial addresses that can be used to fund a particular balance
    pub financial_addresses: Vec<stripe_shared::FundingInstructionsBankTransferFinancialAddress>,
    /// The bank_transfer type
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: FundingInstructionsBankTransferType,
}
#[doc(hidden)]
pub struct FundingInstructionsBankTransferBuilder {
    country: Option<String>,
    financial_addresses:
        Option<Vec<stripe_shared::FundingInstructionsBankTransferFinancialAddress>>,
    type_: Option<FundingInstructionsBankTransferType>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for FundingInstructionsBankTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FundingInstructionsBankTransfer>,
        builder: FundingInstructionsBankTransferBuilder,
    }

    impl Visitor for Place<FundingInstructionsBankTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FundingInstructionsBankTransferBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FundingInstructionsBankTransferBuilder {
        type Out = FundingInstructionsBankTransfer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "country" => Deserialize::begin(&mut self.country),
                "financial_addresses" => Deserialize::begin(&mut self.financial_addresses),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                country: Deserialize::default(),
                financial_addresses: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(country), Some(financial_addresses), Some(type_)) =
                (self.country.take(), self.financial_addresses.take(), self.type_)
            else {
                return None;
            };
            Some(Self::Out { country, financial_addresses, type_ })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for FundingInstructionsBankTransfer {
        type Builder = FundingInstructionsBankTransferBuilder;
    }

    impl FromValueOpt for FundingInstructionsBankTransfer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FundingInstructionsBankTransferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "country" => b.country = FromValueOpt::from_value(v),
                    "financial_addresses" => b.financial_addresses = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The bank_transfer type
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FundingInstructionsBankTransferType {
    EuBankTransfer,
    JpBankTransfer,
}
impl FundingInstructionsBankTransferType {
    pub fn as_str(self) -> &'static str {
        use FundingInstructionsBankTransferType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
        }
    }
}

impl std::str::FromStr for FundingInstructionsBankTransferType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingInstructionsBankTransferType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for FundingInstructionsBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FundingInstructionsBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FundingInstructionsBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for FundingInstructionsBankTransferType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FundingInstructionsBankTransferType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(FundingInstructionsBankTransferType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FundingInstructionsBankTransferType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FundingInstructionsBankTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FundingInstructionsBankTransferType")
        })
    }
}
