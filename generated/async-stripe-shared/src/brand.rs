/// The local credit or debit card brand.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Brand {
    Bc,
    Citi,
    Hana,
    Hyundai,
    Jeju,
    Jeonbuk,
    Kakaobank,
    Kbank,
    Kdbbank,
    Kookmin,
    Kwangju,
    Lotte,
    Mg,
    Nh,
    Post,
    Samsung,
    Savingsbank,
    Shinhan,
    Shinhyup,
    Suhyup,
    Tossbank,
    Woori,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl Brand {
    pub fn as_str(&self) -> &str {
        use Brand::*;
        match self {
            Bc => "bc",
            Citi => "citi",
            Hana => "hana",
            Hyundai => "hyundai",
            Jeju => "jeju",
            Jeonbuk => "jeonbuk",
            Kakaobank => "kakaobank",
            Kbank => "kbank",
            Kdbbank => "kdbbank",
            Kookmin => "kookmin",
            Kwangju => "kwangju",
            Lotte => "lotte",
            Mg => "mg",
            Nh => "nh",
            Post => "post",
            Samsung => "samsung",
            Savingsbank => "savingsbank",
            Shinhan => "shinhan",
            Shinhyup => "shinhyup",
            Suhyup => "suhyup",
            Tossbank => "tossbank",
            Woori => "woori",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for Brand {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Brand::*;
        match s {
            "bc" => Ok(Bc),
            "citi" => Ok(Citi),
            "hana" => Ok(Hana),
            "hyundai" => Ok(Hyundai),
            "jeju" => Ok(Jeju),
            "jeonbuk" => Ok(Jeonbuk),
            "kakaobank" => Ok(Kakaobank),
            "kbank" => Ok(Kbank),
            "kdbbank" => Ok(Kdbbank),
            "kookmin" => Ok(Kookmin),
            "kwangju" => Ok(Kwangju),
            "lotte" => Ok(Lotte),
            "mg" => Ok(Mg),
            "nh" => Ok(Nh),
            "post" => Ok(Post),
            "samsung" => Ok(Samsung),
            "savingsbank" => Ok(Savingsbank),
            "shinhan" => Ok(Shinhan),
            "shinhyup" => Ok(Shinhyup),
            "suhyup" => Ok(Suhyup),
            "tossbank" => Ok(Tossbank),
            "woori" => Ok(Woori),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "Brand");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for Brand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for Brand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Brand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(Brand)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for Brand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for Brand {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<Brand> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(Brand::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(Brand);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Brand {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
