#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodFpx {
    /// Account holder type, if provided. Can be one of `individual` or `company`.
    pub account_holder_type: Option<PaymentMethodFpxAccountHolderType>,
    /// The customer's bank, if provided.
    /// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, `pb_enterprise`, or `bank_of_china`.
    pub bank: PaymentMethodFpxBank,
}
#[doc(hidden)]
pub struct PaymentMethodFpxBuilder {
    account_holder_type: Option<Option<PaymentMethodFpxAccountHolderType>>,
    bank: Option<PaymentMethodFpxBank>,
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

    impl Deserialize for PaymentMethodFpx {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodFpx>,
        builder: PaymentMethodFpxBuilder,
    }

    impl Visitor for Place<PaymentMethodFpx> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodFpxBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodFpxBuilder {
        type Out = PaymentMethodFpx;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_type" => Deserialize::begin(&mut self.account_holder_type),
                "bank" => Deserialize::begin(&mut self.bank),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { account_holder_type: Deserialize::default(), bank: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(account_holder_type), Some(bank)) =
                (self.account_holder_type, self.bank.take())
            else {
                return None;
            };
            Some(Self::Out { account_holder_type, bank })
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

    impl ObjectDeser for PaymentMethodFpx {
        type Builder = PaymentMethodFpxBuilder;
    }

    impl FromValueOpt for PaymentMethodFpx {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodFpxBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder_type" => b.account_holder_type = FromValueOpt::from_value(v),
                    "bank" => b.bank = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Account holder type, if provided. Can be one of `individual` or `company`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodFpxAccountHolderType {
    Company,
    Individual,
}
impl PaymentMethodFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodFpxAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for PaymentMethodFpxAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodFpxAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodFpxAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodFpxAccountHolderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodFpxAccountHolderType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentMethodFpxAccountHolderType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodFpxAccountHolderType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodFpxAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentMethodFpxAccountHolderType")
        })
    }
}
/// The customer's bank, if provided.
/// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, `pb_enterprise`, or `bank_of_china`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodFpxBank {
    pub fn as_str(&self) -> &str {
        use PaymentMethodFpxBank::*;
        match self {
            AffinBank => "affin_bank",
            Agrobank => "agrobank",
            AllianceBank => "alliance_bank",
            Ambank => "ambank",
            BankIslam => "bank_islam",
            BankMuamalat => "bank_muamalat",
            BankOfChina => "bank_of_china",
            BankRakyat => "bank_rakyat",
            Bsn => "bsn",
            Cimb => "cimb",
            DeutscheBank => "deutsche_bank",
            HongLeongBank => "hong_leong_bank",
            Hsbc => "hsbc",
            Kfh => "kfh",
            Maybank2e => "maybank2e",
            Maybank2u => "maybank2u",
            Ocbc => "ocbc",
            PbEnterprise => "pb_enterprise",
            PublicBank => "public_bank",
            Rhb => "rhb",
            StandardChartered => "standard_chartered",
            Uob => "uob",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodFpxBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodFpxBank::*;
        match s {
            "affin_bank" => Ok(AffinBank),
            "agrobank" => Ok(Agrobank),
            "alliance_bank" => Ok(AllianceBank),
            "ambank" => Ok(Ambank),
            "bank_islam" => Ok(BankIslam),
            "bank_muamalat" => Ok(BankMuamalat),
            "bank_of_china" => Ok(BankOfChina),
            "bank_rakyat" => Ok(BankRakyat),
            "bsn" => Ok(Bsn),
            "cimb" => Ok(Cimb),
            "deutsche_bank" => Ok(DeutscheBank),
            "hong_leong_bank" => Ok(HongLeongBank),
            "hsbc" => Ok(Hsbc),
            "kfh" => Ok(Kfh),
            "maybank2e" => Ok(Maybank2e),
            "maybank2u" => Ok(Maybank2u),
            "ocbc" => Ok(Ocbc),
            "pb_enterprise" => Ok(PbEnterprise),
            "public_bank" => Ok(PublicBank),
            "rhb" => Ok(Rhb),
            "standard_chartered" => Ok(StandardChartered),
            "uob" => Ok(Uob),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for PaymentMethodFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodFpxBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodFpxBank {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodFpxBank> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodFpxBank::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodFpxBank);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodFpxBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
