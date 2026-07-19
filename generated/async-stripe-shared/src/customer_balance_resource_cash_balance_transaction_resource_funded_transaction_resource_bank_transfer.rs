#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {
pub eu_bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer>,
pub gb_bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer>,
pub jp_bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer>,
    /// The user-supplied reference field on the bank transfer.
pub reference: Option<String>,
        /// The funding method type used to fund the customer balance.
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
pub type_: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType,
pub us_bank_transfer: Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder {
    eu_bank_transfer: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer>>,
gb_bank_transfer: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer>>,
jp_bank_transfer: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer>>,
reference: Option<Option<String>>,
type_: Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType>,
us_bank_transfer: Option<Option<stripe_shared::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer>>,

}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
       Place::new(out)
    }
}

    struct Builder<'a> {
    out: &'a mut Option<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer>,
    builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder,
}

    impl Visitor for Place<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer> {
    fn map(&mut self) -> Result<Box<dyn Map + '_>> {
        Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferBuilder { eu_bank_transfer: Deserialize::default(),
gb_bank_transfer: Deserialize::default(),
jp_bank_transfer: Deserialize::default(),
reference: Deserialize::default(),
type_: Deserialize::default(),
us_bank_transfer: Deserialize::default(),
 },
        }))
    }
}

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "eu_bank_transfer" => Deserialize::begin(&mut self.builder.eu_bank_transfer),
                "gb_bank_transfer" => Deserialize::begin(&mut self.builder.gb_bank_transfer),
                "jp_bank_transfer" => Deserialize::begin(&mut self.builder.jp_bank_transfer),
                "reference" => Deserialize::begin(&mut self.builder.reference),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "us_bank_transfer" => Deserialize::begin(&mut self.builder.us_bank_transfer),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(eu_bank_transfer),
                Some(gb_bank_transfer),
                Some(jp_bank_transfer),
                Some(reference),
                Some(type_),
                Some(us_bank_transfer),
            ) = (
                self.builder.eu_bank_transfer.take(),
                self.builder.gb_bank_transfer.take(),
                self.builder.jp_bank_transfer.take(),
                self.builder.reference.take(),
                self.builder.type_.take(),
                self.builder.us_bank_transfer.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer { eu_bank_transfer,gb_bank_transfer,jp_bank_transfer,reference,type_,us_bank_transfer });
            Ok(())
        }
    }
};
/// The funding method type used to fund the customer balance.
/// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType
{
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl
    CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType
{
    pub fn as_str(&self) -> &str {
        use CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            GbBankTransfer => "gb_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
            MxBankTransfer => "mx_bank_transfer",
            UsBankTransfer => "us_bank_transfer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::*;
        match s {
    "eu_bank_transfer" => Ok(EuBankTransfer),
"gb_bank_transfer" => Ok(GbBankTransfer),
"jp_bank_transfer" => Ok(JpBankTransfer),
"mx_bank_transfer" => Ok(MxBankTransfer),
"us_bank_transfer" => Ok(UsBankTransfer),
v => { tracing::warn!("Unknown value '{}' for enum '{}'", v, "CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType"); Ok(Unknown(v.to_owned())) }

        }
    }
}
impl std::fmt::Display for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
