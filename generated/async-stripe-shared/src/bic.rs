#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Bic {
    Abnanl2a,
    Adybnl2a,
    Asnbnl21,
    Bitsnl2a,
    Bunqnl2a,
    Buutnl2a,
    Fnomnl22,
    Fvlbnl22,
    Handnl2a,
    Ingbnl2a,
    Knabnl2h,
    Mllenl2a,
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
    Unknown(String),
}
impl Bic {
    pub fn as_str(&self) -> &str {
        use Bic::*;
        match self {
            Abnanl2a => "ABNANL2A",
            Adybnl2a => "ADYBNL2A",
            Asnbnl21 => "ASNBNL21",
            Bitsnl2a => "BITSNL2A",
            Bunqnl2a => "BUNQNL2A",
            Buutnl2a => "BUUTNL2A",
            Fnomnl22 => "FNOMNL22",
            Fvlbnl22 => "FVLBNL22",
            Handnl2a => "HANDNL2A",
            Ingbnl2a => "INGBNL2A",
            Knabnl2h => "KNABNL2H",
            Mllenl2a => "MLLENL2A",
            Moyonl21 => "MOYONL21",
            Nnbanl2g => "NNBANL2G",
            Ntsbdeb1 => "NTSBDEB1",
            Rabonl2u => "RABONL2U",
            Rbrbnl21 => "RBRBNL21",
            Revoie23 => "REVOIE23",
            Revolt21 => "REVOLT21",
            Snsbnl2a => "SNSBNL2A",
            Trionl2u => "TRIONL2U",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for Bic {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Bic::*;
        match s {
            "ABNANL2A" => Ok(Abnanl2a),
            "ADYBNL2A" => Ok(Adybnl2a),
            "ASNBNL21" => Ok(Asnbnl21),
            "BITSNL2A" => Ok(Bitsnl2a),
            "BUNQNL2A" => Ok(Bunqnl2a),
            "BUUTNL2A" => Ok(Buutnl2a),
            "FNOMNL22" => Ok(Fnomnl22),
            "FVLBNL22" => Ok(Fvlbnl22),
            "HANDNL2A" => Ok(Handnl2a),
            "INGBNL2A" => Ok(Ingbnl2a),
            "KNABNL2H" => Ok(Knabnl2h),
            "MLLENL2A" => Ok(Mllenl2a),
            "MOYONL21" => Ok(Moyonl21),
            "NNBANL2G" => Ok(Nnbanl2g),
            "NTSBDEB1" => Ok(Ntsbdeb1),
            "RABONL2U" => Ok(Rabonl2u),
            "RBRBNL21" => Ok(Rbrbnl21),
            "REVOIE23" => Ok(Revoie23),
            "REVOLT21" => Ok(Revolt21),
            "SNSBNL2A" => Ok(Snsbnl2a),
            "TRIONL2U" => Ok(Trionl2u),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "Bic");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for Bic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for Bic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Bic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(Bic)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for Bic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for Bic {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<Bic> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(Bic::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(Bic);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Bic {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
