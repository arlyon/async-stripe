#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    pub eu_bank_transfer: Option<stripe_shared::PaymentMethodOptionsCustomerBalanceEuBankAccount>,
    /// List of address types that should be returned in the financial_addresses response.
    /// If not specified, all valid types will be returned.
    ///
    /// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    pub requested_address_types:
        Option<Vec<CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes>>,
    /// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType>,
}
#[doc(hidden)]
pub struct CheckoutCustomerBalanceBankTransferPaymentMethodOptionsBuilder {
    eu_bank_transfer:
        Option<Option<stripe_shared::PaymentMethodOptionsCustomerBalanceEuBankAccount>>,
    requested_address_types: Option<
        Option<Vec<CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes>>,
    >,
    type_: Option<Option<CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutCustomerBalanceBankTransferPaymentMethodOptions>,
        builder: CheckoutCustomerBalanceBankTransferPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutCustomerBalanceBankTransferPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    CheckoutCustomerBalanceBankTransferPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsBuilder {
        type Out = CheckoutCustomerBalanceBankTransferPaymentMethodOptions;
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
            Some(Self::Out {
                eu_bank_transfer: self.eu_bank_transfer?,
                requested_address_types: self.requested_address_types.take()?,
                type_: self.type_?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
        type Builder = CheckoutCustomerBalanceBankTransferPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                CheckoutCustomerBalanceBankTransferPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "eu_bank_transfer" => b.eu_bank_transfer = Some(FromValueOpt::from_value(v)?),
                    "requested_address_types" => {
                        b.requested_address_types = Some(FromValueOpt::from_value(v)?)
                    }
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes {
    Aba,
    Iban,
    Sepa,
    SortCode,
    Spei,
    Swift,
    Zengin,
}
impl CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        use CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes::*;
        match self {
            Aba => "aba",
            Iban => "iban",
            Sepa => "sepa",
            SortCode => "sort_code",
            Spei => "spei",
            Swift => "swift",
            Zengin => "zengin",
        }
    }
}

impl std::str::FromStr
    for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes::*;
        match s {
            "aba" => Ok(Aba),
            "iban" => Ok(Iban),
            "sepa" => Ok(Sepa),
            "sort_code" => Ok(SortCode),
            "spei" => Ok(Spei),
            "swift" => Ok(Swift),
            "zengin" => Ok(Zengin),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes::from_str(
                s,
            )
            .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes"))
    }
}
/// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}
impl CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    pub fn as_str(self) -> &'static str {
        use CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            GbBankTransfer => "gb_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
            MxBankTransfer => "mx_bank_transfer",
            UsBankTransfer => "us_bank_transfer",
        }
    }
}

impl std::str::FromStr for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "gb_bank_transfer" => Ok(GbBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            "mx_bank_transfer" => Ok(MxBankTransfer),
            "us_bank_transfer" => Ok(UsBankTransfer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType",
            )
        })
    }
}
