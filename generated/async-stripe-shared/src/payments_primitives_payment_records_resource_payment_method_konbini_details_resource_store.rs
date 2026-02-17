#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore {
    /// The name of the convenience store chain where the payment was completed.
    pub chain: Option<
        PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain,
    >,
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
            builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreBuilder
    {
        type Out = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "chain" => Deserialize::begin(&mut self.chain),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { chain: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(chain),) = (self.chain.take(),) else {
                return None;
            };
            Some(Self::Out { chain })
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

    impl ObjectDeser
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore
    {
        type Builder =
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreBuilder;
    }

    impl FromValueOpt
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "chain" => b.chain = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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

impl std::fmt::Debug
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStoreChain
);
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
