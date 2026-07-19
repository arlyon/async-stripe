#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore {
    /// The name of the convenience store chain where the payment was completed.
    pub chain: Option<
        PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain,
    >,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreBuilder {
    chain: Option<
        Option<
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain,
        >,
    >,
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

    impl Deserialize
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore,
        >,
        builder:
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreBuilder,
    }

    impl Visitor
        for Place<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreBuilder { chain: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "chain" => Deserialize::begin(&mut self.builder.chain),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(chain),) = (self.builder.chain.take(),) else {
                return Ok(());
            };
            *self.out = Some(
                PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore {
                    chain,
                },
            );
            Ok(())
        }
    }
};
/// The name of the convenience store chain where the payment was completed.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain {
    Familymart,
    Lawson,
    Ministop,
    Seicomart,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain {
    pub fn as_str(&self) -> &str {
        use PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain::*;
        match self {
            Familymart => "familymart",
            Lawson => "lawson",
            Ministop => "ministop",
            Seicomart => "seicomart",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain::*;
        match s {
            "familymart" => Ok(Familymart),
            "lawson" => Ok(Lawson),
            "ministop" => Ok(Ministop),
            "seicomart" => Ok(Seicomart),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<
        PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain,
    >
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
