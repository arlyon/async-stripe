#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCustomerBalanceEuBankAccount {
    /// The desired country code of the bank account information.
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: PaymentMethodOptionsCustomerBalanceEuBankAccountCountry,
}
#[doc(hidden)]
pub struct PaymentMethodOptionsCustomerBalanceEuBankAccountBuilder {
    country: Option<PaymentMethodOptionsCustomerBalanceEuBankAccountCountry>,
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

    impl Deserialize for PaymentMethodOptionsCustomerBalanceEuBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsCustomerBalanceEuBankAccount>,
        builder: PaymentMethodOptionsCustomerBalanceEuBankAccountBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsCustomerBalanceEuBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsCustomerBalanceEuBankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsCustomerBalanceEuBankAccountBuilder {
        type Out = PaymentMethodOptionsCustomerBalanceEuBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "country" => Deserialize::begin(&mut self.country),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { country: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(country),) = (self.country.take(),) else {
                return None;
            };
            Some(Self::Out { country })
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

    impl ObjectDeser for PaymentMethodOptionsCustomerBalanceEuBankAccount {
        type Builder = PaymentMethodOptionsCustomerBalanceEuBankAccountBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsCustomerBalanceEuBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsCustomerBalanceEuBankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "country" => b.country = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The desired country code of the bank account information.
/// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    Be,
    De,
    Es,
    Fr,
    Ie,
    Nl,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    pub fn as_str(&self) -> &str {
        use PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::*;
        match self {
            Be => "BE",
            De => "DE",
            Es => "ES",
            Fr => "FR",
            Ie => "IE",
            Nl => "NL",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::*;
        match s {
            "BE" => Ok(Be),
            "DE" => Ok(De),
            "ES" => Ok(Es),
            "FR" => Ok(Fr),
            "IE" => Ok(Ie),
            "NL" => Ok(Nl),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodOptionsCustomerBalanceEuBankAccountCountry"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentMethodOptionsCustomerBalanceEuBankAccountCountry>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsCustomerBalanceEuBankAccountCountry);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
