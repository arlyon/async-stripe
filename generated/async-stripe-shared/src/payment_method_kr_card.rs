#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodKrCard {
    /// The local credit or debit card brand.
    pub brand: Option<PaymentMethodKrCardBrand>,
    /// The last four digits of the card. This may not be present for American Express cards.
    pub last4: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodKrCardBuilder {
    brand: Option<Option<PaymentMethodKrCardBrand>>,
    last4: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodKrCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodKrCard>,
        builder: PaymentMethodKrCardBuilder,
    }

    impl Visitor for Place<PaymentMethodKrCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodKrCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodKrCardBuilder {
        type Out = PaymentMethodKrCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brand" => Deserialize::begin(&mut self.brand),
                "last4" => Deserialize::begin(&mut self.last4),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { brand: Deserialize::default(), last4: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(brand), Some(last4)) = (self.brand.take(), self.last4.take()) else {
                return None;
            };
            Some(Self::Out { brand, last4 })
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

    impl ObjectDeser for PaymentMethodKrCard {
        type Builder = PaymentMethodKrCardBuilder;
    }

    impl FromValueOpt for PaymentMethodKrCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodKrCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "brand" => b.brand = FromValueOpt::from_value(v),
                    "last4" => b.last4 = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The local credit or debit card brand.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodKrCardBrand {
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
impl PaymentMethodKrCardBrand {
    pub fn as_str(&self) -> &str {
        use PaymentMethodKrCardBrand::*;
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

impl std::str::FromStr for PaymentMethodKrCardBrand {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodKrCardBrand::*;
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
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PaymentMethodKrCardBrand");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodKrCardBrand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodKrCardBrand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodKrCardBrand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodKrCardBrand {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodKrCardBrand> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodKrCardBrand::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodKrCardBrand);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodKrCardBrand {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
