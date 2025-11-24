#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCustomerBalanceBankTransfer {
    pub eu_bank_transfer: Option<stripe_shared::PaymentMethodOptionsCustomerBalanceEuBankAccount>,
    /// List of address types that should be returned in the financial_addresses response.
    /// If not specified, all valid types will be returned.
    ///
    /// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    pub requested_address_types:
        Option<Vec<PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes>>,
    /// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<PaymentMethodOptionsCustomerBalanceBankTransferType>,
}
#[doc(hidden)]
pub struct PaymentMethodOptionsCustomerBalanceBankTransferBuilder {
    eu_bank_transfer:
        Option<Option<stripe_shared::PaymentMethodOptionsCustomerBalanceEuBankAccount>>,
    requested_address_types:
        Option<Option<Vec<PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes>>>,
    type_: Option<Option<PaymentMethodOptionsCustomerBalanceBankTransferType>>,
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

    impl Deserialize for PaymentMethodOptionsCustomerBalanceBankTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsCustomerBalanceBankTransfer>,
        builder: PaymentMethodOptionsCustomerBalanceBankTransferBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsCustomerBalanceBankTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsCustomerBalanceBankTransferBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsCustomerBalanceBankTransferBuilder {
        type Out = PaymentMethodOptionsCustomerBalanceBankTransfer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "eu_bank_transfer" => Deserialize::begin(&mut self.eu_bank_transfer),
                "requested_address_types" => Deserialize::begin(&mut self.requested_address_types),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                eu_bank_transfer: Deserialize::default(),
                requested_address_types: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(eu_bank_transfer), Some(requested_address_types), Some(type_)) = (
                self.eu_bank_transfer.take(),
                self.requested_address_types.take(),
                self.type_.take(),
            ) else {
                return None;
            };
            Some(Self::Out { eu_bank_transfer, requested_address_types, type_ })
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

    impl ObjectDeser for PaymentMethodOptionsCustomerBalanceBankTransfer {
        type Builder = PaymentMethodOptionsCustomerBalanceBankTransferBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsCustomerBalanceBankTransfer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsCustomerBalanceBankTransferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "eu_bank_transfer" => b.eu_bank_transfer = FromValueOpt::from_value(v),
                    "requested_address_types" => {
                        b.requested_address_types = FromValueOpt::from_value(v)
                    }
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// List of address types that should be returned in the financial_addresses response.
/// If not specified, all valid types will be returned.
///
/// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    Aba,
    Iban,
    Sepa,
    SortCode,
    Spei,
    Swift,
    Zengin,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    pub fn as_str(&self) -> &str {
        use PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::*;
        match self {
            Aba => "aba",
            Iban => "iban",
            Sepa => "sepa",
            SortCode => "sort_code",
            Spei => "spei",
            Swift => "swift",
            Zengin => "zengin",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::*;
        match s {
            "aba" => Ok(Aba),
            "iban" => Ok(Iban),
            "sepa" => Ok(Sepa),
            "sort_code" => Ok(SortCode),
            "spei" => Ok(Spei),
            "swift" => Ok(Swift),
            "zengin" => Ok(Zengin),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(&self) -> &str {
        use PaymentMethodOptionsCustomerBalanceBankTransferType::*;
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

impl std::str::FromStr for PaymentMethodOptionsCustomerBalanceBankTransferType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCustomerBalanceBankTransferType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "gb_bank_transfer" => Ok(GbBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            "mx_bank_transfer" => Ok(MxBankTransfer),
            "us_bank_transfer" => Ok(UsBankTransfer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodOptionsCustomerBalanceBankTransferType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsCustomerBalanceBankTransferType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodOptionsCustomerBalanceBankTransferType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsCustomerBalanceBankTransferType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCustomerBalanceBankTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
