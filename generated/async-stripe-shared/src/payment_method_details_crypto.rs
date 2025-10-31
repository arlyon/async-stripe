#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCrypto {
    /// The wallet address of the customer.
    pub buyer_address: Option<String>,
    /// The blockchain network that the transaction was sent on.
    pub network: Option<PaymentMethodDetailsCryptoNetwork>,
    /// The token currency that the transaction was sent with.
    pub token_currency: Option<PaymentMethodDetailsCryptoTokenCurrency>,
    /// The blockchain transaction hash of the crypto payment.
    pub transaction_hash: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsCryptoBuilder {
    buyer_address: Option<Option<String>>,
    network: Option<Option<PaymentMethodDetailsCryptoNetwork>>,
    token_currency: Option<Option<PaymentMethodDetailsCryptoTokenCurrency>>,
    transaction_hash: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsCrypto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCrypto>,
        builder: PaymentMethodDetailsCryptoBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCrypto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsCryptoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCryptoBuilder {
        type Out = PaymentMethodDetailsCrypto;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buyer_address" => Deserialize::begin(&mut self.buyer_address),
                "network" => Deserialize::begin(&mut self.network),
                "token_currency" => Deserialize::begin(&mut self.token_currency),
                "transaction_hash" => Deserialize::begin(&mut self.transaction_hash),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                buyer_address: Deserialize::default(),
                network: Deserialize::default(),
                token_currency: Deserialize::default(),
                transaction_hash: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(buyer_address), Some(network), Some(token_currency), Some(transaction_hash)) = (
                self.buyer_address.take(),
                self.network,
                self.token_currency,
                self.transaction_hash.take(),
            ) else {
                return None;
            };
            Some(Self::Out { buyer_address, network, token_currency, transaction_hash })
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

    impl ObjectDeser for PaymentMethodDetailsCrypto {
        type Builder = PaymentMethodDetailsCryptoBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsCrypto {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsCryptoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "buyer_address" => b.buyer_address = FromValueOpt::from_value(v),
                    "network" => b.network = FromValueOpt::from_value(v),
                    "token_currency" => b.token_currency = FromValueOpt::from_value(v),
                    "transaction_hash" => b.transaction_hash = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The blockchain network that the transaction was sent on.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsCryptoNetwork {
    Base,
    Ethereum,
    Polygon,
    Solana,
}
impl PaymentMethodDetailsCryptoNetwork {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsCryptoNetwork::*;
        match self {
            Base => "base",
            Ethereum => "ethereum",
            Polygon => "polygon",
            Solana => "solana",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCryptoNetwork {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCryptoNetwork::*;
        match s {
            "base" => Ok(Base),
            "ethereum" => Ok(Ethereum),
            "polygon" => Ok(Polygon),
            "solana" => Ok(Solana),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsCryptoNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsCryptoNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsCryptoNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodDetailsCryptoNetwork {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCryptoNetwork> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentMethodDetailsCryptoNetwork::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDetailsCryptoNetwork);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCryptoNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentMethodDetailsCryptoNetwork")
        })
    }
}
/// The token currency that the transaction was sent with.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsCryptoTokenCurrency {
    Usdc,
    Usdg,
    Usdp,
}
impl PaymentMethodDetailsCryptoTokenCurrency {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsCryptoTokenCurrency::*;
        match self {
            Usdc => "usdc",
            Usdg => "usdg",
            Usdp => "usdp",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCryptoTokenCurrency {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCryptoTokenCurrency::*;
        match s {
            "usdc" => Ok(Usdc),
            "usdg" => Ok(Usdg),
            "usdp" => Ok(Usdp),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsCryptoTokenCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsCryptoTokenCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsCryptoTokenCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodDetailsCryptoTokenCurrency {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCryptoTokenCurrency> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodDetailsCryptoTokenCurrency::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDetailsCryptoTokenCurrency);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCryptoTokenCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentMethodDetailsCryptoTokenCurrency")
        })
    }
}
