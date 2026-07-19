#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputePaymentMethodDetailsCard {
    /// Card brand.
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `jcb`, `link`, `mastercard`, `unionpay`, `visa` or `unknown`.
    pub brand: String,
    /// The type of dispute opened. Different case types may have varying fees and financial impact.
    pub case_type: DisputePaymentMethodDetailsCardCaseType,
    /// The card network's specific dispute reason code, which maps to one of Stripe's primary dispute categories to simplify response guidance.
    /// The [Network code map](https://stripe.com/docs/disputes/categories#network-code-map) lists all available dispute reason codes by network.
    pub network_reason_code: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputePaymentMethodDetailsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputePaymentMethodDetailsCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DisputePaymentMethodDetailsCardBuilder {
    brand: Option<String>,
    case_type: Option<DisputePaymentMethodDetailsCardCaseType>,
    network_reason_code: Option<Option<String>>,
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

    impl Deserialize for DisputePaymentMethodDetailsCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputePaymentMethodDetailsCard>,
        builder: DisputePaymentMethodDetailsCardBuilder,
    }

    impl Visitor for Place<DisputePaymentMethodDetailsCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputePaymentMethodDetailsCardBuilder {
                    brand: Deserialize::default(),
                    case_type: Deserialize::default(),
                    network_reason_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brand" => Deserialize::begin(&mut self.builder.brand),
                "case_type" => Deserialize::begin(&mut self.builder.case_type),
                "network_reason_code" => Deserialize::begin(&mut self.builder.network_reason_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(brand), Some(case_type), Some(network_reason_code)) = (
                self.builder.brand.take(),
                self.builder.case_type.take(),
                self.builder.network_reason_code.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(DisputePaymentMethodDetailsCard { brand, case_type, network_reason_code });
            Ok(())
        }
    }
};
/// The type of dispute opened. Different case types may have varying fees and financial impact.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DisputePaymentMethodDetailsCardCaseType {
    Block,
    Chargeback,
    Compliance,
    Inquiry,
    Resolution,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DisputePaymentMethodDetailsCardCaseType {
    pub fn as_str(&self) -> &str {
        use DisputePaymentMethodDetailsCardCaseType::*;
        match self {
            Block => "block",
            Chargeback => "chargeback",
            Compliance => "compliance",
            Inquiry => "inquiry",
            Resolution => "resolution",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DisputePaymentMethodDetailsCardCaseType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputePaymentMethodDetailsCardCaseType::*;
        match s {
            "block" => Ok(Block),
            "chargeback" => Ok(Chargeback),
            "compliance" => Ok(Compliance),
            "inquiry" => Ok(Inquiry),
            "resolution" => Ok(Resolution),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "DisputePaymentMethodDetailsCardCaseType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DisputePaymentMethodDetailsCardCaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for DisputePaymentMethodDetailsCardCaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputePaymentMethodDetailsCardCaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(DisputePaymentMethodDetailsCardCaseType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DisputePaymentMethodDetailsCardCaseType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for DisputePaymentMethodDetailsCardCaseType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<DisputePaymentMethodDetailsCardCaseType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DisputePaymentMethodDetailsCardCaseType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DisputePaymentMethodDetailsCardCaseType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
