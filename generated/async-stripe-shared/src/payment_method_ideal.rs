#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodIdeal {
    /// The customer's bank, if provided.
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `n26`, `nn`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
    pub bank: Option<PaymentMethodIdealBank>,
    /// The Bank Identifier Code of the customer's bank, if the bank was provided.
    pub bic: Option<PaymentMethodIdealBic>,
}
#[doc(hidden)]
pub struct PaymentMethodIdealBuilder {
    bank: Option<Option<PaymentMethodIdealBank>>,
    bic: Option<Option<PaymentMethodIdealBic>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodIdeal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodIdeal>,
        builder: PaymentMethodIdealBuilder,
    }

    impl Visitor for Place<PaymentMethodIdeal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodIdealBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodIdealBuilder {
        type Out = PaymentMethodIdeal;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank" => Deserialize::begin(&mut self.bank),
                "bic" => Deserialize::begin(&mut self.bic),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { bank: Deserialize::default(), bic: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bank), Some(bic)) = (self.bank, self.bic) else {
                return None;
            };
            Some(Self::Out { bank, bic })
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

    impl ObjectDeser for PaymentMethodIdeal {
        type Builder = PaymentMethodIdealBuilder;
    }

    impl FromValueOpt for PaymentMethodIdeal {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodIdealBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank" => b.bank = FromValueOpt::from_value(v),
                    "bic" => b.bic = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The customer's bank, if provided.
/// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `n26`, `nn`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl PaymentMethodIdealBank {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodIdealBank::*;
        match self {
            AbnAmro => "abn_amro",
            AsnBank => "asn_bank",
            Bunq => "bunq",
            Handelsbanken => "handelsbanken",
            Ing => "ing",
            Knab => "knab",
            Moneyou => "moneyou",
            N26 => "n26",
            Nn => "nn",
            Rabobank => "rabobank",
            Regiobank => "regiobank",
            Revolut => "revolut",
            SnsBank => "sns_bank",
            TriodosBank => "triodos_bank",
            VanLanschot => "van_lanschot",
            Yoursafe => "yoursafe",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for PaymentMethodIdealBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodIdealBank::*;
        match s {
            "abn_amro" => Ok(AbnAmro),
            "asn_bank" => Ok(AsnBank),
            "bunq" => Ok(Bunq),
            "handelsbanken" => Ok(Handelsbanken),
            "ing" => Ok(Ing),
            "knab" => Ok(Knab),
            "moneyou" => Ok(Moneyou),
            "n26" => Ok(N26),
            "nn" => Ok(Nn),
            "rabobank" => Ok(Rabobank),
            "regiobank" => Ok(Regiobank),
            "revolut" => Ok(Revolut),
            "sns_bank" => Ok(SnsBank),
            "triodos_bank" => Ok(TriodosBank),
            "van_lanschot" => Ok(VanLanschot),
            "yoursafe" => Ok(Yoursafe),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for PaymentMethodIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodIdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodIdealBank {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodIdealBank> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodIdealBank::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodIdealBank);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodIdealBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// The Bank Identifier Code of the customer's bank, if the bank was provided.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodIdealBic {
    Abnanl2a,
    Asnbnl21,
    Bitsnl2a,
    Bunqnl2a,
    Fvlbnl22,
    Handnl2a,
    Ingbnl2a,
    Knabnl2h,
    Moyonl21,
    Nnbanl2g,
    Ntsbdeb1,
    Rabonl2u,
    Rbrbnl21,
    Revoie23,
    Revolt21,
    Snsbnl2a,
    Trionl2u,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl PaymentMethodIdealBic {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodIdealBic::*;
        match self {
            Abnanl2a => "ABNANL2A",
            Asnbnl21 => "ASNBNL21",
            Bitsnl2a => "BITSNL2A",
            Bunqnl2a => "BUNQNL2A",
            Fvlbnl22 => "FVLBNL22",
            Handnl2a => "HANDNL2A",
            Ingbnl2a => "INGBNL2A",
            Knabnl2h => "KNABNL2H",
            Moyonl21 => "MOYONL21",
            Nnbanl2g => "NNBANL2G",
            Ntsbdeb1 => "NTSBDEB1",
            Rabonl2u => "RABONL2U",
            Rbrbnl21 => "RBRBNL21",
            Revoie23 => "REVOIE23",
            Revolt21 => "REVOLT21",
            Snsbnl2a => "SNSBNL2A",
            Trionl2u => "TRIONL2U",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for PaymentMethodIdealBic {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodIdealBic::*;
        match s {
            "ABNANL2A" => Ok(Abnanl2a),
            "ASNBNL21" => Ok(Asnbnl21),
            "BITSNL2A" => Ok(Bitsnl2a),
            "BUNQNL2A" => Ok(Bunqnl2a),
            "FVLBNL22" => Ok(Fvlbnl22),
            "HANDNL2A" => Ok(Handnl2a),
            "INGBNL2A" => Ok(Ingbnl2a),
            "KNABNL2H" => Ok(Knabnl2h),
            "MOYONL21" => Ok(Moyonl21),
            "NNBANL2G" => Ok(Nnbanl2g),
            "NTSBDEB1" => Ok(Ntsbdeb1),
            "RABONL2U" => Ok(Rabonl2u),
            "RBRBNL21" => Ok(Rbrbnl21),
            "REVOIE23" => Ok(Revoie23),
            "REVOLT21" => Ok(Revolt21),
            "SNSBNL2A" => Ok(Snsbnl2a),
            "TRIONL2U" => Ok(Trionl2u),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for PaymentMethodIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodIdealBic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodIdealBic {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodIdealBic> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodIdealBic::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodIdealBic);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodIdealBic {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
